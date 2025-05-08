use ore_boost_api::state::Boost;
use ore_bury_api::{consts::AUTHORITY, state::Authority};
use steel::*;

/// Initialize creates the config account and opens a stake account in the boost program to receive boost rewards.
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, authority_info, boost_info, boost_config_info, boost_deposits_info, boost_proof_info, boost_rewards_info, nft_mint_info, ore_mint_info, rewards_info, sender_info, stake_info, treasury_info, treasury_tokens_info, ore_program, ore_boost_program, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    authority_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[AUTHORITY], &ore_bury_api::ID)?;
    boost_info
        .as_account::<Boost>(&ore_boost_api::ID)?
        .assert(|s| s.mint == *nft_mint_info.key)?;
    nft_mint_info
        .has_address(&ore_bury_api::consts::NFT_MINT_ADDRESS)?
        .as_mint()?;
    ore_mint_info
        .has_address(&ore_api::consts::MINT_ADDRESS)?
        .as_mint()?;
    rewards_info.is_empty()?.is_writable()?;
    sender_info
        .as_associated_token_account(authority_info.key, nft_mint_info.key)?
        .assert(|s| s.amount() == 1)?;
    stake_info.is_empty()?.is_writable()?;
    ore_program.is_program(&ore_api::ID)?;
    ore_boost_program.is_program(&ore_boost_api::ID)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Initialize config.
    create_program_account::<Authority>(
        authority_info,
        system_program,
        signer_info,
        &ore_bury_api::ID,
        &[AUTHORITY],
    )?;

    // Create a token account to hold onto promoter rewards.
    create_associated_token_account(
        signer_info,
        authority_info,
        rewards_info,
        ore_mint_info,
        system_program,
        token_program,
        associated_token_program,
    )?;

    // Open a stake account in the boost program.
    invoke_signed(
        &ore_boost_api::sdk::open(*authority_info.key, *signer_info.key, *nft_mint_info.key),
        &[
            authority_info.clone(),
            signer_info.clone(),
            boost_info.clone(),
            nft_mint_info.clone(),
            stake_info.clone(),
            system_program.clone(),
        ],
        &ore_bury_api::ID,
        &[AUTHORITY],
    )?;

    // Deposit the NFT, allowing this program to claim boost rewards.
    invoke_signed(
        &ore_boost_api::sdk::deposit(*authority_info.key, *nft_mint_info.key, 1),
        &[
            authority_info.clone(),
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
        &ore_bury_api::ID,
        &[AUTHORITY],
    )?;

    Ok(())
}
