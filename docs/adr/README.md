# Architectural Decision Records

This directory tracks the high-level design choices behind Komorebi's
**federated personality** roadmap — the multi-stage plan for letting an
opt-in community of users improve default model behavior without ever
sharing raw conversation text.

| #    | Title                                                                 | Status      |
| ---- | --------------------------------------------------------------------- | ----------- |
| 0001 | [Feedback telemetry](0001-feedback-telemetry.md)                      | Implemented (v1.7.0) |
| 0002 | [Local personality LoRA](0002-local-personality-lora.md)              | Planned (v1.8) |
| 0003 | [Federated LoRA upload](0003-federated-lora-upload.md)                | Proposed    |
| 0004 | [Aggregation & release pipeline](0004-aggregation-release-pipeline.md)| Proposed    |
| 0005 | [Transparency log & revocation](0005-transparency-log-revocation.md)  | Proposed    |
| 0006 | [Always-on listening without Self-echo](0006-always-on-listening-self-echo.md) | Accepted |

## Conventions

* One ADR per file, numbered sequentially. Never edit a ratified ADR in
  place — supersede it with a new one and link both ways.
* Each ADR opens with a short **Status** line (`Proposed`, `Accepted`,
  `Implemented`, `Superseded by NNNN`) and a **Date** in ISO format.
* Sections we expect every ADR to carry: *Context*, *Decision*,
  *Consequences*, *Alternatives considered*. Optional: *Security model*,
  *Open questions*.
* Keep ADRs brief — link out to RFCs or external papers rather than
  reproducing them.
