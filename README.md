# EddyThink v2

- Pay only for AI compute
- pay nothing to move money

EddyThink is the open-source AI credit layer for any Cashu mint.  
Remove mint transaction fees forever - charge only for AI usage.

**MIT**

## Features

- Zero mint fees forever
- Pay-per-request AI billing via Routstr
- Coco proofs for credit state
- NIP-59 encrypted DM delivery
- NIP-05 login
- Nostr zaps → instant credits
- LDK + ecash melt hooks
- Flutter + Rust bridge ready
- Works with any CDK v0.14+ mint

## Architecture

- User (Flutter app): NIP-05 login → npub
- Billing: Stripe / Beyon / Zaps
- EddyThink core: Lookup npub
- Cashu mint: Issues Coco proof token
- Nostr: NIP-59 sealed DM → wallet receives token
- Routstr proxy: x-cashu header → Groq/Llama3
- AI response: Back to user

## Quick Start

## Development

# Backend (Rust core)
cargo test --lib

# Frontend (Flutter)
flutter run

```bash
# Run Routstr proxy (per-request billing)
docker run -d -p 8000:8000 \
  -e UPSTREAM_BASE_URL="https://api.groq.com/openai/v1" \
  -e UPSTREAM_API_KEY="your_groq_key" \
  -e CASHU_MINTS="https://your-mint.eddy.cash" \
  ghcr.io/routstr/proxy:latest

# Point Goose MCP at localhost:8000
# Pay with zaps, Lightning, or ecash → instant credits
