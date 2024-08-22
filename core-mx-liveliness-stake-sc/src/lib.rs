#![no_std]

use config::MAX_PERCENT;
use contexts::base::StorageCache;

use crate::errors::{ERR_CONTRACT_NOT_READY, ERR_ENDPOINT_CALLABLE_ONLY_BY_ACCEPTED_CALLERS};

multiversx_sc::imports!();

pub mod admin;
pub mod config;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod liveliness_stake_proxy;
pub mod proxy_contracts;
pub mod rewards;
pub mod storage;
pub mod views;

#[multiversx_sc::contract]
pub trait CoreMxLivelinessStake:
    admin::AdminModule
    + config::ConfigModule
    + events::EventsModule
    + rewards::RewardsModule
    + storage::StorageModule
    + views::ViewsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {
        self.set_contract_state_inactive();
    }
    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);

        let caller = self.blockchain().get_caller();

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        let user_last_rewards_per_share = self.address_last_reward_per_share(&caller).get();

        let (total_staked_amount, user_stake_amount, liveliness_score) = self
            .tx()
            .to(self.bond_contract_address().get())
            .typed(proxy_contracts::life_bonding_sc_proxy::LifeBondingContractProxy)
            .get_address_bonds_info(caller.clone())
            .returns(ReturnsResult)
            .sync_call();

        self.calculate_caller_share_in_rewards(
            &caller,
            total_staked_amount,
            user_stake_amount,
            &mut storage_cache,
        );

        let mut stack_rewards = self.address_stack_rewards(&caller).get();

        if liveliness_score < 95_00u64 {
            stack_rewards = (liveliness_score * stack_rewards) / MAX_PERCENT
        }

        self.claim_rewards_event(
            &caller,
            &stack_rewards,
            self.blockchain().get_block_timestamp(),
            self.blockchain().get_block_nonce(),
            &storage_cache.rewards_reserve,
            &storage_cache.accumulated_rewards,
            &storage_cache.rewards_per_share,
            &user_last_rewards_per_share,
            &storage_cache.rewards_per_block,
        );

        if stack_rewards > BigUint::zero() {
            storage_cache.accumulated_rewards -= &stack_rewards;
            self.address_stack_rewards(&caller).clear();

            self.send().direct_non_zero_esdt_payment(
                &caller,
                &EsdtTokenPayment::new(self.rewards_token_identifier().get(), 0u64, stack_rewards),
            );
        }
    }

    #[endpoint(setAddressRewardsPerShare)]
    fn set_address_rewards_per_share(&self, address: ManagedAddress) {
        let caller = self.blockchain().get_caller();

        require!(
            caller == self.bond_contract_address().get(),
            ERR_ENDPOINT_CALLABLE_ONLY_BY_ACCEPTED_CALLERS
        );

        if self.address_last_reward_per_share(&address).is_empty() {
            let rewards_per_share = self.rewards_per_share().get();

            self.address_rewards_per_share_event(&address, &rewards_per_share);

            self.address_last_reward_per_share(&address)
                .set(rewards_per_share);
        } else {
            self.stack_rewards(address);
        }
    }

    #[endpoint(stackRewards)]
    fn stack_rewards(&self, address: ManagedAddress) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);

        let caller = self.blockchain().get_caller();

        require!(
            caller == self.bond_contract_address().get(),
            ERR_ENDPOINT_CALLABLE_ONLY_BY_ACCEPTED_CALLERS
        );

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        let user_last_rewards_per_share = self.address_last_reward_per_share(&address).get();

        let (total_staked_amount, user_stake_amount) = self
            .tx()
            .to(self.bond_contract_address().get())
            .typed(proxy_contracts::life_bonding_sc_proxy::LifeBondingContractProxy)
            .get_address_stake_info(address.clone())
            .returns(ReturnsResult)
            .sync_call();

        let current_rewards = self.calculate_caller_share_in_rewards(
            &address,
            total_staked_amount,
            user_stake_amount,
            &mut storage_cache,
        );
        // bypass liveliness score

        self.stack_rewards_event(
            &address,
            &current_rewards,
            self.blockchain().get_block_timestamp(),
            self.blockchain().get_block_nonce(),
            &storage_cache.rewards_reserve,
            &storage_cache.accumulated_rewards,
            &storage_cache.rewards_per_share,
            &user_last_rewards_per_share,
            &storage_cache.rewards_per_block,
        );
    }

    #[endpoint(stakeRewards)]
    fn stake_rewards(&self, token_identifier: TokenIdentifier) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);

        let caller = self.blockchain().get_caller();

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        let user_last_rewards_per_share = self.address_last_reward_per_share(&caller).get();

        let (total_staked_amount, user_stake_amount, liveliness_score) = self
            .tx()
            .to(self.bond_contract_address().get())
            .typed(proxy_contracts::life_bonding_sc_proxy::LifeBondingContractProxy)
            .get_address_bonds_info(&caller)
            .returns(ReturnsResult)
            .sync_call();

        self.calculate_caller_share_in_rewards(
            &caller,
            total_staked_amount,
            user_stake_amount,
            &mut storage_cache,
        );

        let mut stack_rewards = self.address_stack_rewards(&caller).get();

        if liveliness_score < 95_00u64 {
            stack_rewards = (liveliness_score * stack_rewards) / MAX_PERCENT
        }

        self.claim_rewards_event(
            &caller,
            &stack_rewards,
            self.blockchain().get_block_timestamp(),
            self.blockchain().get_block_nonce(),
            &storage_cache.rewards_reserve,
            &storage_cache.accumulated_rewards,
            &storage_cache.rewards_per_share,
            &user_last_rewards_per_share,
            &storage_cache.rewards_per_block,
        );

        if stack_rewards > BigUint::zero() {
            storage_cache.accumulated_rewards -= &stack_rewards;
            self.address_stack_rewards(&caller).clear();

            self.tx()
                .to(self.bond_contract_address().get())
                .typed(proxy_contracts::life_bonding_sc_proxy::LifeBondingContractProxy)
                .stake_rewards(caller, token_identifier, stack_rewards.clone())
                .esdt(EsdtTokenPayment::new(
                    self.rewards_token_identifier().get(),
                    0u64,
                    stack_rewards,
                ))
                .sync_call();
        }
    }
}
