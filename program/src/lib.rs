mod claim;
mod collect;
mod initialize;
mod update_score;

use claim::*;
use initialize::*;
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
        OrePromoInstruction::Claim => process_claim(accounts, data)?,
        OrePromoInstruction::Initialize => process_initialize(accounts, data)?,
        OrePromoInstruction::UpdateScore => process_update_score(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
