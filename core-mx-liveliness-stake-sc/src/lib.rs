#![no_std]

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
    fn claim_rewards(&self, address: OptionalValue<ManagedAddress>) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);

        let caller = self.blockchain().get_caller();

        // Determine the address to be used
        let address_to_use = match address {
            OptionalValue::Some(addr) => {
                require!(
                    caller == self.bond_contract_address().get(),
                    ERR_ENDPOINT_CALLABLE_ONLY_BY_ACCEPTED_CALLERS
                );
                addr
            }
            OptionalValue::None => caller.clone(),
        };

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        let user_last_rewards_per_share = self.address_last_reward_per_share(&address_to_use).get();

        let rewards =
            self.calculate_caller_share_in_rewards(&address_to_use, &mut storage_cache, false);

        self.claim_rewards_event(
            &address_to_use,
            &rewards,
            self.blockchain().get_block_timestamp(),
            self.blockchain().get_block_nonce(),
            &storage_cache.rewards_reserve,
            &storage_cache.accumulated_rewards,
            &storage_cache.rewards_per_share,
            &user_last_rewards_per_share,
            &storage_cache.rewards_per_block,
        );

        if rewards > BigUint::zero() {
            storage_cache.accumulated_rewards -= &rewards;

            self.send().direct_non_zero_esdt_payment(
                &address_to_use,
                &EsdtTokenPayment::new(self.rewards_token_identifier().get(), 0u64, rewards),
            );
        } else {
            sc_panic!("No rewards to claim");
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
        }
    }

    #[endpoint(stakeRewards)]
    fn stake_rewards(&self, token_identifier: TokenIdentifier) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);

        let caller = self.blockchain().get_caller();

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        let user_last_rewards_per_share = self.address_last_reward_per_share(&caller).get();

        let rewards = self.calculate_caller_share_in_rewards(&caller, &mut storage_cache, false);

        self.claim_rewards_event(
            &caller,
            &rewards,
            self.blockchain().get_block_timestamp(),
            self.blockchain().get_block_nonce(),
            &storage_cache.rewards_reserve,
            &storage_cache.accumulated_rewards,
            &storage_cache.rewards_per_share,
            &user_last_rewards_per_share,
            &storage_cache.rewards_per_block,
        );

        if rewards > BigUint::zero() {
            storage_cache.accumulated_rewards -= &rewards;

            self.tx()
                .to(self.bond_contract_address().get())
                .typed(proxy_contracts::life_bonding_sc_proxy::LifeBondingContractProxy)
                .stake_rewards(caller, token_identifier, rewards.clone())
                .esdt(EsdtTokenPayment::new(
                    self.rewards_token_identifier().get(),
                    0u64,
                    rewards,
                ))
                .sync_call();
        }
    }
}
