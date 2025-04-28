mod authority;

pub use authority::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum OrePromoAccount {
    Authority = 0,
}

/// Fetch PDA of the authority account.
pub fn authority_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[AUTHORITY], &crate::id())
}
