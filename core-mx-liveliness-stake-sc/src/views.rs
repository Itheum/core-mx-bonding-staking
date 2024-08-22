use crate::{
    config::{self, MAX_PERCENT},
    contexts::base::StorageCache,
    events,
    proxy_contracts::{self},
    rewards, storage,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, PartialEq, Debug)]
pub struct ContractDetails<M: ManagedTypeApi> {
    pub rewards_reserve: BigUint<M>,
    pub accumulated_rewards: BigUint<M>,
    pub rewards_token_identifier: TokenIdentifier<M>,
    pub rewards_per_block: BigUint<M>,
    pub rewards_per_share: BigUint<M>,
    pub administrator: ManagedAddress<M>,
    pub bond_contract_address: ManagedAddress<M>,
    pub last_reward_block_nonce: u64,
    pub max_apr: BigUint<M>,
}

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, PartialEq, Debug)]
pub struct UserData<M: ManagedTypeApi> {
    pub total_staked_amount: BigUint<M>,
    pub user_staked_amount: BigUint<M>,
    pub liveliness_score: BigUint<M>,
    pub accumulated_rewards: BigUint<M>,
    pub accumulated_rewards_bypass: BigUint<M>,
    pub vault_nonce: u64,
}

#[multiversx_sc::module]
pub trait ViewsModule:
    rewards::RewardsModule + events::EventsModule + config::ConfigModule + storage::StorageModule
{
    #[view(claimableRewards)]
    fn claimable_rewards(
        &self,
        caller: ManagedAddress,
        opt_bypass_liveliness: Option<bool>,
    ) -> BigUint {
        let bypass_liveliness = opt_bypass_liveliness.unwrap_or(false);

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

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

        let stack_rewards = self.address_stack_rewards(&caller).get();
        if bypass_liveliness {
            stack_rewards
        } else {
            (liveliness_score * stack_rewards) / MAX_PERCENT
        }
    }

    #[view(contractDetails)]
    fn contract_details(self) -> ContractDetails<Self::Api> {
        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        ContractDetails {
            rewards_reserve: storage_cache.rewards_reserve.clone(),
            accumulated_rewards: storage_cache.accumulated_rewards.clone(),
            rewards_token_identifier: storage_cache.rewards_token_identifier.clone(),
            rewards_per_block: storage_cache.rewards_per_block.clone(),
            last_reward_block_nonce: storage_cache.last_reward_block_nonce,
            rewards_per_share: storage_cache.rewards_per_share.clone(),
            max_apr: storage_cache.max_apr.clone(),
            administrator: self.administrator().get(),
            bond_contract_address: self.bond_contract_address().get(),
        }
    }

    #[view(userDataOut)]
    fn user_data_out(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
    ) -> (ContractDetails<Self::Api>, UserData<Self::Api>) {
        let (total_staked_amount, user_staked_amount, liveliness_score) = self
            .tx()
            .to(self.bond_contract_address().get())
            .typed(proxy_contracts::life_bonding_sc_proxy::LifeBondingContractProxy)
            .get_address_bonds_info(address.clone())
            .returns(ReturnsResult)
            .sync_call();

        let address_nonce_vault = self
            .tx()
            .to(self.bond_contract_address().get())
            .typed(proxy_contracts::life_bonding_sc_proxy::LifeBondingContractProxy)
            .address_vault_nonce(address.clone(), token_identifier)
            .returns(ReturnsResult)
            .sync_call();

        let accumulated_rewards = self.claimable_rewards(address.clone(), Some(true));

        let accumulated_rewards_bypass = self.claimable_rewards(address, Option::<bool>::None);

        let contract_config = self.contract_details();

        (
            contract_config,
            UserData {
                total_staked_amount,
                user_staked_amount,
                liveliness_score,
                accumulated_rewards,
                accumulated_rewards_bypass,
                vault_nonce: address_nonce_vault,
            },
        )
    }
}
