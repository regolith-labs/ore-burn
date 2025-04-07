use ore_promo_api::prelude::*;
use steel::*;

pub fn process_claim(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Claim::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, creator_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let config = config_info.as_account_mut::<Config>(&ore_promot_api::ID)?;
    let creator = creator_info
        .as_account_mut::<Creator>(&ore_promo_api::ID)?
        .assert_mut(|p| p.authority == *signer_info.key)?;

    // Claim rewards.
    creator.collect_rewards(config, proof, stake);

    // TODO Transfer tokens to beneficiary

    let amount = amount.min(promotor.rewards);
    creator.rewards -= amount;

    Ok(())
}
