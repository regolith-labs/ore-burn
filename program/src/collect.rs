use ore_promo_api::prelude::*;
use steel::*;

pub fn process_collect(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateScore::try_from_bytes(data)?;
    let new_score = u64::from_le_bytes(args.new_score);

    // Load accounts.
    let [signer_info, boost_info, boost_config_info, boost_proof_info, boost_rewards_info, config_info, proof_info, stake_info, treasury_info, treasury_tokens_info, ore_program, ore_boost_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let config = config_info
        .as_account_mut::<Config>(&ore_promo_info::ID)?
        .assert_mut(|c| c.admin == *signer.key)?;
    let proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *boost_info.key);
    let stake = stake_info
        .as_account::<Stake>(&ore_boost_api::ID)?
        .assert(|s| s.authority = *config_info.key)?
        .assert(|s| s.boost = *boost_info.key)?;
    ore_program.is_program(&ore_api::ID)?;
    ore_boost_program.is_program(&ore_boost_program::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Calculate amount to collect
    let amount = proof.balance + stake.rewards;

    // Invoke CPI
    invoke_signed(
        &ore_boost_api::sdk::claim(
            *config_info.key,
            *rewards_info.key,
            *nft_mint_info.key,
            amount,
        ),
        &[
            config_info.clone(),
            rewards_info.clone(),
            boost_info.clone(),
            boost_config_info.clone(),
            boost_proof_info.clone(),
            boost_rewards_info.clone(),
            stake_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            ore_program.clone(),
            token_program.clone(),
        ],
        &ore_promo_api::ID,
        &[CONFIG],
    )?;

    // Increment rewards factor
    config.rewards_factor += Numeric::from_fraction(amount, config.total_score);

    Ok(())
}
