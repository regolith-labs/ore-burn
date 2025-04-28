mod burn;
mod initialize;

use burn::*;
use initialize::*;

use ore_burn_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&ore_burn_api::ID, program_id, data)?;

    match ix {
        OreBurnInstruction::Burn => process_burn(accounts, data)?,
        OreBurnInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
