use crate::{
    config::{self, State, MAX_PERCENT},
    contexts::base::StorageCache,
    errors::{
        ERR_ALREADY_ACTIVE, ERR_ALREADY_INACTIVE, ERR_INVALID_AMOUNT, ERR_INVALID_TOKEN_IDENTIFIER,
        ERR_INVALID_VALUE, ERR_NOT_PRIVILEGED,
    },
    events, only_privileged, rewards, storage,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule:
    config::ConfigModule + events::EventsModule + storage::StorageModule + rewards::RewardsModule
{
    #[endpoint(setContractStateActive)]
    fn set_contract_state_active(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            self.contract_state().get() == State::Inactive,
            ERR_ALREADY_ACTIVE
        );
        self.contract_state().set(State::Active);
        self.contract_state_event(State::Active);
    }

    #[endpoint(setContractStateInactive)]
    fn set_contract_state_inactive(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            self.contract_state().get() == State::Active,
            ERR_ALREADY_INACTIVE
        );
        self.contract_state().set(State::Inactive);
        self.contract_state_event(State::Inactive);
    }

    // not used (useful to enforce a max APR)
    #[endpoint(setMaxApr)]
    fn set_max_apr(&self, max_apr: BigUint) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        require!(max_apr <= MAX_PERCENT, ERR_INVALID_VALUE);
        self.max_apr_event(&max_apr);
        self.max_apr().set(max_apr);

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);
    }

    #[endpoint(setRewardsTokenIdentifier)]
    fn set_rewards_token_identifier(&self, token_identifier: TokenIdentifier) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.rewards_token_identifier_event(&token_identifier);
        self.rewards_token_identifier().set(token_identifier);
    }

    #[endpoint(setPerBlockRewardAmount)]
    fn set_per_block_rewards(&self, per_block_amount: BigUint) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);
        self.rewards_per_block_event(&per_block_amount);
        storage_cache.rewards_per_block = per_block_amount;
    }

    #[payable("*")]
    #[endpoint(topUpRewards)]
    fn top_up_rewards(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        let payment = self.call_value().single_esdt();

        require!(
            payment.token_identifier == self.rewards_token_identifier().get(),
            ERR_INVALID_TOKEN_IDENTIFIER
        );

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);
        self.top_up_rewards_event(&payment.amount);
        storage_cache.rewards_reserve += payment.amount;
    }

    #[endpoint(withdrawRewards)]
    fn withdraw_rewards(&self, amount: BigUint) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);

        require!(storage_cache.rewards_reserve >= amount, ERR_INVALID_AMOUNT);

        self.withdraw_rewards_event(&amount);
        storage_cache.rewards_reserve -= amount;
    }

    #[endpoint(startProduceRewards)]
    fn start_produce_rewards(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.rewards_state_event(State::Active);
        self.rewards_state().set(State::Active);
        self.last_reward_block_nonce()
            .set(self.blockchain().get_block_nonce());
    }

    #[endpoint(endProduceRewards)]
    fn end_produce_rewards(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let mut storage_cache = StorageCache::new(self);
        self.generate_aggregated_rewards(&mut storage_cache);

        self.rewards_state_event(State::Inactive);
        self.rewards_state().set(State::Inactive);
    }

    #[endpoint(setBondContractAddress)]
    fn set_bond_contract_address(&self, bond_contract_address: ManagedAddress) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.bond_contract_address_event(&bond_contract_address);
        self.bond_contract_address().set(bond_contract_address);
    }
}
