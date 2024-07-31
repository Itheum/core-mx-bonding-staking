use crate::{config::State, contexts::mappers::object_to_id_mapper::ObjectToIdMapper};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug, Copy,
)]
pub enum PenaltyType {
    Minimum,
    Custom,
    Maximum,
}

#[derive(
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    ManagedVecItem,
    PartialEq,
    Eq,
    Debug,
)]
pub struct ContractConfiguration<M: ManagedTypeApi> {
    pub contract_state: State,
    pub bond_payment_token_identifier: TokenIdentifier<M>,
    pub lock_periods: ManagedVec<M, u64>,
    pub bond_amounts: ManagedVec<M, BigUint<M>>,
    pub minimum_penalty: u64,
    pub maximum_penalty: u64,
    pub withdraw_penalty: u64,
    pub accepted_callers: ManagedVec<M, ManagedAddress<M>>,
}

#[derive(
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    ManagedVecItem,
    PartialEq,
    Eq,
    Debug,
)]
pub struct Bond<M: ManagedTypeApi> {
    pub bond_id: u64,
    pub address: ManagedAddress<M>,
    pub token_identifier: TokenIdentifier<M>,
    pub nonce: u64,
    pub lock_period: u64,
    pub bond_timestamp: u64,
    pub unbond_timestamp: u64,
    pub bond_amount: BigUint<M>,
    pub remaining_amount: BigUint<M>,
}

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
    ManagedVecItem,
)]
pub struct Compensation<M: ManagedTypeApi> {
    pub compensation_id: u64,
    pub token_identifier: TokenIdentifier<M>,
    pub nonce: u64,
    pub accumulated_amount: BigUint<M>,
    pub proof_amount: BigUint<M>,
    pub end_date: u64,
}

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
    ManagedVecItem,
)]
pub struct Refund<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub proof_of_refund: EsdtTokenPayment<M>,
    pub compensation_id: u64,
}

#[multiversx_sc::module]
pub trait StorageModule {
    // Compensation storage
    #[storage_mapper("compensations_ids")]
    fn compensations_ids(&self) -> ObjectToIdMapper<Self::Api, (TokenIdentifier, u64)>;

    #[storage_mapper("compensations")]
    fn compensations(&self) -> UnorderedSetMapper<u64>;

    #[storage_mapper("compensation_token_identifer")]
    fn compensation_token_identifer(
        &self,
        compensation_id: u64,
    ) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("compensation_nonce")]
    fn compensation_nonce(&self, compensation_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("compensation_accumulated_amount")]
    fn compensation_accumulated_amount(&self, compensation_id: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("compensation_proof_amount")]
    fn compensation_proof_amount(&self, compensation_id: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("compensation_end_date")]
    fn compensation_end_date(&self, compensation_id: u64) -> SingleValueMapper<u64>;

    #[view(getCompensationBlacklist)]
    #[storage_mapper("compensation_blacklist")]
    fn compensation_blacklist(&self, compensation_id: u64) -> UnorderedSetMapper<ManagedAddress>;

    // Bond storage
    #[storage_mapper("bonds_ids")]
    fn bonds_ids(&self) -> ObjectToIdMapper<Self::Api, (TokenIdentifier, u64)>;

    #[storage_mapper("bonds")]
    fn bonds(&self) -> UnorderedSetMapper<u64>;

    #[storage_mapper("bond_address")]
    fn bond_address(&self, bond_id: u64) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("bond_token_identifier")]
    fn bond_token_identifier(&self, bond_id: u64) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("bond_nonce")]
    fn bond_nonce(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_lock_period")]
    fn bond_lock_period(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_timestamp")]
    fn bond_timestamp(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("unbond_timestamp")]
    fn unbond_timestamp(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_amount")]
    fn bond_amount(&self, bond_id: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("remaining_amount")]
    fn remaining_amount(&self, bond_id: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("address_bonds")]
    fn address_bonds(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("address_refund")]
    fn address_refund(
        &self,
        address: &ManagedAddress,
        compensation_id: u64,
    ) -> SingleValueMapper<Refund<Self::Api>>;

    #[view(getTotalBondAmount)]
    #[storage_mapper("total_bond_amount")]
    fn total_bond_amount(&self) -> SingleValueMapper<BigUint>;

    // NFMEid storage

    #[view(getAddressVaultNone)]
    #[storage_mapper("address_vault_nonce")]
    fn address_vault_nonce(
        &self,
        address: &ManagedAddress,
        token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<u64>;

    #[view(getLivelinessStakeAddress)]
    #[storage_mapper("liveliness_stake_address")]
    fn liveliness_stake_address(&self) -> SingleValueMapper<ManagedAddress>;
}
