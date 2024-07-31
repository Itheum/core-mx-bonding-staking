use core_mx_life_bonding_sc::{
    storage::{Bond, Compensation},
    views::ProxyTrait,
};
use multiversx_sc::{
    codec::multi_types::MultiValue2,
    types::{BigUint, ManagedVec, MultiValueEncoded},
};
use multiversx_sc_scenario::{
    managed_address, managed_token_id,
    scenario_model::{
        AddressValue, CheckAccount, CheckStateStep, ScQueryStep, TransferStep, TxExpect,
    },
};

use crate::bonding_state::bonding_state::{
    ContractState, ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, ANOTHER_TOKEN_IDENTIFIER_EXPR,
    BONDING_CONTRACT_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, FIRST_USER_ADDRESS_EXPR,
    ITHEUM_TOKEN_IDENTIFIER, ITHEUM_TOKEN_IDENTIFIER_EXPR, MINTER_CONTRACT_ADDRESS_EXPR,
    OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
};

#[test]
fn bond() {
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
        )
        .unpause_contract(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, None);

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
        (ANOTHER_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        Some(TxExpect::user_error("str:Invalid token identifier")),
    );

    state.bond(
        MINTER_CONTRACT_ADDRESS_EXPR, // another bond contract acts as minter mock
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        11u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 50u64),
        Some(TxExpect::user_error("str:Invalid lock period")),
    );

    state.bond(
        MINTER_CONTRACT_ADDRESS_EXPR, // another bond contract acts as minter mock
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 50u64),
        Some(TxExpect::user_error("str:Invalid amount")),
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
        .check_state_step(CheckStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"),
        ));

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100"),
        ));

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: BigUint::from(0u64),
                proof_amount: BigUint::from(0u64),
                end_date: 0u64,
            }),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                bond_id: 1u64,
                address: managed_address!(&first_user_address.clone()),
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 0u64,
                bond_amount: BigUint::from(100u64),
                unbond_timestamp: 10u64,
                remaining_amount: BigUint::from(100u64),
            }),
    );

    let mut managed_vec_bond = ManagedVec::new();
    managed_vec_bond.push(Bond {
        bond_id: 1u64,
        address: managed_address!(&first_user_address.clone()),
        token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
        nonce: 1u64,
        lock_period: 10u64,
        bond_timestamp: 0u64,
        bond_amount: BigUint::from(100u64),
        unbond_timestamp: 10u64,
        remaining_amount: BigUint::from(100u64),
    });

    state.world.sc_query(
        ScQueryStep::new()
            .call(
                state
                    .contract
                    .get_address_bonds(managed_address!(&first_user_address.clone())),
            )
            .expect_value(managed_vec_bond.clone()),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_paged_bonds(0u64, 1u64))
            .expect_value(managed_vec_bond.clone()),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bonds_len())
            .expect_value(1usize),
    );

    let mut bonds_ids = MultiValueEncoded::new();
    bonds_ids.push(1u64);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bonds(bonds_ids.clone()))
            .expect_value(managed_vec_bond.clone()),
    );

    let mut compensation_tokens = MultiValueEncoded::new();

    compensation_tokens.push(MultiValue2((managed_token_id!(DATA_NFT_IDENTIFIER), 1u64)));

    compensation_tokens.push(MultiValue2((managed_token_id!(DATA_NFT_IDENTIFIER), 2u64)));
    let mut managed_vec_compensation = ManagedVec::new();

    managed_vec_compensation.push(Compensation {
        compensation_id: 1u64,
        token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
        nonce: 1u64,
        accumulated_amount: BigUint::from(0u64),
        proof_amount: BigUint::from(0u64),
        end_date: 0u64,
    });

    state.world.sc_query(
        ScQueryStep::new()
            .call(
                state
                    .contract
                    .get_bonds_by_token_identifier_nonce(compensation_tokens.clone()),
            )
            .expect_value(managed_vec_bond.clone()),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensations(compensation_tokens))
            .expect_value(managed_vec_compensation.clone()),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_paged_compensations(0u64, 1u64))
            .expect_value(managed_vec_compensation.clone()),
    );

    let mut managed_vc_lock_period = ManagedVec::new();
    managed_vc_lock_period.push(10u64);

    let mut managed_vec_bond_amount = ManagedVec::new();
    managed_vec_bond_amount.push(BigUint::from(100u64));

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_lock_periods_bonds())
            .expect_value((managed_vc_lock_period, managed_vec_bond_amount)),
    );
}
