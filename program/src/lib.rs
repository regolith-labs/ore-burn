mod claim;
mod collect;
mod initialize;
mod open;
mod update_score;

use claim::*;
use collect::*;
use initialize::*;
use open::*;
use update_score::*;

use ore_promo_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&ore_promo_api::ID, program_id, data)?;

    match ix {
        // User
        OrePromoInstruction::Collect => process_collect(accounts, data)?,
        OrePromoInstruction::Claim => process_claim(accounts, data)?,
        OrePromoInstruction::Open => process_open(accounts, data)?,

        // Admin
        OrePromoInstruction::Initialize => process_initialize(accounts, data)?,
        OrePromoInstruction::UpdateScore => process_update_score(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
