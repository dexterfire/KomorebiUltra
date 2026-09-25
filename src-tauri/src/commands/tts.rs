//! TTS commands and shared synthesis helpers.

use crate::settings;
use tauri::{AppHandle, Manager, Wry};

#[tauri::command]
pub async fn set_piper_binary(app: AppHandle<Wry>, path: String) -> Result<(), String> {
    settings::set_piper_binary(&app, &path).map_err(|e| e.to_string())?;
    reload_tts(&app).await;
    Ok(())
}

#[tauri::command]
pub async fn set_piper_voice(app: AppHandle<Wry>, path: String) -> Result<(), String> {
    settings::set_piper_voice(&app, &path).map_err(|e| e.to_string())?;
    reload_tts(&app).await;
    Ok(())
}

#[tauri::command]
pub async fn set_tts_enabled(app: AppHandle<Wry>, enabled: bool) -> Result<(), String> {
    settings::set_tts_enabled(&app, enabled).map_err(|e| e.to_string())?;
    reload_tts(&app).await;
    Ok(())
}

#[tauri::command]
pub async fn set_tts_provider(app: AppHandle<Wry>, provider: String) -> Result<(), String> {
    settings::set_tts_provider(&app, &provider).map_err(|e| e.to_string())?;
    reload_tts(&app).await;
    Ok(())
}

#[tauri::command]
pub async fn set_tts_prosody(
    app: AppHandle<Wry>,
    length_scale: Option<f64>,
    noise_scale: Option<f64>,
    noise_w: Option<f64>,
) -> Result<(), String> {
    settings::set_tts_length_scale(&app, length_scale).map_err(|e| e.to_string())?;
    settings::set_tts_noise_scale(&app, noise_scale).map_err(|e| e.to_string())?;
    settings::set_tts_noise_w(&app, noise_w).map_err(|e| e.to_string())?;
    reload_tts(&app).await;
    Ok(())
}

#[tauri::command]
pub fn set_tts_volume(app: AppHandle<Wry>, volume: f64) -> Result<(), String> {
    settings::set_tts_volume(&app, volume).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_sovits_config(
    app: AppHandle<Wry>,
    endpoint: String,
    ref_audio: String,
    prompt_text: String,
    prompt_lang: String,
    text_lang: String,
    speed: f64,
) -> Result<(), String> {
    settings::set_sovits_endpoint(&app, &endpoint).map_err(|e| e.to_string())?;
    settings::set_sovits_ref_audio(&app, &ref_audio).map_err(|e| e.to_string())?;
    settings::set_sovits_prompt_text(&app, &prompt_text).map_err(|e| e.to_string())?;
    settings::set_sovits_prompt_lang(&app, &prompt_lang).map_err(|e| e.to_string())?;
    settings::set_sovits_text_lang(&app, &text_lang).map_err(|e| e.to_string())?;
    settings::set_sovits_speed(&app, speed).map_err(|e| e.to_string())?;
    reload_tts(&app).await;
    Ok(())
}

#[tauri::command]
pub fn set_openrouter_tts_enabled(app: AppHandle<Wry>, enabled: bool) -> Result<(), String> {
    settings::set_openrouter_tts_enabled(&app, enabled).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_openrouter_tts_model(app: AppHandle<Wry>, model: String) -> Result<(), String> {
    settings::set_openrouter_tts_model(&app, &model).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_openrouter_tts_voice(app: AppHandle<Wry>, voice: String) -> Result<(), String> {
    settings::set_openrouter_tts_voice(&app, &voice).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn speak_text(app: AppHandle<Wry>, text: String) -> Result<(), String> {
    if text.trim().is_empty() {
        return Ok(());
    }
    match synthesize_via_provider(&app, &text).await {
        Ok(Some(wav)) => {
            emit_tts_wav(&app, &wav);
            Ok(())
        }
        Ok(None) => Err("no TTS provider configured".into()),
        Err(e) => Err(e),
    }
}

/// LLM-driven, multilingual reaction line played when the user taps the
/// avatar. `zone` is one of `head`, `body`, `hand` (legacy `head`/`body`
/// callers still work). Generation route follows the global Mode setting
/// (Local / Cloud / Auto) and falls back to canned localized strings on
/// timeout or error so the user always hears something.
#[tauri::command]
pub async fn speak_reaction(app: AppHandle<Wry>, zone: String) -> Result<(), String> {
    let text = crate::react::generate(&app, &zone).await;
    if text.is_empty() {
        return Ok(());
    }
    if let Ok(Some(wav)) = synthesize_via_provider(&app, &text).await {
        emit_tts_wav(&app, &wav);
    }
    Ok(())
}

/// Generic event-driven reaction (drag, idle, custom). Same pipeline as
/// [`speak_reaction`] — exposed separately so frontend code reads more
/// naturally at call sites that aren't avatar taps.
#[tauri::command]
pub async fn react_event(app: AppHandle<Wry>, kind: String) -> Result<(), String> {
    let text = crate::react::generate(&app, &kind).await;
    if text.is_empty() {
        return Ok(());
    }
    if let Ok(Some(wav)) = synthesize_via_provider(&app, &text).await {
        emit_tts_wav(&app, &wav);
    }
    Ok(())
}

/// Read raw bytes from a TTS temp file (used by the frontend to construct
/// a Blob/object-URL for `<audio>` playback).
#[tauri::command]
pub async fn read_tts_bytes(path: String) -> Result<tauri::ipc::Response, String> {
    // Only allow reading from our own temp dir.
    let expected_root = std::env::temp_dir().join("komorebi-tts");
    let p = std::path::PathBuf::from(&path);
    if !p.starts_with(&expected_root) {
        return Err("path outside tts temp dir".into());
    }
    let bytes = std::fs::read(&p).map_err(|e| e.to_string())?;
    Ok(tauri::ipc::Response::new(bytes))
}

// -----------------------------------------------------------------------
// Helpers (also called from chat.rs and lib.rs).
// -----------------------------------------------------------------------

/// Route a text snippet through the currently selected TTS provider and
/// return the synthesized WAV bytes. Returns Ok(None) when no provider is
/// configured (TTS disabled or mis-configured) — callers should silently
/// skip playback in that case.
pub async fn synthesize_via_provider(
    app: &AppHandle<Wry>,
    text: &str,
) -> Result<Option<Vec<u8>>, String> {
    // Every TTS entry point (chat replies, avatar reactions, speak_text)
    // funnels through here, so strip symbols once. Cloud voices like Edge
    // read `~`, `*`, `^` aloud even though the chat bubble hides them.
    let clean = crate::chat::speak::sanitize_for_tts(text);
    if clean.is_empty() {
        return Ok(None);
    }
    let text = clean.as_str();
    let provider = settings::get_tts_provider(app);
    match provider.as_str() {
        "openrouter" => {
            if !settings::get_openrouter_tts_enabled(app) {
                return Ok(None);
            }
            let Some(key) = settings::get_openrouter_key(app) else {
                return Ok(None);
            };
            let cfg = komorebi_voice::openrouter::OpenRouterTtsConfig {
                api_key: key,
                model: settings::get_openrouter_tts_model(app),
                voice: settings::get_openrouter_tts_voice(app),
            };
            let tts = komorebi_voice::openrouter::OpenRouterTts::new();
            tts.configure(Some(cfg)).await;
            tts.synthesize(text)
                .await
                .map(Some)
                .map_err(|e| e.to_string())
        }
        "sovits" => {
            let Some(sovits) = app.try_state::<komorebi_voice::sovits::SoVitsTts>() else {
                return Ok(None);
            };
            if !sovits.is_configured().await {
                return Ok(None);
            }
            sovits
                .synthesize(text)
                .await
                .map(Some)
                .map_err(|e| e.to_string())
        }
        _ => {
            let Some(tts) = app.try_state::<komorebi_voice::tts::PiperTts>() else {
                return Ok(None);
            };
            if !tts.is_configured().await {
                return Ok(None);
            }
            tts.synthesize(text)
                .await
                .map(Some)
                .map_err(|e| e.to_string())
        }
    }
}

/// Emit synthesized WAV audio to the frontend.
/// Writes the WAV to a temp file and emits the file path. The frontend
/// reads the bytes via `read_tts_bytes` → Blob → object URL, which goes
/// through the native media pipeline (no base64 data URL, no asset: proto).
pub fn emit_tts_wav(app: &AppHandle<Wry>, wav: &[u8]) {
    use tauri::Emitter;
    let dir = std::env::temp_dir().join("komorebi-tts");
    if let Err(e) = std::fs::create_dir_all(&dir) {
        tracing::warn!(?e, "failed to create tts temp dir");
        return;
    }
    // Best-effort cleanup of older files (keep dir small).
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let _ = std::fs::remove_file(entry.path());
        }
    }
    let fname = format!(
        "tts-{}.wav",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    let path = dir.join(fname);
    if let Err(e) = std::fs::write(&path, wav) {
        tracing::warn!(?e, "failed to write tts wav");
        return;
    }
    let path_str = path.to_string_lossy().to_string();
    if let Err(e) = app.emit("tts:play", path_str) {
        tracing::warn!(?e, "failed to emit tts:play");
    }
}

/// Resolves the bundled Piper binary shipped as a Tauri resource.
/// Returns `None` if the resource dir does not contain our sidecar — this is
/// expected during `cargo test` or when the dev build skipped `fetch-piper`.
fn bundled_piper(app: &AppHandle<Wry>) -> Option<std::path::PathBuf> {
    let name = if cfg!(windows) { "piper.exe" } else { "piper" };
    let path = app
        .path()
        .resolve(
            format!("binaries/piper/{name}"),
            tauri::path::BaseDirectory::Resource,
        )
        .ok()?;
    path.exists().then_some(path)
}

/// Re-reads persisted TTS settings and applies them to the shared handle.
/// Called on startup and whenever any TTS-related setting changes.
pub async fn reload_tts(app: &AppHandle<Wry>) {
    use komorebi_voice::tts::{PiperConfig, PiperTts};
    let Some(tts) = app.try_state::<PiperTts>() else {
        return;
    };
    let enabled = settings::get_tts_enabled(app);
    let bin_setting = settings::get_piper_binary(app);
    let voice = settings::get_piper_voice(app);

    // Resolution order for the Piper binary:
    //   1. User-provided override (non-empty `piper_binary_path`).
    //   2. Bundled sidecar (`binaries/piper/piper[.exe]`).
    let bin = bin_setting
        .filter(|s| !s.is_empty())
        .map(std::path::PathBuf::from)
        .or_else(|| bundled_piper(app));

    let length = settings::get_tts_length_scale(app).map(|v| v as f32);
    let noise = settings::get_tts_noise_scale(app).map(|v| v as f32);
    let noise_w = settings::get_tts_noise_w(app).map(|v| v as f32);

    let cfg = match (enabled, bin, voice) {
        (true, Some(b), Some(v)) if !v.is_empty() => {
            Some(PiperConfig::from_voice(b, v).with_prosody(length, noise, noise_w))
        }
        _ => None,
    };
    tts.inner().configure(cfg).await;

    // SoVITS provider — independent of the Piper path. Only enabled when
    // TTS is on and an endpoint URL is configured; the provider selector
    // (`tts_provider`) decides which one is actually used at synth time.
    if let Some(sovits) = app.try_state::<komorebi_voice::sovits::SoVitsTts>() {
        let sv_cfg = if enabled {
            settings::get_sovits_config(app)
        } else {
            None
        };
        sovits.inner().configure(sv_cfg).await;
    }
}
