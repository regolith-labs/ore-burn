use ore_api::state::Proof;
use ore_boost_api::state::Stake;
use ore_promo_api::prelude::*;
use steel::*;

pub fn process_update_score(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateScore::try_from_bytes(data)?;
    let new_score = u64::from_le_bytes(args.new_score);

    // Load accounts.
    let [signer_info, boost_info, config_info, proof_info, creator_info, stake_info] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let config = config_info
        .as_account_mut::<Config>(&ore_promo_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;
    let creator = creator_info.as_account_mut::<Creator>(&ore_promo_api::ID)?;
    let proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *boost_info.key)?
        .assert(|p| p.balance == 0)?;
    let stake = stake_info
        .as_account::<Stake>(&ore_boost_api::ID)?
        .assert(|s| s.authority == *config_info.key)?
        .assert(|s| s.boost == *boost_info.key)?
        .assert(|s| s.rewards == 0)?;

    // Claim rewards.
    creator.collect_rewards(config, proof, stake);

    // Update total score state.
    if new_score > creator.score {
        config.total_score += new_score - creator.score;
    } else {
        config.total_score -= creator.score - new_score;
    }

    // Update creator score
    creator.score = new_score;

    Ok(())
}
