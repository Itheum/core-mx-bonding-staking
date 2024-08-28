use crate::{
    config::{self, BLOCKS_IN_YEAR, DIVISION_SAFETY_CONST, MAX_PERCENT},
    contexts::base::StorageCache,
    events, proxy_contracts, storage,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait RewardsModule:
    storage::StorageModule + config::ConfigModule + events::EventsModule
{
    #[endpoint(generateAggregatedRewards)]
    fn generate_rewards(&self) {
        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);
    }
    fn generate_aggregated_rewards(&self, storage_cache: &mut StorageCache<Self>) {
        if self.can_produce_rewards() {
            let last_reward_nonce = self.last_reward_block_nonce().get();
            let extra_rewards_unbounded =
                self.calculate_rewards_since_last_allocation(storage_cache);
            let max_apr = self.max_apr().get();

            let extra_rewards: BigUint;
            let total_staked_amount = self
                .tx()
                .to(self.bond_contract_address().get())
                .typed(proxy_contracts::life_bonding_sc_proxy::LifeBondingContractProxy)
                .total_bond_amount()
                .returns(ReturnsResult)
                .sync_call();
            if max_apr > BigUint::zero() {
                let extra_rewards_apr_bounded_per_block =
                    self.get_amount_apr_bounded(&total_staked_amount); // max APR based on the total staked amount

                let current_block_nonce = self.blockchain().get_block_nonce();

                let block_nonce_diff = current_block_nonce - last_reward_nonce;

                let extra_rewards_apr_bounded =
                    extra_rewards_apr_bounded_per_block * block_nonce_diff;

                extra_rewards = core::cmp::min(extra_rewards_unbounded, extra_rewards_apr_bounded);
            } else {
                extra_rewards = extra_rewards_unbounded;
            }

            if extra_rewards > BigUint::zero() && extra_rewards <= storage_cache.rewards_reserve {
                let increment = &extra_rewards * DIVISION_SAFETY_CONST / &total_staked_amount;

                storage_cache.rewards_per_share += &increment;
                storage_cache.accumulated_rewards += &extra_rewards;
                storage_cache.rewards_reserve -= &extra_rewards;
            }
        }
    }

    // not used (useful to enforce a max APR)
    fn get_amount_apr_bounded(&self, amount: &BigUint) -> BigUint {
        let max_apr = self.max_apr().get();
        amount * &max_apr / MAX_PERCENT / BLOCKS_IN_YEAR
    }

    fn calculate_rewards_since_last_allocation(
        &self,
        storage_cache: &mut StorageCache<Self>,
    ) -> BigUint {
        let current_block_nonce = self.blockchain().get_block_nonce();

        if current_block_nonce <= storage_cache.last_reward_block_nonce {
            return BigUint::zero();
        }

        let block_nonce_diff = current_block_nonce - storage_cache.last_reward_block_nonce;

        storage_cache.last_reward_block_nonce = current_block_nonce;

        &storage_cache.rewards_per_block * block_nonce_diff
    }

    fn calculate_caller_share_in_rewards(
        self,
        caller: &ManagedAddress,
        total_staked_amount: BigUint,
        user_stake_amount: BigUint,
        storage_cache: &mut StorageCache<Self>,
    ) -> BigUint {
        if total_staked_amount > BigUint::zero()
            && storage_cache.accumulated_rewards > BigUint::zero()
        {
            let user_last_rewards_per_share = self.address_last_reward_per_share(caller).get();

            let user_rewards = user_stake_amount
                * (&storage_cache.rewards_per_share - &user_last_rewards_per_share)
                / DIVISION_SAFETY_CONST;

            self.address_last_reward_per_share(caller)
                .set(storage_cache.rewards_per_share.clone());

            self.address_stack_rewards(caller)
                .update(|value| *value += &user_rewards);

            user_rewards
        } else {
            BigUint::zero()
        }
    }
}
