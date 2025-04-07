use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum OrePromoInstruction {
    Claim = 0,
    Collect = 1,
    Open = 2,

    Initialize = 101,
    UpdateScore = 102,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Claim {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Collect {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UpdateScore {
    pub new_score: [u8; 8],
}

instruction!(OrePromoInstruction, Claim);
instruction!(OrePromoInstruction, Collect);
instruction!(OrePromoInstruction, Initialize);
instruction!(OrePromoInstruction, Open);
instruction!(OrePromoInstruction, UpdateScore);
