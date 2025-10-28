use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Configuration for UniversalSettle facilitator
#[derive(Debug, Clone)]
pub struct UniversalSettleConfig {
    pub universalsettle_program_id: Pubkey,
    pub fee_destination: Pubkey, // Read from Config account
}

impl UniversalSettleConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let program_id = std::env::var("UNIVERSALSETTLE_PROGRAM_ID")
            .expect("UNIVERSALSETTLE_PROGRAM_ID required");
        let universalsettle_program_id = Pubkey::from_str(&program_id)?;

        Ok(Self {
            universalsettle_program_id,
            fee_destination: Pubkey::default(), // Will be set after reading Config
        })
    }
}
