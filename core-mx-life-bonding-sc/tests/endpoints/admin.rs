use core_mx_life_bonding_sc::{
    config::State,
    storage::{Bond, Compensation, PenaltyType, ProxyTrait as _},
    views::ProxyTrait,
};
use multiversx_sc::{codec::multi_types::OptionalValue, types::MultiValueEncoded};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id,
    scenario_model::{ScQueryStep, SetStateStep, TxExpect},
};

use crate::bonding_state::bonding_state::{
    ContractState, ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, DATA_NFT_IDENTIFIER,
    FIRST_USER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER, ITHEUM_TOKEN_IDENTIFIER_EXPR,
    OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
};

#[test]
fn pause_unpause_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();
    state.deploy().set_administrator(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        admin,
        Some(TxExpect::ok()),
    );

    state.check_contract_state(State::Inactive);

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));
    state.check_contract_state(State::Active);

    state.pause_contract(
        FIRST_USER_ADDRESS_EXPR,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.pause_contract(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));

    state.check_contract_state(State::Inactive);
}

#[test]
fn accepted_callers_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();
    let minter_address = state.first_user_address.clone();

    state
        .deploy()
        .set_administrator(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin, None)
        .set_accepted_caller(
            ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
            minter_address.clone(),
            None,
        );

    state.set_accepted_caller(
        FIRST_USER_ADDRESS_EXPR,
        minter_address.clone(),
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.remove_accepted_caller(
        FIRST_USER_ADDRESS_EXPR,
        minter_address.clone(),
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_accepted_caller(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        minter_address.clone(),
        Some(TxExpect::user_error("str:Already in storage")),
    );

    state.remove_accepted_caller(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        minter_address.clone(),
        None,
    );
}

#[test]
fn bond_token_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();

    state
        .deploy()
        .set_administrator(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin, None);

    state.set_bond_token(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        None,
    );

    state.set_bond_token(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        None,
    );

    state.set_bond_token(
        FIRST_USER_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        Some(TxExpect::user_error("str:Not privileged")),
    );
}

#[test]
fn periods_and_bonds_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();

    state
        .deploy()
        .set_administrator(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin, None);

    state.set_lock_period_and_bond(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, 900u64, 10u64, None);
    state.set_lock_period_and_bond(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, 400u64, 20u64, None);

    state.set_lock_period_and_bond(
        FIRST_USER_ADDRESS_EXPR,
        500u64,
        30u64,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.remove_lock_period_and_bond(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, 900u64, None);
    state.remove_lock_period_and_bond(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, 400u64, None);

    state.remove_lock_period_and_bond(
        FIRST_USER_ADDRESS_EXPR,
        500u64,
        Some(TxExpect::user_error("str:Not privileged")),
    );
}

#[test]
fn penalty_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();

    state
        .deploy()
        .set_administrator(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin, None);

    state.set_minimum_penalty(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, 100u64, None);
    state.set_minimum_penalty(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, 200u64, None);

    state.set_minimum_penalty(
        FIRST_USER_ADDRESS_EXPR,
        300u64,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_maximum_penalty(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, 500u64, None);
    state.set_maximum_penalty(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, 10_000u64, None);

    state.set_maximum_penalty(
        FIRST_USER_ADDRESS_EXPR,
        3000u64,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_withdraw_penalty(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, 4_000u64, None);
    state.set_withdraw_penalty(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, 10_000u64, None);

    state.set_withdraw_penalty(
        FIRST_USER_ADDRESS_EXPR,
        150_000u64,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_minimum_penalty(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        6_000u64,
        Some(TxExpect::user_error("str:Invalid penalty value")),
    );

    state.set_minimum_penalty(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        0u64,
        Some(TxExpect::user_error("str:Invalid penalty value")),
    );

    state.set_maximum_penalty(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        15_000u64,
        Some(TxExpect::user_error("str:Invalid penalty value")),
    );

    state.set_maximum_penalty(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        0u64,
        Some(TxExpect::user_error("str:Invalid penalty value")),
    );

    state.set_withdraw_penalty(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        15_000u64,
        Some(TxExpect::user_error("str:Invalid penalty value")),
    );

    state.set_withdraw_penalty(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        0u64,
        Some(TxExpect::user_error("str:Invalid penalty value")),
    );
}

#[test]
fn initiate_refund_test() {
    let mut state = ContractState::new();
    let first_user_address = state.first_user_address.clone();

    state
        .default_deploy_and_set(10u64, 100u64)
        .unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.initiate_refund(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        20u64,
        Some(TxExpect::user_error("str:Unknown object")),
    );

    // bond and compensation with id 1
    state.bond(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: managed_biguint!(0u64),
                proof_amount: managed_biguint!(0u64),
                end_date: 0u64,
            }),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                bond_id: 1u64,
                address: managed_address!(&first_user_address),
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 0u64,
                unbond_timestamp: 10u64,
                bond_amount: managed_biguint!(100u64),
                remaining_amount: managed_biguint!(100u64),
            }),
    );

    state.initiate_refund(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        20u64,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.initiate_refund(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        20u64,
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: managed_biguint!(0u64),
                proof_amount: managed_biguint!(0u64),
                end_date: 20u64,
            }),
    );
}

#[test]
fn modify_bond() {
    let mut state = ContractState::new();
    let first_user_address = state.first_user_address.clone();

    state
        .default_deploy_and_set(10u64, 100u64)
        .unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    // bond and compensation with id 1

    state.modify_bond(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Unknown object")),
    );

    state.bond(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: managed_biguint!(0u64),
                proof_amount: managed_biguint!(0u64),
                end_date: 0u64,
            }),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                bond_id: 1u64,
                address: managed_address!(&first_user_address),
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 0u64,
                unbond_timestamp: 10u64,
                bond_amount: managed_biguint!(100u64),
                remaining_amount: managed_biguint!(100u64),
            }),
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(1u64));

    state.modify_bond(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.modify_bond(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                bond_id: 1u64,
                address: managed_address!(&first_user_address),
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 0u64,
                unbond_timestamp: 1u64,
                bond_amount: managed_biguint!(100u64),
                remaining_amount: managed_biguint!(100u64),
            }),
    );
}

#[test]
fn sanction_test() {
    let mut state = ContractState::new();
    let first_user_address = state.first_user_address.clone();

    state
        .default_deploy_and_set(10u64, 100u64)
        .unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.sanction(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Custom,
        OptionalValue::Some(20u64),
        Some(TxExpect::user_error("str:Unknown object")),
    );

    // bond and compensation with id 1
    state.bond(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: managed_biguint!(0u64),
                proof_amount: managed_biguint!(0u64),
                end_date: 0u64,
            }),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                bond_id: 1u64,
                address: managed_address!(&first_user_address),
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 0u64,
                unbond_timestamp: 10u64,
                bond_amount: managed_biguint!(100u64),
                remaining_amount: managed_biguint!(100u64),
            }),
    );

    state.sanction(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Minimum,
        OptionalValue::None,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.sanction(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Minimum,
        OptionalValue::None,
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: managed_biguint!(5u64),
                proof_amount: managed_biguint!(0u64),
                end_date: 0u64,
            }),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                bond_id: 1u64,
                address: managed_address!(&first_user_address),
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 0u64,
                unbond_timestamp: 10u64,
                bond_amount: managed_biguint!(100u64),
                remaining_amount: managed_biguint!(95u64),
            }),
    );

    state.sanction(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Custom,
        OptionalValue::None,
        Some(TxExpect::user_error("str:Invalid penalty value")),
    );

    state.sanction(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Custom,
        OptionalValue::Some(1_000u64),
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: managed_biguint!(15u64),
                proof_amount: managed_biguint!(0u64),
                end_date: 0u64,
            }),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                bond_id: 1u64,
                address: managed_address!(&first_user_address),
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 0u64,
                unbond_timestamp: 10u64,
                bond_amount: managed_biguint!(100u64),
                remaining_amount: managed_biguint!(85u64),
            }),
    );

    state.sanction(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Maximum,
        OptionalValue::Some(10_000u64),
        Some(TxExpect::user_error("str:Invalid penalty value")),
    );

    state.sanction(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Custom,
        OptionalValue::Some(8_500u64),
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: managed_biguint!(100u64),
                proof_amount: managed_biguint!(0u64),
                end_date: 0u64,
            }),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                bond_id: 1u64,
                address: managed_address!(&first_user_address),
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 0u64,
                unbond_timestamp: 10u64,
                bond_amount: managed_biguint!(100u64),
                remaining_amount: managed_biguint!(0u64),
            }),
    );
}

#[test]
fn blacklist_test() {
    let mut state = ContractState::new();
    let first_user_address = state.first_user_address.clone();

    state
        .default_deploy_and_set(10u64, 100u64)
        .unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.set_blacklist(
        FIRST_USER_ADDRESS_EXPR,
        1u64,
        first_user_address.clone(),
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.remove_blacklist(
        FIRST_USER_ADDRESS_EXPR,
        1u64,
        first_user_address.clone(),
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.set_blacklist(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        1u64,
        first_user_address.clone(),
        Some(TxExpect::user_error("str:Compensation not found")),
    );

    state.remove_blacklist(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        1u64,
        first_user_address.clone(),
        Some(TxExpect::user_error("str:Compensation not found")),
    );

    state.bond(
        OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
        first_user_address.clone(),
        DATA_NFT_IDENTIFIER,
        1u64,
        10u64,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        None,
    );

    state.set_blacklist(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        1u64,
        first_user_address.clone(),
        None,
    );

    let mut blacklist = MultiValueEncoded::new();
    blacklist.push(managed_address!(&first_user_address));

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.compensation_blacklist(1u64))
            .expect_value(blacklist),
    );

    state.remove_blacklist(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        1u64,
        first_user_address.clone(),
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.compensation_blacklist(1u64))
            .expect_value(MultiValueEncoded::new()),
    );
}
