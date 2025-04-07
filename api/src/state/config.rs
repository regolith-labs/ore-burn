use steel::*;

use super::OrePromoAccount;

/// Config tracks the global program variables.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    /// Admin authority with permission to update the oracle scores.
    pub admin: Pubkey,

    /// Net sum of all creator scores.
    pub total_score: u64,

    /// Rewards factor for distributing creator rewards.
    pub rewards_factor: Numeric,
}

account!(OrePromoAccount, Config);
