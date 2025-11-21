# EddyThink – AI Credit Layer for Eddy

Open-source subscription system for AI-powered Cashu commands.

Pay for LLM compute - not for ecash movement.

## Goals

- Remove mint transaction fees
- Charge only for AI usage (credits)
- Deliver credits via Coco proofs (CoCP – Nostr kind-1) → automatic backup & offline receive
- Work with any CDK v0.14+ mint
- No server-side credit state – stateless JWT + Coco proofs

## Features

- Coco proof issuance (preferred path)
- JWT fallback (for non-Coco wallets)
- Payment hooks: Lightning (LDK), ecash melt, Nostr zap (NIP-57)
- NIP-44 + NIP-59 sealed DM for JWT delivery
- Goose MCP verification (one-line integration)
- MIT licensed

## Crates

- `eddythink-core` – Rust library
- `eddythink-flutter` – Flutter bindings (flutter_rust_bridge)

## Quick Start

```rust
use eddythink_core::EddyThink;

let think = EddyThink::new(secret_key, mint_keys.clone());

// Preferred: issue as Coco proof
let coco_proof = think.issue_coco(&user_npub, 20_000, "family")?;
client.publish_event(coco_proof.to_event()).await?;
