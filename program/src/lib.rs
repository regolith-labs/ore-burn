mod bury;
mod initialize;

use bury::*;
use initialize::*;

use ore_bury_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&ore_bury_api::ID, program_id, data)?;

    match ix {
        OreBurnInstruction::Bury => process_bury(accounts, data)?,
        OreBurnInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
