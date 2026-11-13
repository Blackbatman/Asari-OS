# Asari-OS Agent Instructions

You are working inside the Asari-OS repository.

## Core Philosophy
- Human sets architecture, constraints, acceptance criteria, and visual identity.
- You implement, test, and iterate inside those bounds.
- Prefer elegant composition of services (the “melding” principle) over monolithic complexity.
- Be honest: clearly mark what is proven vs research-grade.

## Development Order
1. QEMU and Linux-native platforms first.
2. Platform trait abstraction must stay clean.
3. Visual identity (Spectre Netrunner + Asari elegance) is non-negotiable for UI work.
4. Every feature must update `agents/feature_list.json`.

## Rules
- Never claim a platform capability that is not actually implemented and tested.
- Prefer small, reviewable commits with clear evidence.
- When blocked, name the exact missing prerequisite and the next concrete action.
- Keep the multi-platform contract intact.

## Key Files
- `crates/asari-platform/src/traits.rs` — the Platform contract
- `agents/feature_list.json` — living status of every feature
- `assets/` — visual identity and boot sequence
