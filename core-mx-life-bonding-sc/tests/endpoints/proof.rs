
use core_mx_life_bonding_sc::{
    storage::{PenaltyType, Refund},
    views::ProxyTrait,
};
use multiversx_sc::{
    codec::multi_types::OptionalValue,
    types::{EsdtTokenPayment, ManagedVec, MultiValueEncoded},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_token_id,
    scenario_model::{
        AddressValue, CheckAccount, CheckStateStep, ScQueryStep, SetStateStep, TransferStep,
        TxExpect,
    },
};

use crate::bonding_state::bonding_state::{
    ContractState, ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, BONDING_CONTRACT_ADDRESS_EXPR,
    DATA_NFT_IDENTIFIER, DATA_NFT_IDENTIFIER_EXPR, FIRST_USER_ADDRESS_EXPR,
    ITHEUM_TOKEN_IDENTIFIER, ITHEUM_TOKEN_IDENTIFIER_EXPR, MINTER_CONTRACT_ADDRESS_EXPR,
    OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
};

#[test]
fn proof_test() {
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

    state.proof(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        Some(TxExpect::user_error("str:Contract not ready")),
    );

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.proof(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
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

    state.proof(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        Some(TxExpect::user_error("str:Invalid timeline to proof")),
    );

    state.sanction(
        ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        PenaltyType::Maximum,
        OptionalValue::None,
        None,
    );

    state.proof(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        Some(TxExpect::user_error("str:Invalid timeline to proof")),
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
        .set_state_step(SetStateStep::new().block_timestamp(13u64));

    state.proof(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        Some(TxExpect::user_error("str:Invalid timeline to proof")),
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(12u64));

    let mut multiValue = MultiValueEncoded::new();

    multiValue.push(1u64);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_address_refund_for_compensations(
                managed_address!(&first_user_address),
                multiValue.clone(),
            ))
            .expect_value(ManagedVec::<StaticApi, Refund<StaticApi>>::new()),
    );

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_address_refund_for_compensation(
                managed_address!(&first_user_address),
                managed_token_id!(DATA_NFT_IDENTIFIER),
                1u64,
            ))
            .expect_value(Option::<Refund<StaticApi>>::None),
    );

    state.proof(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        None,
    );

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

    let refund = Refund {
        address: managed_address!(&first_user_address),
        proof_of_refund: EsdtTokenPayment {
            token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
            amount: 2u64.into(),
            token_nonce: 1u64,
        },
        compensation_id: 1u64,
    };

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_address_refund_for_compensation(
                managed_address!(&first_user_address),
                managed_token_id!(DATA_NFT_IDENTIFIER),
                1u64,
            ))
            .expect_value(Some(refund.clone())),
    );

    let mut managedVec = ManagedVec::new();

    managedVec.push(refund.clone());

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_address_refund_for_compensations(
                managed_address!(&first_user_address),
                multiValue,
            ))
            .expect_value(managedVec),
    );
}
