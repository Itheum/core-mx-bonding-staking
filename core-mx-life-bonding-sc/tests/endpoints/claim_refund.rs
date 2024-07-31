use core_mx_life_bonding_sc::{
    config::COMPENSATION_SAFE_PERIOD,
    storage::{Compensation, PenaltyType},
    views::ProxyTrait,
};
use multiversx_sc::codec::multi_types::OptionalValue;
use multiversx_sc_scenario::{
    managed_token_id,
    scenario_model::{
        AddressValue, CheckAccount, CheckStateStep, ScQueryStep, SetStateStep, TransferStep,
        TxExpect,
    },
};

use crate::bonding_state::bonding_state::{
    ContractState, ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, BONDING_CONTRACT_ADDRESS_EXPR,
    DATA_NFT_IDENTIFIER, DATA_NFT_IDENTIFIER_EXPR, FIRST_USER_ADDRESS_EXPR,
    ITHEUM_TOKEN_IDENTIFIER, ITHEUM_TOKEN_IDENTIFIER_EXPR, MINTER_CONTRACT_ADDRESS_EXPR,
    OWNER_BONDING_CONTRACT_ADDRESS_EXPR, SECOND_USER_ADDRESS_EXPR, THIRD_USER_ADDRESS_EXPR,
};

#[test]
fn claim_refund_without_blacklist_test() {
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

    state.claim_refund(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Contract not ready")),
    );

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.world.transfer_step(
        // mocks the mint call in minter and transfers the bond amount
        TransferStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .to(MINTER_CONTRACT_ADDRESS_EXPR)
            .esdt_transfer(ITHEUM_TOKEN_IDENTIFIER, 0u64, 100u64),
    );

    state.claim_refund(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Unknown object")),
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
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Maximum,
        OptionalValue::None,
        None,
    );

    state.initiate_refund(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        12u64,
        None,
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(12u64));

    state.proof(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        None,
    );

    state.proof(
        SECOND_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        None,
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(13u64));

    state.claim_refund(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Invalid timeline to refund")),
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(13u64 + COMPENSATION_SAFE_PERIOD));

    state.claim_refund(
        THIRD_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Refund not found")),
    );

    state.claim_refund(FIRST_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: 50u64.into(),
                proof_amount: 2u64.into(),
                end_date: 12u64,
            }),
    );

    state.world.check_state_step(
        CheckStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            CheckAccount::new()
                .esdt_nft_balance_and_attributes(DATA_NFT_IDENTIFIER_EXPR, 1u64, 2u64, None::<u8>)
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "50"),
        ),
    );

    state.world.check_state_step(
        CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new()
                .esdt_nft_balance_and_attributes(DATA_NFT_IDENTIFIER_EXPR, 1u64, 2u64, None::<u8>)
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "50"),
        ),
    );

    state.claim_refund(SECOND_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state.world.check_state_step(
        CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new()
                .esdt_nft_balance_and_attributes(DATA_NFT_IDENTIFIER_EXPR, 1u64, 0u64, None::<u8>)
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"),
        ),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: 0u64.into(),
                proof_amount: 0u64.into(),
                end_date: 12u64,
            }),
    );

    state.world.check_state_step(
        CheckStateStep::new().put_account(
            SECOND_USER_ADDRESS_EXPR,
            CheckAccount::new()
                .esdt_nft_balance_and_attributes(DATA_NFT_IDENTIFIER_EXPR, 1u64, 2u64, None::<u8>)
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "50"),
        ),
    );
}

#[test]
fn claim_refund_with_blacklist_test() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();
    let first_user_address = state.first_user_address.clone();
    let third_user_address = state.third_user_address.clone();

    state
        .default_deploy_and_set(10u64, 100u64)
        .remove_accepted_caller(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin.clone(), None)
        .set_accepted_caller(
            OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
            AddressValue::from(MINTER_CONTRACT_ADDRESS_EXPR).to_address(),
            None,
        );

    state.claim_refund(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Contract not ready")),
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
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Maximum,
        OptionalValue::None,
        None,
    );

    state.initiate_refund(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        12u64,
        None,
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(12u64));

    state.proof(
        FIRST_USER_ADDRESS_EXPR, // the bond was slashed for the first user and it's trying to get refund
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        None,
    );

    state.proof(
        SECOND_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        None,
    );

    state.proof(
        THIRD_USER_ADDRESS_EXPR, // this is another address of the first user
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        None,
    );

    state.world.check_state_step(
        CheckStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            CheckAccount::new()
                .esdt_nft_balance_and_attributes(DATA_NFT_IDENTIFIER_EXPR, 1u64, 0u64, None::<u8>)
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"),
        ),
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(13u64)); // entering safe period of 24hrs(86_400 seconds)

    state.set_blacklist(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        1u64,
        first_user_address.clone(),
        None,
    );

    state.set_blacklist(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        1u64,
        third_user_address,
        None,
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: 100u64.into(),
                proof_amount: 6u64.into(),
                end_date: 12u64,
            }),
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(13u64 + COMPENSATION_SAFE_PERIOD));

    // first user was blacklisted and it's trying to get refund

    state.claim_refund(FIRST_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state.world.check_state_step(
        CheckStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            CheckAccount::new()
                .esdt_nft_balance_and_attributes(DATA_NFT_IDENTIFIER_EXPR, 1u64, 2u64, None::<u8>)
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"),
        ),
    );

    state.world.check_state_step(
        CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new()
                .esdt_nft_balance_and_attributes(DATA_NFT_IDENTIFIER_EXPR, 1u64, 4u64, None::<u8>)
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100"),
        ),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: 100u64.into(),
                proof_amount: 4u64.into(),
                end_date: 12u64,
            }),
    );

    // should get all the funds
    state.claim_refund(SECOND_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new().esdt_nft_balance_and_attributes(
                DATA_NFT_IDENTIFIER_EXPR,
                1u64,
                2u64,
                None::<u8>,
            ),
        ));

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            SECOND_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_nft_balance_and_attributes(
                DATA_NFT_IDENTIFIER_EXPR,
                1u64,
                2u64,
                None::<u8>,
            ),
        ));

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            SECOND_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, 200u64),
        ));

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_compensation(1u64))
            .expect_value(Compensation {
                compensation_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                accumulated_amount: 0u64.into(),
                proof_amount: 2u64.into(),
                end_date: 12u64,
            }),
    );

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64),
        ));

    state.claim_refund(THIRD_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            BONDING_CONTRACT_ADDRESS_EXPR,
            CheckAccount::new().esdt_nft_balance_and_attributes(
                DATA_NFT_IDENTIFIER_EXPR,
                1u64,
                0u64,
                None::<u8>,
            ),
        ));

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            THIRD_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_nft_balance_and_attributes(
                DATA_NFT_IDENTIFIER_EXPR,
                1u64,
                2u64,
                None::<u8>,
            ),
        ));
}
