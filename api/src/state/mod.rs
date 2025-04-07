mod config;
mod promoter;

pub use config::*;
pub use promoter::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum OrePromoAccount {
    Config = 0,
    Promoter = 1,
}

/// Fetch PDA of the config account.
pub fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG], &crate::id())
}

/// Fetch PDA of the promoter account.
pub fn promoter_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PROMOTER, authority.as_ref()], &crate::id())
}
