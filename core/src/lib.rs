use nostr_sdk::{Client, Keys, PublicKey, RelayUrl};
use cashu_sdk::{Mint, Amount};
use thiserror::Error;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

#[derive(Error, Debug)]
pub enum ThinkError {
    #[error("Nostr error: {0}")]
    Nostr(String),
    #[error("Cashu error: {0}")]
    Cashu(String),
    #[error("JWT error: {0}")]
    Jwt(jsonwebtoken::errors::Error),
    #[error("Insufficient credits")]
    InsufficientCredits,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,        // user npub
    tier: String,
    credits: u64,
    exp: usize,
    iat: usize,
}

pub struct EddyThink {
    secret: String,
    nostr_keys: Keys,
    mint: Mint,
}

impl EddyThink {
    pub async fn new(secret: String, nostr_keys: Keys, mint_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mint = Mint::new(mint_url).await?;
        Ok(Self { secret, nostr_keys, mint })
    }

    // Preferred: Issue credits as Coco proof (Cashu token)
    pub async fn issue_coco(&self, user_npub: &str, credits: u64, tier: &str) -> Result<String, ThinkError> {
        let sats = credits * 8; // 8 msat per credit (Groq cost)
        let proofs = self.mint.mint_proofs(sats.into(), None).await.map_err(|e| ThinkError::Cashu(e.to_string()))?;
        let token = proofs.to_cashu_token();

        // Send via encrypted Nostr DM (NIP-59 sealed)
        let client = Client::new(&self.nostr_keys);
        client.add_relay(RelayUrl::parse("wss://relay.damus.io").unwrap()).await.map_err(|e| ThinkError::Nostr(e.to_string()))?;
        client.connect().await;
        let pk = PublicKey::parse(user_npub).map_err(|e| ThinkError::Nostr(e.to_string()))?;
        client.send_sealed_msg(pk, &token).await.map_err(|e| ThinkError::Nostr(e.to_string()))?;

        Ok(token)
    }

    // Fallback: Issue JWT + sealed DM
    pub async fn issue_jwt(&self, user_npub: &str, tier: &str) -> Result<String, ThinkError> {
        let credits = match tier {
            "personal" => 5000,
            "family" => 20000,
            "business" => 50000,
            "unlimited" => u64::MAX,
            _ => 500,
        };
        let claims = Claims {
            sub: user_npub.to_string(),
            tier: tier.to_string(),
            credits,
            exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let jwt = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(self.secret.as_bytes())).map_err(ThinkError::Jwt)?;

        let client = Client::new(&self.nostr_keys);
        client.add_relay(RelayUrl::parse("wss://relay.damus.io").unwrap()).await.map_err(|e| ThinkError::Nostr(e.to_string()))?;
        client.connect().await;
        let pk = PublicKey::parse(user_npub).map_err(|e| ThinkError::Nostr(e.to_string()))?;
        client.send_sealed_msg(pk, &jwt).await.map_err(|e| ThinkError::Nostr(e.to_string()))?;

        Ok(jwt)
    }

    // Verify token before Goose MCP command
    pub fn verify(&self, token: &str) -> Result<u64, ThinkError> {
        if token.starts_with("cashu") {
            // Validate Cashu token value
            let proofs = cashu_sdk::Proof::from_cashu_token(token).map_err(|e| ThinkError::Cashu(e.to_string()))?;
            let total_sats = proofs.iter().map(|p| p.amount.to_sat()).sum::<u64>();
            if total_sats < 8 {
                Err(ThinkError::InsufficientCredits)
            } else {
                Ok(total_sats * 125) // 8 msat = 1 credit
            }
        } else {
            // JWT fallback
            let data = decode::<Claims>(token, &DecodingKey::from_secret(self.secret.as_bytes()), &Validation::new(Algorithm::HS256))
                .map_err(ThinkError::Jwt)?;
            if data.claims.credits == 0 {
                Err(ThinkError::InsufficientCredits)
            } else {
                Ok(data.claims.credits)
            }
        }
    }

    // On payment received (zap/LDK/ecash melt) → issue credits
    pub async fn on_payment(&self, payer_npub: &str, amount_msat: u64, tier: &str) -> Result<String, ThinkError> {
        let credits = amount_msat / 8;
        self.issue_coco(payer_npub, credits, tier).await
    }
}
