use ore_boost_api::*;
use steel::*;

use super::OrePromoAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Promoter {
    pub authority: Pubkey,

    pub score: u64,

    pub rewards: u64,

    pub last_rewards_factor: Numeric,
}

impl Promoter {
    pub fn collect_rewards(&self, config: &mut Config, proof: &Proof, stake: &Stake) {
        // Sanity checks that all boost rewards have been collected, and config rewards factor is up to date.
        assert_eq!(proof.balance, 0);
        assert_eq!(stake.rewards, 0);

        // Accumulate weighted rewards into the promoter account
        if config.rewards_factor > self.last_rewards_factor {
            let accumulated_rewards = config.rewards_factor - self.last_rewards_factor;
            assert!(accumulated_rewards > Numeric::ZERO);
            let promoter_rewards = accumulated_rewards * Numeric::from_u64(self.score);
            self.rewards += promoter_rewards;
        }

        // Update rewards factor
        self.last_rewards_factor = config.rewards_factor;
    }
}

account!(OrePromoAccount, Promoter);
