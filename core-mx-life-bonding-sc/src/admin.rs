use crate::{
    config::State,
    contexts::{
        bond_cache::BondCache,
        compensation_cache::{self, CompensationCache},
    },
    errors::{
        ERR_ALREADY_ACTIVE, ERR_ALREADY_INACTIVE, ERR_ALREADY_IN_STORAGE,
        ERR_COMPENSATION_NOT_FOUND, ERR_INVALID_PENALTY_VALUE, ERR_INVALID_TIMESTAMP,
        ERR_INVALID_TOKEN_IDENTIFIER, ERR_NOT_IN_STORAGE, ERR_NOT_PRIVILEGED,
    },
    events, only_privileged, proxy_contracts,
    storage::{self, PenaltyType},
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule:
    crate::config::ConfigModule + storage::StorageModule + events::EventsModule
{
    #[endpoint(initiateBond)]
    fn initiate_bond_for_address(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
    ) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let bond_id = self
            .bonds_ids()
            .get_id_or_insert((token_identifier.clone(), nonce));

        self.address_bonds(&address).insert(bond_id);
    }

    #[endpoint(setBlacklist)]
    fn add_to_black_list(
        &self,
        compensation_id: u64,
        addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        require!(
            self.compensations_ids().contains_id(compensation_id),
            ERR_COMPENSATION_NOT_FOUND
        );

        self.add_to_blacklist_event(&compensation_id, &addresses);

        for address in addresses.into_iter() {
            require!(
                !self
                    .compensation_blacklist(compensation_id)
                    .contains(&address),
                ERR_ALREADY_IN_STORAGE
            );
            self.compensation_blacklist(compensation_id).insert(address);
        }
    }

    #[endpoint(removeBlacklist)]
    fn remove_from_black_list(
        &self,
        compensation_id: u64,
        addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        require!(
            self.compensations_ids().contains_id(compensation_id),
            ERR_COMPENSATION_NOT_FOUND
        );

        self.remove_from_blacklist_event(&compensation_id, &addresses);

        for address in addresses.into_iter() {
            require!(
                self.compensation_blacklist(compensation_id)
                    .contains(&address),
                ERR_NOT_IN_STORAGE
            );
            self.compensation_blacklist(compensation_id)
                .swap_remove(&address);
        }
    }

    #[endpoint(initiateRefund)]
    fn initiate_refund(&self, token_identifier: TokenIdentifier, nonce: u64, timestamp: u64) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let compensation_id = self
            .compensations_ids()
            .get_id_non_zero((token_identifier.clone(), nonce));

        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(timestamp > current_timestamp, ERR_INVALID_TIMESTAMP);

        let mut compensation_cache =
            compensation_cache::CompensationCache::new(self, compensation_id);

        compensation_cache.end_date = timestamp;

        self.initiate_refund_event(&compensation_id, &token_identifier, &nonce, &timestamp);
    }

    #[endpoint(sanction)]
    fn sanction(
        &self,
        token_identifier: TokenIdentifier,
        nonce: u64,
        penalty: PenaltyType,
        custom_penalty: OptionalValue<u64>,
    ) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        if penalty != PenaltyType::Custom {
            require!(custom_penalty.is_none(), ERR_INVALID_PENALTY_VALUE);
        }

        let bond_id = self
            .bonds_ids()
            .get_id_non_zero((token_identifier.clone(), nonce));
        let compensation_id = self
            .compensations_ids()
            .get_id_non_zero((token_identifier, nonce));

        let mut bond_cache = BondCache::new(self, bond_id);
        let mut compensation_cache = CompensationCache::new(self, compensation_id);

        self.tx()
            .to(self.liveliness_stake_address().get())
            .typed(proxy_contracts::liveliness_stake_proxy::CoreMxLivelinessStakeProxy)
            .generate_rewards()
            .sync_call();

        let penalty = match penalty {
            PenaltyType::Minimum => self.minimum_penalty().get(),
            PenaltyType::Custom => {
                if let Some(custom_value) = custom_penalty.into_option() {
                    require!(
                        custom_value <= 10_000 && custom_value > self.minimum_penalty().get(),
                        ERR_INVALID_PENALTY_VALUE
                    );
                    custom_value
                } else {
                    sc_panic!(ERR_INVALID_PENALTY_VALUE);
                }
            }
            PenaltyType::Maximum => self.maximum_penalty().get(),
        };

        let penalty_amount =
            &bond_cache.bond_amount * &BigUint::from(penalty) / &BigUint::from(10_000u64);

        require!(
            bond_cache.remaining_amount >= penalty_amount,
            ERR_INVALID_PENALTY_VALUE
        );

        self.total_bond_amount()
            .update(|value| *value -= &penalty_amount);

        self.sanction_event(
            &bond_id,
            &compensation_id,
            &bond_cache.token_identifier,
            &nonce,
            &penalty_amount,
        );

        bond_cache.remaining_amount -= &penalty_amount;

        compensation_cache.accumulated_amount += &penalty_amount;
    }

    #[endpoint(modifyBond)]
    fn modify_bond(&self, token_identifier: TokenIdentifier, nonce: u64) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        let bond_id = self.bonds_ids().get_id_non_zero((token_identifier, nonce));

        let mut bond_cache = BondCache::new(self, bond_id);

        let current_timestamp = self.blockchain().get_block_timestamp();
        bond_cache.unbond_timestamp = current_timestamp;

        self.modify_bond_event(&bond_id, &bond_cache.unbond_timestamp);
    }

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

    #[endpoint(setAcceptedCallers)]
    fn set_accepted_callers(&self, callers: MultiValueEncoded<ManagedAddress>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.set_accepted_callers_event(&callers);
        for caller in callers.into_iter() {
            require!(
                !self.accepted_callers().contains(&caller),
                ERR_ALREADY_IN_STORAGE
            );
            self.accepted_callers().insert(caller);
        }
    }

    #[endpoint(removeAcceptedCallers)]
    fn remove_accepted_callers(&self, callers: MultiValueEncoded<ManagedAddress>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.remove_accepted_callers_event(&callers);
        for caller in callers.into_iter() {
            require!(
                self.accepted_callers().contains(&caller),
                ERR_NOT_IN_STORAGE
            );
            self.accepted_callers().swap_remove(&caller);
        }
    }

    #[endpoint(setBondToken)]
    fn set_bond_token(&self, token_identifier: TokenIdentifier) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            token_identifier.is_valid_esdt_identifier(),
            ERR_INVALID_TOKEN_IDENTIFIER
        );
        self.set_bond_token_event(&token_identifier);
        self.bond_payment_token().set(token_identifier);
    }

    #[endpoint(addPeriodsBonds)]
    fn add_lock_periods_with_bonds(&self, args: MultiValueEncoded<MultiValue2<u64, BigUint>>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        for input in args.into_iter() {
            let (lock_period, bond) = input.into_tuple();

            require!(
                !self.lock_periods().contains(&lock_period),
                ERR_ALREADY_IN_STORAGE
            );

            self.set_period_and_bond_event(&lock_period, &bond);
            self.lock_periods().insert(lock_period);
            self.lock_period_bond_amount(lock_period).set(bond);
        }
    }

    #[endpoint(removePeriodsBonds)]
    fn remove_lock_periods_with_bonds(&self, lock_periods: MultiValueEncoded<u64>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        for period in lock_periods.into_iter() {
            require!(self.lock_periods().contains(&period), ERR_NOT_IN_STORAGE);
            self.remove_period_and_bond_event(&period, &self.lock_period_bond_amount(period).get());
            self.lock_periods().remove(&period);
            self.lock_period_bond_amount(period).clear();
        }
    }

    #[endpoint(setMinimumPenalty)]
    fn set_minimum_penalty(&self, penalty: u64) {
        require!(penalty <= 5_000 && penalty > 0, ERR_INVALID_PENALTY_VALUE);
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.minimum_penalty_event(penalty);
        self.minimum_penalty().set(penalty);
    }

    #[endpoint(setMaximumPenalty)]
    fn set_maximum_penalty(&self, penalty: u64) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(penalty <= 10_000 && penalty > 0, ERR_INVALID_PENALTY_VALUE);
        self.maximum_penalty_event(penalty);
        self.maximum_penalty().set(penalty);
    }

    #[endpoint(setWithdrawPenalty)]
    fn set_withdraw_penalty(&self, penalty: u64) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(penalty <= 10_000 && penalty > 0, ERR_INVALID_PENALTY_VALUE);
        self.withdraw_penalty_event(penalty);
        self.withdraw_penalty().set(penalty);
    }

    #[endpoint(setLivelinessStakeAddress)]
    fn set_liveliness_stake_address(&self, address: ManagedAddress) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.set_liveliness_stake_address_event(&address);
        self.liveliness_stake_address().set(address);
    }

    #[endpoint(setTopUpAdministrator)]
    fn set_top_up_administrator(&self, address: ManagedAddress) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.set_top_up_administrator_event(&address);
        self.top_up_administrator().set(address);
    }
}
