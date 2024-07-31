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

pub const BLOCK_TIME: u64 = 6;
pub const BLOCKS_IN_YEAR: u64 = 31_536_000 / BLOCK_TIME;

pub const DIVISION_SAFETY_CONST: u64 = 1_000_000_000;
pub const MAX_PERCENT: u64 = 10_000;

#[multiversx_sc::module]
pub trait ConfigModule: events::EventsModule + storage::StorageModule {
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

    fn contract_is_ready(&self) -> bool {
        let mut is_ready = true;

        if self.administrator().is_empty() {
            is_ready = false;
        }

        if !self.is_state_active(self.contract_state().get()) {
            is_ready = false;
        }

        if self.bond_contract_address().is_empty() {
            is_ready = false;
        }

        if self.rewards_token_identifier().is_empty() {
            is_ready = false;
        }

        if self.rewards_per_block().is_empty() {
            is_ready = false;
        }

        if self.rewards_reserve().is_empty() {
            is_ready = false;
        }

        is_ready
    }

    #[inline]
    fn can_produce_rewards(&self) -> bool {
        self.is_state_active(self.rewards_state().get())
    }

    #[inline]
    fn is_state_active(&self, state: State) -> bool {
        state == State::Active
    }

    #[view(getContractState)]
    #[storage_mapper("contract_state")]
    fn contract_state(&self) -> SingleValueMapper<State>;

    #[view(rewardsState)]
    #[storage_mapper("rewards_state")]
    fn rewards_state(&self) -> SingleValueMapper<State>;

    #[view(getAdministrator)]
    #[storage_mapper("administrator")]
    fn administrator(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(bondContractAddress)]
    #[storage_mapper("bond_contract_address")]
    fn bond_contract_address(&self) -> SingleValueMapper<ManagedAddress>;
}
