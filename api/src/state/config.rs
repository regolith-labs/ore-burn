use steel::*;

use super::OrePromoAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    pub admin: Pubkey,

    pub total_score: u64,

    pub rewards_factor: Numeric,
}

account!(OrePromoAccount, Config);
