# Asari-OS Architecture

## High-Level Goals
- Multi-platform from day one
- Elegant composition of services (inspired by Asari melding)
- Honest capability reporting
- Strong, cohesive visual identity
- Agent-directed development with human architectural control

## Platform Layer
All hardware and virtualization differences are hidden behind the `Platform` trait  
(`crates/asari-platform/src/traits.rs`).

Primary development targets:
1. QEMU (fast iteration cycle)
2. Linux-native

Other platforms are peers that must implement the same contract.

## Visual & Identity Layer
- Spectre Netrunner aesthetic (Mass Effect holographic clarity + Cyberpunk neon density)
- Custom boot sequence based on Asari biotic energy and crest geometry
- Assets live in `assets/ui/` and `assets/boot/`

## Agent Workflow
- Human owns architecture, constraints, and acceptance criteria
- Agents implement and update `agents/feature_list.json`
- Every feature must have clear, testable acceptance criteria

## Current Focus
- Solidify Platform trait
- QEMU smoke path
- Keep visual identity consistent as UI work begins
