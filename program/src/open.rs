use ore_promo_api::prelude::*;
use steel::*;

pub fn process_open(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, creator_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    creator_info.is_empty()?.is_writable()?;
    system_program.is_program(&system_program::ID)?;

    // Initialize config.
    create_program_account::<Config>(
        creator_info,
        system_program,
        signer_info,
        &ore_promo_api::ID,
        &[CREATOR, signer_info.key.as_ref()],
    )?;
    let creator = creator_info.as_account_mut::<Creator>(&ore_promo_api::ID)?;
    creator.authority = *signer_info.key;
    creator.score = 0;
    creator.rewards = 0;
    creator.last_rewards_factor = Numeric::ZERO;

    Ok(())
}
