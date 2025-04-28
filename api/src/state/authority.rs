use steel::*;

use super::OrePromoAccount;

/// Authority tracks the global program variables.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Authority {}

account!(OrePromoAccount, Authority);
