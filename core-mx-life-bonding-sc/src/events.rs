use multiversx_sc::types::MultiValueEncoded;

use crate::{
    config::State,
    storage::{Bond, Compensation},
};

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("minimum_penalty_event")]
    fn minimum_penalty_event(&self, #[indexed] value: u64);

    #[event("maximum_penalty_event")]
    fn maximum_penalty_event(&self, #[indexed] value: u64);

    #[event("withdraw_penalty_event")]
    fn withdraw_penalty_event(&self, #[indexed] value: u64);

    #[event("contract_state_event")]
    fn contract_state_event(&self, #[indexed] state: State);

    #[event("bond_event")]
    fn bond_event(&self, #[indexed] bond: &Bond<Self::Api>);

    #[event("compensation_event")]
    fn compensation_event(&self, #[indexed] compensation: &Compensation<Self::Api>);

    #[event("withdraw_event")]
    fn withdraw_event(
        &self,
        #[indexed] bond_id: &u64,
        #[indexed] caller: &ManagedAddress,
        #[indexed] withdraw_amount: &BigUint,
        #[indexed] penalty_amount: &BigUint,
    );

    #[event("renew_event")]
    fn renew_event(
        &self,
        #[indexed] bond_id: &u64,
        #[indexed] caller: &ManagedAddress,
        #[indexed] unbond_timestmap: &u64,
    );

    #[event("proof_event")]
    fn proof_event(
        &self,
        #[indexed] compensation_id: &u64,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] nonce: &u64,
        proof_amount: &BigUint,
    );

    #[event("claim_refund_event")]
    fn claim_refund_event(
        &self,
        #[indexed] compensation_id: &u64,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] nonce: &u64,
        #[indexed] amount: &BigUint,
        #[indexed] refund_token_identifier: &TokenIdentifier,
        #[indexed] refund_token_nonce: &u64,
        #[indexed] refund_amount: &BigUint,
    );

    #[event("add_to_blacklist_event")]
    fn add_to_blacklist_event(
        &self,
        #[indexed] compensation_id: &u64,
        #[indexed] addresses: &MultiValueEncoded<ManagedAddress>,
    );

    #[event("remove_from_blacklist_event")]
    fn remove_from_blacklist_event(
        &self,
        #[indexed] compensation_id: &u64,
        #[indexed] addresses: &MultiValueEncoded<ManagedAddress>,
    );

    #[event("initiate_refund_event")]
    fn initiate_refund_event(
        &self,
        #[indexed] compensation_id: &u64,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] nonce: &u64,
        #[indexed] timestamp: &u64,
    );

    #[event("sanction_event")]
    fn sanction_event(
        &self,
        #[indexed] bond_id: &u64,
        #[indexed] compensation_id: &u64,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] nonce: &u64,
        #[indexed] penalty_amount: &BigUint,
    );

    #[event("modify_bond_event")]
    fn modify_bond_event(&self, #[indexed] bond_id: &u64, #[indexed] unbond_timestamp: &u64);

    #[event("set_accepted_callers_event")]
    fn set_accepted_callers_event(&self, #[indexed] callers: &MultiValueEncoded<ManagedAddress>);

    #[event("remove_accepted_callers_event")]
    fn remove_accepted_callers_event(&self, #[indexed] callers: &MultiValueEncoded<ManagedAddress>);

    #[event("set_bond_token_event")]
    fn set_bond_token_event(&self, #[indexed] token_identifier: &TokenIdentifier);

    #[event("set_period_and_bond_event")]
    fn set_period_and_bond_event(&self, #[indexed] period: &u64, #[indexed] bond: &BigUint);

    #[event("remove_period_and_bond_event")]
    fn remove_period_and_bond_event(&self, #[indexed] period: &u64, #[indexed] bond: &BigUint);

    #[event("set_administrator_event")]
    fn set_administrator_event(&self, #[indexed] administrator: &ManagedAddress);

    #[event("set_liveliness_stake_address")]
    fn set_liveliness_stake_address_event(&self, #[indexed] address: &ManagedAddress);

    #[event("set_top_up_administrator_event")]
    fn set_top_up_administrator_event(&self, #[indexed] top_up_administrator: &ManagedAddress);
}
