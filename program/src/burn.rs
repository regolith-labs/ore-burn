use ore_api::state::Proof;
use ore_boost_api::state::Stake;
use ore_burn_api::prelude::*;
use steel::*;

pub fn process_burn(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, authority_info, boost_info, boost_authority_info, boost_proof_info, boost_rewards_info, ore_mint_info, proof_info, rewards_info, stake_info, treasury_info, treasury_tokens_info, ore_program, ore_boost_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    authority_info.as_account_mut::<Authority>(&ore_burn_api::ID)?;
    ore_mint_info
        .has_address(&ore_api::consts::MINT_ADDRESS)?
        .as_mint()?;
    proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *boost_info.key)?;
    rewards_info.as_associated_token_account(
        &authority_info.key,
        &ore_burn_api::consts::NFT_MINT_ADDRESS,
    )?;
    stake_info
        .as_account::<Stake>(&ore_boost_api::ID)?
        .assert(|s| s.authority == *authority_info.key)?
        .assert(|s| s.boost == *boost_info.key)?;
    ore_program.is_program(&ore_api::ID)?;
    ore_boost_program.is_program(&ore_boost_api::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Claim rewards from the program's stake account.
    invoke_signed(
        &ore_boost_api::sdk::claim(
            *authority_info.key,
            *rewards_info.key,
            ore_burn_api::consts::NFT_MINT_ADDRESS,
            u64::MAX,
        ),
        &[
            authority_info.clone(),
            rewards_info.clone(),
            boost_info.clone(),
            boost_authority_info.clone(),
            boost_proof_info.clone(),
            boost_rewards_info.clone(),
            stake_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            ore_program.clone(),
            token_program.clone(),
        ],
        &ore_burn_api::ID,
        &[AUTHORITY],
    )?;

    // Get the amount to burn.
    let rewards = rewards_info
        .as_associated_token_account(authority_info.key, &ore_api::consts::MINT_ADDRESS)?;
    let amount = rewards.amount();

    // Burn the claimed amount.
    burn_signed(
        rewards_info,
        ore_mint_info,
        authority_info,
        token_program,
        amount,
        &[AUTHORITY],
    )?;

    Ok(())
}
