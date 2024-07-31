use crate::config::State;

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("set_administrator_event")]
    fn set_administrator_event(&self, #[indexed] administrator: &ManagedAddress);

    #[event("contract_state_event")]
    fn contract_state_event(&self, #[indexed] state: State);

    #[event("max_apr")]
    fn max_apr_event(&self, #[indexed] max_apr: &BigUint);

    #[event("rewards_token_identifier")]
    fn rewards_token_identifier_event(&self, #[indexed] token_identifier: &TokenIdentifier);

    #[event("per_block_reward_amount")]
    fn rewards_per_block_event(&self, #[indexed] per_block_amount: &BigUint);

    #[event("top_up_rewards_event")]
    fn top_up_rewards_event(&self, #[indexed] amount: &BigUint);

    #[event("withdraw_rewards_event")]
    fn withdraw_rewards_event(&self, #[indexed] amount: &BigUint);

    #[event("rewards_state_event")]
    fn rewards_state_event(&self, #[indexed] state: State);

    #[event("bond_contract_address")]
    fn bond_contract_address_event(&self, #[indexed] bond_contract_address: &ManagedAddress);

    #[event("claim_rewards")]
    fn claim_rewards_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] rewards_amount: &BigUint,
        #[indexed] timestamp: u64,
        #[indexed] block_nonce: u64,
        #[indexed] rewards_reserve: &BigUint,
        #[indexed] accumulated_rewards: &BigUint,
        #[indexed] current_rewards_per_share: &BigUint,
        #[indexed] user_last_rewards_per_share: &BigUint,
        #[indexed] rewards_per_block: &BigUint,
    );

    #[event("address_rewards_per_share_event")]
    fn address_rewards_per_share_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] rewards_per_share: &BigUint,
    );
}
