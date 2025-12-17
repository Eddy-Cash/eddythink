# EddyThink v2

**Pay for the brain, not the cash.**

EddyThink is the open-source AI credit layer for any Cashu mint.  
Remove mint transaction fees forever - charge only for AI usage.

**MIT**

## Features

- Zero mint fees forever
- Pay-per-request AI billing via Routstr
- Full unlinkability — no accounts, no logs
- Coco proofs (Nostr kind-30078) for credit state
- NIP-59 encrypted DM delivery
- NIP-05 login
- Nostr zaps → instant credits
- LDK + ecash melt hooks
- Goose MCP one-line integration
- Flutter + Rust bridge ready
- Works with any CDK v0.14+ mint

- ## Architecture

- **User (Flutter app)**: NIP-05 login → npub
- **Billing**: Stripe / Beyon / Zaps
- **EddyThink core**: Lookup npub
- **Cashu mint**: Issues Coco proof token
- **Nostr**: NIP-59 sealed DM → wallet receives token
- **Routstr proxy**: x-cashu header → Groq/Llama3
- **AI response**: Back to user

**No database**  
**No custody**  
**No trace**  

## Quick Start

```bash
# Run Routstr proxy (per-request billing)
docker run -d -p 8000:8000 \
  -e UPSTREAM_BASE_URL="https://api.groq.com/openai/v1" \
  -e UPSTREAM_API_KEY="your_groq_key" \
  -e CASHU_MINTS="https://your-mint.eddy.cash" \
  ghcr.io/routstr/proxy:latest

# Point Goose MCP at localhost:8000
# Pay with zaps, Lightning, or ecash → instant credits
