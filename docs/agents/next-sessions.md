# Next sessions queue

One block = one fresh session. Agent: when a block's task is done, delete that block from this file; when the file has no blocks left, delete the file and its pointer in `AGENTS.md`.

## 1. Commit pending local changes
```
Закоммить незакоммиченные правки прошлой сессии (фикс тильды в TTS, GLOSSARY.md, docs/research/realtime-voice-ru.md, AGENTS.md, docs/agents/*) логичными коммитами и запушь в origin (форк dexterfire/KomorebiUltra). Потом удали блок 1 из docs/agents/next-sessions.md.
```

## 2. Screenshot bug
```
/diagnosing-bugs #1
```

## 3. Tickets: live conversation
```
/to-tickets #2
```

## 4. Tickets: voice stack
```
/to-tickets #3
```

## 5. Tickets: companions
```
/to-tickets #4
```

## 6. Latency instrumentation
```
/implement #5
```
