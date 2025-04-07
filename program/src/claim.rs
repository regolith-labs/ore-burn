use ore_promo_api::prelude::*;
use steel::*;

pub fn process_claim(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Claim::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, beneficiary_info, boost_info, boost_config_info, config_info, creator_info, proof_info, rewards_info, stake_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    beneficiary_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint() == ore_api::consts::MINT_ADDRESS)?;
    boost_info
        .as_account::<Boost>(&ore_boost_api::ID)?
        .assert(|s| s.mint == ore_promo_api::consts::NFT_MINT_ADDRESS)?;
    boost_config_info.as_account::<BoostConfig>(&ore_boost_api::ID)?;
    let config = config_info.as_account_mut::<Config>(&ore_promot_api::ID)?;
    let creator = creator_info
        .as_account_mut::<Creator>(&ore_promo_api::ID)?
        .assert_mut(|p| p.authority == *signer_info.key)?;
    rewards_info
        .is_writable()?
        .as_associated_token_account(config_info.key, &ore_api::consts::MINT_ADDRESS)?;
    let stake = stake_info
        .as_account::<Stake>(&ore_boost_api::ID)?
        .assert(|s| s.authority == *boost_info.key)?
        .assert(|s| s.rewards == 0)?;
    proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == boost_config_info.key)?
        .assert(|p| p.balance == 0)?;

    // Claim rewards.
    creator.collect_rewards(config, proof, stake);

    // Update creator rewards amount
    let amount = amount.min(promotor.rewards);
    creator.rewards -= amount;

    // Transfer tokens to beneficiary.
    transfer_signed(
        config_info,
        rewards_info,
        beneficiary_info,
        token_program,
        amount,
        &[CONFIG],
    )?;

    Ok(())
}
