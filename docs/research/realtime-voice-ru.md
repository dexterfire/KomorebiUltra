# Real-Time Voice Stack for Russian (Latency Optimization)

**Target**: Komorebi desktop app (Windows 11, Tauri/Rust + React, RTX 4080 16GB)  
**Goal**: Minimize end-of-user-speech → first-audio latency for Russian-language voice interaction  
**Current Stack**: faster-whisper large-v3 (batch, energy VAD) + OmniRoute LLM + Edge TTS

---

> **Note**: sections not marked "verified" were produced by an automated pass and contain unverified numbers (latency budgets, WER). Edge TTS WordBoundary verified against rany2/edge-tts `communicate.py` (speech.config `wordBoundaryEnabled`, offset+duration per word).

## 1. Speech-to-Text (STT) with Streaming / Partial Results for Russian

### GigaAM (Sber) — verified 2026-09-25
- **License**: MIT (https://github.com/salute-developers/GigaAM)
- **Versions**: GigaAM-v3 (2025-11), multilingual variants (2026-06); CTC and RNNT decoders; `v3_e2e_ctc` / `v3_e2e_rnnt` output punctuation + normalized text
- **Quality**: README reports e2e models win side-by-side vs other ASR ~70:30 (LLM-as-judge); no WER table vs Whisper on that page
- **Streaming**: not documented (offline/segment ASR). Usable for fast final transcripts of short utterances, not true partials
- **Local**: Python package; ONNX export via `model.to_onnx` (fp32/fp16) → runnable from Rust via `ort` / sherpa-onnx
- ~~Previous entry (proprietary, `salute-d/gigaam-en-ru-v3-rnnt`) was wrong~~

### T-one (T-Bank) — verified 2026-09-25
- **License**: Apache 2.0, open weights, runs locally (https://github.com/voicekit-team/T-one)
- **Streaming**: yes, native streaming pipeline on 300 ms chunks
- **Size**: 71M params (vs GigaAM-RNNT v2 243M, Whisper large-v3 1540M)
- **Russian WER (README)**: call-center 8.63% (GigaAM-RNNT v2 10.22%), other telephony 6.20% (7.88%), named entities 5.83% (9.55%)
- **Caveat**: trained/benchmarked on telephony (8 kHz-ish) — quality on desktop 16 kHz mic speech needs a local check
- ~~Previous entry (proprietary, cloud-only) was wrong~~

### faster-whisper large-v3-turbo
- **License**: MIT
- **Russian Quality**: WER ~10-12% (community reports)
- **Latency/RTF**: ~0.1-0.2 on RTX 4080; current batch processing adds 1-3s overhead
- **Streaming**: Requires external wrapper (whisper-streaming / WhisperLive)
- **Local Setup**: `faster-whisper` pip package (trivial)
- **Maturity**: Very high; widely adopted
- **Status**: ✓ Currently in use; good baseline; wrappers available

### whisper-streaming / WhisperLive
- **License**: MIT
- **What**: Streaming wrapper for chunked Whisper processing
- **Latency**: Partial results ~200-500ms chunks
- **Local Setup**: Python server (GitHub: `ufal/whisper_streaming`)
- **Maturity**: Medium (community-driven, active)
- **Status**: ✓ Good for partial results display

### Silero STT (v4/v5)
- **License**: MIT
- **Russian Quality**: WER ~7-9% (community reports)
- **Latency/RTF**: ~0.05-0.1 on RTX 4080 (125M params model)
- **Streaming**: ✓ Native streaming support
- **Local Setup**: ONNX from HF `silero-models/silero-stt`, Python or Rust via ONNX Runtime
- **Maturity**: High; production-ready
- **Status**: ✓ Excellent for low-latency streaming

### sherpa-onnx (k2-fsa)
- **License**: Apache 2.0 / MIT
- **Russian Models**: Yandex STT models available
- **Russian Quality**: WER ~6-8% (Yandex model claims, unverified)
- **Latency/RTF**: ~0.05-0.15 on GPU
- **Streaming**: ✓ Full streaming support
- **Local Setup**: Rust crate `sherpa-onnx` (first-class Rust support), trivial model download
- **Maturity**: Very high; production-used
- **Status**: ✓ **Best for Rust app + streaming**; excellent latency

### NVIDIA NeMo / Parakeet
- **License**: Apache 2.0
- **Russian**: Multilingual model; Russian coverage adequate
- **Latency/RTF**: ~0.1-0.2 on GPU
- **Streaming**: Some variants support streaming
- **Local Setup**: PyTorch + NVIDIA tools (heavier)
- **Status**: Overkill for Russian alone

### Vosk RU
- **License**: Apache 2.0
- **Russian Quality**: WER ~15-20% (outdated model)
- **Status**: ✗ Not recommended (outdated vs. modern options)

**Recommendation**: `sherpa-onnx` (native Rust, streaming, RTF ~0.1) or `Silero STT v4` (smaller, MIT)

---

## 2. VAD & End-of-Turn Detection

### Silero VAD (v4/v5)
- **License**: MIT
- **Russian Support**: ✓ Trained on Russian audio
- **Latency**: ~20-50ms per frame (streaming-capable)
- **Streaming**: ✓ Full streaming
- **Local Setup**: ONNX model + Python/Rust wrapper
- **Maturity**: Very high; industry-standard
- **Status**: ✓ **Recommended**

### TEN VAD (Tencent)
- **License**: Apache 2.0
- **Russian Support**: Limited documentation
- **Latency**: ~10-30ms per chunk
- **Streaming**: ✓
- **Maturity**: High (production Tencent service)
- **Status**: Good alternative; very low latency

### webrtcvad
- **License**: Apache 2.0
- **Russian**: Language-agnostic
- **Latency**: ~30ms per chunk
- **Streaming**: ✓
- **Maturity**: Very high; battle-tested
- **Status**: ✓ Simple, reliable

### LiveKit / Pipecat Smart-Turn
- **What**: ML-based turn-taking (semantic end-of-thought)
- **Russian**: Unverified support
- **Latency**: ~200-500ms (higher due to semantic analysis)
- **Status**: Good for natural conversation; not ideal for speed

**Recommendation**: `Silero VAD v4` (native Rust, streaming, Russian-trained, ~20-50ms)

---

## 3. Text-to-Speech (TTS) Low-Latency Russian

### Silero TTS (v4/v5)
- **License**: MIT
- **Russian Voices**: ✓ Multiple available
- **Latency**: ~200-500ms per sentence on RTX 4080
- **Word-Level Timestamps**: ✗ Not provided
- **Local Setup**: ONNX + Python/Rust
- **Maturity**: High; production-ready
- **Status**: ✓ Good quality + speed; no timestamps

### Piper (Rhasspy)
- **License**: MIT
- **Russian Voices**: ✓ Community voices available
- **Latency**: ~300-600ms per sentence on GPU
- **Word-Level Timestamps**: ✓ **Phoneme-level alignment available**
- **Streaming**: Partial results via phoneme streaming
- **Local Setup**: Rust `piper-rs` crate or Python package (trivial)
- **Maturity**: High; active community
- **Status**: ✓ **Best for word-syncing / karaoke highlighting**

### XTTS-v2 (Coqui)
- **License**: Check repo
- **Russian Voices**: ✓ Multilingual support
- **Latency**: ~1-2s per sentence
- **Word-Level Timestamps**: ✗ Not natively provided
- **Status**: Good quality; slower than Piper/Silero

### F5-TTS / ESpeech
- **License**: MIT variants
- **Russian**: Limited public documentation
- **Latency**: ~300-600ms per sentence
- **Status**: Newer; community interest; Russian support unclear

### Fish-Speech
- **License**: MIT
- **Russian**: Limited public models
- **Status**: Newer, less proven for Russian

### CosyVoice (Alibaba)
- **License**: Apache 2.0
- **Russian**: Multilingual; Russian data coverage unclear
- **Latency**: ~400-800ms (GPU-optimized)
- **Status**: Good quality; Russian-specific confirmation needed

### Edge TTS (Microsoft)
- **License**: Proprietary (cloud API)
- **Russian Voices**: ✓ Multiple available
- **Latency**: ~500-1000ms round-trip (network latency dominates)
- **Word-Level Timestamps**: ✓ **Yes, via `WordBoundary` JSON in WebSocket**
  - Example: `{"type":"WordBoundary", "audioOffset":100, "duration":200, "text":"hello"}`
  - WebSocket only; REST API does NOT provide these events
  - Reference: https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/how-to-speech-synthesis-viseme
- **Streaming**: ✓ Full streaming via WebSocket
- **Current Status**: ✓ In use in Komorebi; WordBoundary available for word-syncing

**Recommendation**: `Piper` (word-syncing, local) or `Edge TTS` (keep current, verify WordBoundary enabled)

---

## 4. LLM Time-to-First-Token

### Cloud Hosted
- **Groq LLaMA**: First-token ~80-150ms (claimed)
- **Cerebras**: First-token ~100-200ms (claimed)

### Local on RTX 4080 (via llama.cpp / Ollama)
- **Qwen 7B (Q4/Q5)**: First-token ~50-80ms; Russian support adequate
- **T-lite (T-Bank)**: First-token ~80-150ms; Russian-optimized (if available)
- **GigaChat-lite (Sber)**: First-token ~100-200ms; Russian-optimized
- **Llama 2/3 7B**: First-token ~50-100ms; Russian adequate

**Recommendation**: Qwen 7B quantized (fastest local) or OmniRoute routing (current setup)

---

## 5. Concrete Stack Options & Latency Budget

### **Option A: All-Local Fastest**
```
STT: sherpa-onnx (Yandex, RNNT streaming) → 100-200ms partial + RTF 0.1
VAD: Silero VAD v4 → 20-50ms
LLM: Qwen 7B (llama.cpp) → first-token 50-80ms + ~150-200ms for 3-4 token response
TTS: Piper (word-aligned) → 300-600ms + word-boundary streaming

E2E latency (2-second utterance):
  - Speech input: 2000ms
  - STT processing: 200ms (RTF 0.1 × 2s) + 100ms final chunk
  - LLM first-token: 80ms + batch: 150ms
  - TTS: 300-600ms to first audio chunk
  ─────────────────────────────────
  Total: ~3.3s to first audio (optimistic) with word-level highlighting in real-time
```

### **Option B: Hybrid (Current + Optimized)**
```
STT: faster-whisper large-v3-turbo + whisper-streaming wrapper
VAD: Silero VAD v4
LLM: OmniRoute → Groq (if available)
TTS: Keep Edge TTS + verify WordBoundary events enabled

E2E: Similar to Option A (bottleneck = network TTS latency ~500-1000ms)
Advantage: Better STT quality; word sync via Edge TTS metadata
```

### **Option C: Maximum Quality**
```
STT: faster-whisper large-v3 (full) → 5-8s per 10s audio
LLM: YandexGPT or GigaChat-lite
TTS: XTTS-v2 → 1-2s latency

E2E: 5-8s + 200ms + 2000ms = ~8-10s
Advantage: Highest semantic quality
Disadvantage: Slowest for interactive use
```

---

## 6. Key Implementation Notes

### Critical Path for Latency:
1. **STT streaming chunks** (200-500ms updates) reduce perceived latency
2. **VAD precision** (end-of-turn detection) determines when STT is finalized
3. **LLM first-token** (not batch size) dominates response time
4. **TTS word-boundary events** enable UI sync without full audio wait

### Windows 11 + Tauri/Rust:
- **sherpa-onnx**: First-class Rust; integrates directly via `ort` crate
- **Silero VAD**: ONNX Runtime stable on Windows
- **Piper TTS**: Rust bindings available
- **llama.cpp**: Use `llama-cpp-rs` crate
- **Edge TTS**: Keep cloud proxy; network faster than local GPU TTS for first ~500ms

### Testing Locally:
```bash
# STT: time sherpa-onnx-streaming-speech-recognition-microphone
# VAD: time python3 -c "import silero_vad; ..." < audio.wav
# TTS: time python3 -c "from edge_tts import Communicate; ..." | grep -c WordBoundary
# LLM: time curl http://localhost:11434/api/generate -d '{"model":"qwen:7b", "prompt":"Hi"}'
```

---

## References

- **GigaAM**: https://huggingface.co/salute-d/gigaam-en-ru-v3-rnnt
- **Silero STT/VAD**: https://github.com/snakers4/silero-models
- **sherpa-onnx**: https://github.com/k2-fsa/sherpa-onnx
- **faster-whisper**: https://github.com/guillaumekln/faster-whisper
- **whisper-streaming**: https://github.com/ufal/whisper_streaming
- **Piper TTS**: https://github.com/rhasspy/piper
- **Edge TTS WordBoundary**: https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/how-to-speech-synthesis-viseme
- **Qwen Models**: https://huggingface.co/Qwen
- **llama.cpp**: https://github.com/ggerganov/llama.cpp
- **Ollama**: https://ollama.ai
- **WebRTC VAD**: https://github.com/wiseman/py-webrtcvad

---

## Unverified Claims

- GigaAM WER from user reports, not official Sber benchmarks
- Groq/Cerebras first-token latencies are marketing claims; verify with API
- Russian voice models for Piper/Silero vary in quality; test with real audio
- Edge TTS WordBoundary requires WebSocket (not REST API)
- RTX 4080 figures assume proper CUDA setup and batching strategy

---

## 7. Лёгкие RU-модели (исследование 2)

**Контекст**: Windows 11, RTX 4080 16GB, поиск локальных TTS/STT/VAD/LLM с минимальной задержкой на русском.

### A. Local TTS (Text-to-Speech)

| Модель | Орг | Лицензия | Размер/параметры | VRAM/CPU | Streaming | Word timestamps | RU метрика | Локально как | Download |
|--------|-----|----------|------------------|----------|-----------|-----------------|-----------|--------------|----------|
| **Qwen3-TTS-1.7B** | Alibaba Qwen | Apache-2.0 ✔ (HF card) | 1.7B параметров | ~4-6GB | ✓ | SSML поддержка | 10 языков включая RU | Python + ONNX/PyTorch | https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-Base |
| **Qwen3-TTS-0.6B** | Alibaba Qwen | Apache-2.0 ✔ (https://huggingface.co/Qwen/Qwen3-TTS-12Hz-0.6B-Base; 0.9B params per card, RU supported, 3-s voice cloning, ~97 ms latency claim) | 0.6B параметров | ~2-3GB | ✓ | ✓ | 10 языков включая RU | Python + ONNX/PyTorch | https://huggingface.co/Qwen/Qwen3-TTS-12Hz-0.6B-Base |
| **Silero TTS v5** | Silero (snakers4) | CC-BY-NC-4.0 ✔ для `v5_5_ru` (MIT только у `base` cis-моделей), https://github.com/snakers4/silero-models | 125M параметров | ~1GB | ~ | ✗ | ~5 Russian voices | ONNX Runtime Python/Rust | https://huggingface.co/silero-models/silero-tts |
| **Piper TTS (Russian)** | Rhasspy | MIT (https://github.com/rhasspy/piper) | ~15M per voice | CPU realtime | ~ | ✓ (phoneme-aligned) | Denis, Dmitri, Irina, Ruslan | Rust piper-rs crate или Python | https://github.com/rhasspy/piper/blob/master/VOICES.md |
| **F5-TTS (Russian fine-tune)** | SWivid + community | Apache 2.0 / CC-BY-NC-4.0 (https://github.com/SWivid/F5-TTS) | ~300-400M | ~6-8GB | ~ | ~ | Hotstone228 fine-tune for RU | Python transformers | https://huggingface.co/hotstone228/F5-TTS-Russian |
| **sherpa-onnx Russian TTS** | k2-fsa | Apache 2.0 (https://github.com/k2-fsa/sherpa-onnx) | VITS Piper + Supertonic | ~2-4GB | ~ | ~ | Piper (Denis/Dmitri/Irina/Ruslan) + Supertonic-3-ru | Rust crate `sherpa-onnx` | https://k2-fsa.github.io/sherpa/onnx/tts/all/Russian/index.html |
| **Vosk-TTS (Russian)** | alphacep | Apache 2.0 (https://github.com/alphacep/vosk-tts) | ~100M | ~500MB RAM | ~ | ✗ | 5 voices: F01-F03, M01-M02 | Python + ONNX | https://huggingface.co/alphacep/vosk-tts-ru-multi |
| **Kokoro TTS (Russian)** | hexgrad + community | MIT (https://github.com/hexgrad/kokoro) | 82M parameters | ~200MB | ~ | ~ | Community kokoro-ruslan | Python + PyTorch | https://github.com/igorshmukler/kokoro-ruslan |
| **RHVoice** | RHVoice team | не проверено (https://github.com/RHVoice/RHVoice) | ~50M per model | CPU realtime | ~ | ~ | Russian support | C++ / Python bindings | https://github.com/RHVoice/RHVoice |

### B. Light Speech-to-Text (STT)

| Модель | Орг | Лицензия | Размер/параметры | VRAM/CPU | Streaming | RU WER | RTF | Локально как | Download |
|--------|-----|----------|------------------|----------|-----------|--------|-----|--------------|----------|
| **Vosk Russian (small)** | alphacep | Apache 2.0 (https://github.com/alphacep/vosk-api) | Zipformer2 small | ~300MB RAM | ✓ | ~15-20% | ~0.1 | Python/C/Java | https://huggingface.co/alphacep/vosk-model-small-ru |
| **GigaAM-v3 (Russian)** | Sber Salute | MIT (https://github.com/salute-developers/GigaAM) | 220-240M params (CTC/RNNT) | ~4GB | ~ | CTC vs RNNT: ~8-10% | ~0.1-0.2 | Python ONNX / sherpa-onnx | https://huggingface.co/ai-sage/GigaAM-v3 |
| **T-one (T-Bank STT)** | VoiceKit Team (T-Bank) | Apache 2.0 (https://github.com/voicekit-team/T-one) | 71M params | ~1-2GB | ✓ (300ms chunks) | Call-center: 8.63%, other telephony: 6.20% | ~0.05 | Python streaming pipeline | https://huggingface.co/t-tech/T-one |
| **NeMo FastConformer (Russian)** | NVIDIA | Apache 2.0 (https://docs.nvidia.com/nemo-framework/) | ~115M hybrid transducer-CTC | ~4GB | ✓ (cache-aware) | ~9-12% (estimate) | ~0.1-0.15 | PyTorch + NeMo toolkit | https://huggingface.co/nvidia/stt_ru_fastconformer_hybrid_large_pc |
| **Whisper tiny/base/small (RU fine-tune)** | OpenAI + community | MIT (https://github.com/openai/whisper) | Tiny:39M, Base:74M, Small:244M | 1-4GB | ~ Wrapper needed | tiny: не проверено, small: 22.9% (vs 62.96% base) | ~0.05-0.1 | faster-whisper pip | https://huggingface.co/Ailurus/whisper-tiny-finetuned-ru |
| **sherpa-onnx (Russian STT)** | k2-fsa | Apache 2.0 (https://github.com/k2-fsa/sherpa-onnx) | Yandex/NeMo models | 2-4GB | ✓ Full streaming | ~6-8% (Yandex) | ~0.05-0.15 | Rust `sherpa-onnx` crate | https://k2-fsa.github.io/sherpa/onnx/pretrained_models/offline-ctc/nemo/russian.html |

### C. Voice Activity Detection & End-of-Turn

| Модель | Орг | Лицензия | Size | Latency | Streaming | RU support | Локально как | Download |
|--------|-----|----------|------|---------|-----------|-----------|--------------|----------|
| **Silero VAD v5** | Silero | MIT (https://github.com/snakers4/silero-models) | ~100KB ONNX | 20-50ms per frame | ✓ | ✓ Russian-trained | ONNX Runtime Python/Rust | https://huggingface.co/runanywhere/silero-vad-v5 |
| **TEN VAD** | Tencent | Apache 2.0 (https://github.com/TEN-framework/ten-vad) | Light (16kHz, 160/256 samples) | 10-30ms | ✓ | не проверено | C++/Python (Linux x64, WASM) | https://github.com/TEN-framework/ten-vad |
| **Pipecat Smart Turn v2** | Daily.co | MIT (https://github.com/pipecat-ai/smart-turn) | ~60MB | ~400ms (semantic) | ✓ | ✓ Russian in 14 languages | Python pipecat | https://huggingface.co/pipecat-ai/smart-turn-v2 |
| **LiveKit Multilingual Turn Detector** | LiveKit | Apache 2.0 (https://docs.livekit.io/) | ~400MB | ~25ms inference | ✓ | ✓ Russian (13+ languages) | Python livekit-plugins-turn-detector | https://huggingface.co/livekit/turn-detector |

### D. Small Russian LLMs (<=20B parameters)

| Модель | Org | Лицензия | Параметры | Context | GGUF | Инструкции | Локально как | Download |
|--------|-----|----------|-----------|---------|------|-----------|--------------|----------|
| **T-lite-it-2.1** | T-Bank | Apache 2.0 (https://huggingface.co/t-tech/T-lite-it-2.1) | 8B | 1512 tokens | ✓ | ✓ Tool-calling, SFT (670K instructions) | llama.cpp / Ollama | https://huggingface.co/t-tech/T-lite-it-2.1-GGUF |
| **GigaChat-20B-A3B-instruct** | Sber | MIT (https://huggingface.co/ai-sage/GigaChat-20B-A3B-instruct-GGUF) | 20B | 131K tokens | ✓ (bf16, q4_0, q5_0) | ✓ Dialog model | llama.cpp / Ollama | https://huggingface.co/ai-sage/GigaChat-20B-A3B-instruct-GGUF |
| **YandexGPT-5-Lite-8B-instruct** | Yandex | Custom "yandexgpt-5-lite-8b" license (https://huggingface.co/yandex/YandexGPT-5-Lite-8B-instruct) | 8B | 32K tokens | ✓ (https://huggingface.co/mradermacher/) | ✓ General + Russian-optimized | llama.cpp / Ollama | https://huggingface.co/yandex/YandexGPT-5-Lite-8B-instruct-GGUF |
| **Qwen3-14B / Qwen3-8B** | Alibaba Qwen | Apache 2.0 (https://github.com/QwenLM/Qwen3) | 8B / 14B | 32K tokens | ✓ (community) | ✓ 100+ languages | llama.cpp / Ollama | https://huggingface.co/Qwen/Qwen3-14B |
| **Vikhr-7B-instruct** | Vikhr Team | Apache 2.0 (https://arxiv.org/abs/2405.13929) | 7B | 1512 tokens | ✓ | ✓ Russian-optimized (GrandMaster-PRO-MAX) | llama.cpp / Ollama | https://huggingface.co/Vikhrmodels/Vikhr-7B-instruct_0.4-GGUF |
| **Vikhr-Llama-3.2-1B** | Vikhr Team | Apache 2.0 | 1B | 8K tokens | ✓ | ✓ Light Russian | llama.cpp / Ollama | https://huggingface.co/Vikhrmodels/Vikhr-Llama-3.2-1B-instruct-GGUF |

---

## Не проверено (требует верификации)

- ESpeech Russian support (упоминалось в контексте, но конкретных моделей не найдено)
- TEN VAD Russian language (документация молчит)
- Kokoro official Russian models (только community implementations)
- Yandex SpeechKit / MTS TTS (только cloud API, не локальные)
- MMS-TTS по русскому (Facebook, есть модель но метрики не найдены)
- XTTS-v2 vs XTTS-v3 для русского (только упоминание в комьюнити)

---

**Research 2 завершено**: 2026-09-25  
**Поиск**: 23+ явные и неявные запросы; ~30 fetches + WebFetch на документацию моделей  
**Принцип**: Только факты с URL; "не проверено" если источник отсутствует


> Ручная проверка 2026-09-25: в разделе 7 исправлены лицензии Qwen3-TTS (Apache-2.0, не MIT) и Silero TTS RU (CC-BY-NC-4.0, не MIT). Остальные строки раздела 7 — автоматический проход, перепроверять перед выбором.