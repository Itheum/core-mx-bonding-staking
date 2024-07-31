use crate::{errors::ERR_ALREADY_IN_STORAGE, events, storage};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Copy,
    ManagedVecItem,
)]
pub enum State {
    Inactive,
    Active,
}

pub const COMPENSATION_SAFE_PERIOD: u64 = 86_400;

#[multiversx_sc::module]
pub trait ConfigModule: storage::StorageModule + events::EventsModule {
    #[only_owner]
    #[endpoint(setAdministrator)]
    fn set_administrator(&self, administrator: ManagedAddress) {
        self.set_administrator_event(&administrator);

        if !self.administrator().is_empty() {
            require!(
                administrator != self.administrator().get(),
                ERR_ALREADY_IN_STORAGE
            );
        }
        self.administrator().set(administrator);
    }

    #[inline]
    fn is_contract_owner(&self, address: &ManagedAddress) -> bool {
        &(self.blockchain().get_owner_address()) == address
    }

    #[inline]
    fn is_admin(&self, address: &ManagedAddress) -> bool {
        &(self.administrator().get()) == address
    }

    #[inline]
    fn is_privileged(&self, address: &ManagedAddress) -> bool {
        self.is_contract_owner(address) || self.is_admin(address)
    }

    #[inline]
    fn is_state_active(&self, state: State) -> bool {
        state == State::Active
    }

    fn contract_is_ready(&self) -> bool {
        let mut is_ready = true;

        if !self.is_state_active(self.contract_state().get()) {
            is_ready = false;
        }

        if self.administrator().is_empty() {
            is_ready = false;
        }
        if self.accepted_callers().is_empty() {
            is_ready = false;
        }
        if self.bond_payment_token().is_empty() {
            is_ready = false;
        }
        if self.lock_periods().is_empty() {
            is_ready = false;
        }

        if self.liveliness_stake_address().is_empty() {
            is_ready = false;
        }

        if self.top_up_administrator().is_empty() {
            is_ready = false;
        }
        is_ready
    }

    #[view(getContractState)]
    #[storage_mapper("contract_state")]
    fn contract_state(&self) -> SingleValueMapper<State>;

    #[view(getAdministrator)]
    #[storage_mapper("administrator")]
    fn administrator(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTopUpAdministrator)]
    #[storage_mapper("top_up_administrator")]
    fn top_up_administrator(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getAcceptedCallers)]
    #[storage_mapper("accepted_callers")]
    fn accepted_callers(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getBondPaymentToken)]
    #[storage_mapper("bond_payment_token")]
    fn bond_payment_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getLockPeriods)]
    #[storage_mapper("lock_periods")]
    fn lock_periods(&self) -> SetMapper<u64>;

    #[view(getLockPeriodBondAmount)]
    #[storage_mapper("lock_period_bond_amount")]
    fn lock_period_bond_amount(&self, lock_period: u64) -> SingleValueMapper<BigUint>;

    #[view(getMinimumPenalty)]
    #[storage_mapper("minimum_penalty")]
    fn minimum_penalty(&self) -> SingleValueMapper<u64>;

    #[view(getMaximumPenalty)]
    #[storage_mapper("maximum_penalty")]
    fn maximum_penalty(&self) -> SingleValueMapper<u64>;

    #[view(getWithdrawPenalty)]
    #[storage_mapper("withdraw_penalty")]
    fn withdraw_penalty(&self) -> SingleValueMapper<u64>;
}
