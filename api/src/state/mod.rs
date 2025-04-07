mod config;
mod creator;

pub use config::*;
pub use creator::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum OrePromoAccount {
    Config = 0,
    Creator = 1,
}

/// Fetch PDA of the config account.
pub fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG], &crate::id())
}

/// Fetch PDA of the creator account.
pub fn creator_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CREATOR, authority.as_ref()], &crate::id())
}
