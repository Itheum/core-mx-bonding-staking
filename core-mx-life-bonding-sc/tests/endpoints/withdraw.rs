use core_mx_life_bonding_sc::{
    storage::{Bond, Compensation, PenaltyType},
    views::ProxyTrait,
};
use multiversx_sc::{
    codec::multi_types::OptionalValue,
    types::{BigUint, ManagedVec},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_token_id,
    scenario_model::{
        AddressValue, CheckAccount, CheckStateStep, ScQueryStep, SetStateStep, TransferStep,
        TxExpect,
    },
};

use crate::bonding_state::bonding_state::{
    ContractState, BONDING_CONTRACT_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, FIRST_USER_ADDRESS_EXPR,
    ITHEUM_TOKEN_IDENTIFIER, ITHEUM_TOKEN_IDENTIFIER_EXPR, MINTER_CONTRACT_ADDRESS_EXPR,
    OWNER_BONDING_CONTRACT_ADDRESS_EXPR, SECOND_USER_ADDRESS_EXPR,
};

#[test]
fn withdraw_with_withdraw_penalty_test() {
    let mut state = ContractState::new();
    let first_user_address = state.first_user_address.clone();
    let admin = state.admin.clone();

    state
        .default_deploy_and_set(10u64, 100u64)
        .remove_accepted_caller(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin.clone(), None)
        .set_accepted_caller(
            OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
            AddressValue::from(MINTER_CONTRACT_ADDRESS_EXPR).to_address(),
            None,
        );

    state.withdraw(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Contract not ready")),
    );

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.withdraw(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Unknown object")),
    );

    state.world.transfer_step(
        // mocks the mint call in minter and transfers the bond amount
        TransferStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .to(MINTER_CONTRACT_ADDRESS_EXPR)
            .esdt_transfer(ITHEUM_TOKEN_IDENTIFIER, 0u64, 100u64),
    );

    state.bond(
        MINTER_CONTRACT_ADDRESS_EXPR, // another bond contract acts as minter mock
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        None,
    );

    state.withdraw(
        SECOND_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Bond not found")),
    );

    state.withdraw(FIRST_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "20"),
        ));

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "80"),
        ));

    let bonds: ManagedVec<StaticApi, Bond<StaticApi>> = ManagedVec::new();

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_all_bonds())
            .expect_value(bonds),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: BigUint::from(80u64),
                proof_amount: BigUint::from(0u64),
                end_date: 0u64,
            }),
    );
}

#[test]
fn withdraw_after_penalty_was_enforced_test() {
    let mut state = ContractState::new();
    let first_user_address = state.first_user_address.clone();
    let second_user_address = state.second_user_address.clone();
    let admin = state.admin.clone();

    state
        .default_deploy_and_set(10u64, 100u64)
        .remove_accepted_caller(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin.clone(), None)
        .set_accepted_caller(
            OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
            AddressValue::from(MINTER_CONTRACT_ADDRESS_EXPR).to_address(),
            None,
        );

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.world.transfer_step(
        // mocks the mint call in minter and transfers the bond amount
        TransferStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .to(MINTER_CONTRACT_ADDRESS_EXPR)
            .esdt_transfer(ITHEUM_TOKEN_IDENTIFIER, 0u64, 100u64),
    );

    state.bond(
        MINTER_CONTRACT_ADDRESS_EXPR, // another bond contract acts as minter mock
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        None,
    );

    state.sanction(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Custom,
        OptionalValue::Some(3_000u64),
        None,
    );

    state.withdraw(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error(
            "str:Penalties exceed withdrawal amount",
        )),
    );

    state.world.transfer_step(
        // mocks the mint call in minter and transfers the bond amount
        TransferStep::new()
            .from(SECOND_USER_ADDRESS_EXPR)
            .to(MINTER_CONTRACT_ADDRESS_EXPR)
            .esdt_transfer(ITHEUM_TOKEN_IDENTIFIER, 0u64, 100u64),
    );

    state.bond(
        MINTER_CONTRACT_ADDRESS_EXPR,
        second_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        2u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        None,
    );

    state.sanction(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        2u64,
        PenaltyType::Minimum,
        OptionalValue::None,
        None,
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(11u64));

    state.withdraw(SECOND_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 2u64, None);

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            SECOND_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "95"), // 5% penalty
        ));

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "105"),
        ));

    // after unbond period
    state.withdraw(FIRST_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "70"), //30 % penalty
        ));

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "35"), // 30% first user penalty + 5% second user penalty
        ));
}

#[test]
fn withdraw_with_no_penalty_test() {
    let mut state = ContractState::new();
    let first_user_address = state.first_user_address.clone();
    let admin = state.admin.clone();

    state
        .default_deploy_and_set(10u64, 100u64)
        .remove_accepted_caller(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin.clone(), None)
        .set_accepted_caller(
            OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
            AddressValue::from(MINTER_CONTRACT_ADDRESS_EXPR).to_address(),
            None,
        );

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.world.transfer_step(
        // mocks the mint call in minter and transfers the bond amount
        TransferStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .to(MINTER_CONTRACT_ADDRESS_EXPR)
            .esdt_transfer(ITHEUM_TOKEN_IDENTIFIER, 0u64, 100u64),
    );

    state.bond(
        MINTER_CONTRACT_ADDRESS_EXPR, // another bond contract acts as minter mock
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        None,
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(11u64));

    state.withdraw(FIRST_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    let no_bond: ManagedVec<StaticApi, Bond<StaticApi>> = ManagedVec::new();
    let no_compensation: ManagedVec<StaticApi, Compensation<StaticApi>> = ManagedVec::new();

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_all_bonds())
            .expect_value(no_bond),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_paged_compensations(1u64, 1u64))
            .expect_value(no_compensation),
    );
}
