use ore_boost_api::*;
use steel::*;

use super::OrePromoAccount;

/// Creator tracks the current score and claimable rewards of a given creator account.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Creator {
    /// Authority with permission to claim from this creator account.
    pub authority: Pubkey,

    /// The current score of the creator (used for divvying up creator boost rewards).
    pub score: u64,

    /// The total amount of rewards claimable by this creator.
    pub rewards: u64,

    /// The rewards factor last time rewards was updated.
    pub last_rewards_factor: Numeric,
}

impl Creator {
    pub fn collect_rewards(&self, config: &mut Config, proof: &Proof, stake: &Stake) {
        // Sanity checks that all boost rewards have been collected, and config rewards factor is up to
        assert_eq!(proof.balance, 0);
        assert_eq!(stake.rewards, 0);

        // Accumulate weighted rewards into the promoter account
        if config.rewards_factor > self.last_rewards_factor {
            let accumulated_rewards = config.rewards_factor - self.last_rewards_factor;
            assert!(accumulated_rewards > Numeric::ZERO);
            let personal_rewards = accumulated_rewards * Numeric::from_u64(self.score);
            self.rewards += personal_rewards;
        }

        // Update rewards factor
        self.last_rewards_factor = config.rewards_factor;
    }
}

account!(OrePromoAccount, Promoter);
