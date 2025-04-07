use ore_promo_api::prelude::*;
use steel::*;

/// Initialize creates the config account and opens a stake account in the boost program to receive boost rewards.
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, boost_config_info, boost_deposits_info, boost_proof_info, boost_rewards_info, config_info, nft_mint_info, ore_mint_info, sender_info, stake_info, treasury_info, treasury_tokens_info, ore_program, ore_boost_program, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info
        .as_account::<Boost>(&ore_boost_api::ID)?
        .assert(|b| b.mint == ore_mint_info.key)?;
    config_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[CONFIG], &ore_promo_api::ID)?;
    nft_mint_info
        .has_address(ore_promo_api::consts::PROMO_MINT_ADDRESS)?
        .as_mint()?;
    ore_mint_info
        .has_address(ore_api::consts::MINT_ADDRESS)?
        .as_mint()?;
    sender_info
        .as_associated_token_account(config_info.key, nft_mint_info.key)?
        .assert(|s| s.balance == 1)?;
    stake_info.is_empty()?.is_writeable()?;
    ore_program.is_program(&ore_api::ID)?;
    ore_boost_program.is_program(&ore_boost_api::ID)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Initialize config.
    create_program_account::<Config>(
        config_info,
        system_program,
        signer_info,
        &ore_promo_api::ID,
        &[COUNTER],
    )?;
    let config = config_info.as_account_mut::<Config>(&ore_promo_api::ID)?;
    config.admin = *singer_info.key;
    config.rewards_factor = Numeric::ZERO;
    config.total_score = 0;

    // Create a token account to hold onto promoter rewards.
    create_associated_token_account(
        signer_info,
        config_info,
        rewards_info,
        ore_mint_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    // Open a stake account in the boost program.
    invoke_signed(
        &ore_boost_api::sdk::open(*config_info.key, *signer_info.key, *nft_mint_info.key),
        &[
            config_info.clone(),
            signer_info.clone(),
            boost_info.clone(),
            nft_mint_info.clone(),
            stake_info.clone(),
            system_program.clone(),
        ],
        &ore_promo_api::ID,
        &[CONFIG],
    )?;

    // Deposit the NFT, allowing this program to claim boost rewards.
    invoke_signed(
        &ore_boost_api::sdk::deposit(*config_info.key, *nft_mint_info.key, 1),
        &[
            config_info.clone(),
            boost_info.clone(),
            boost_config_info.clone(),
            boost_deposits_info.clone(),
            nft_mint_info.clone(),
            boost_proof_info.clone(),
            boost_rewards_info.clone(),
            sender_info.clone(),
            stake_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            ore_program.clone(),
            token_program.clone(),
        ],
        &ore_promo_api::ID,
        &[CONFIG],
    )?;

    Ok(())
}
