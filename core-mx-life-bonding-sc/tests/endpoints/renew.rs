use core_mx_life_bonding_sc::{storage::Bond, views::ProxyTrait};

use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id,
    scenario_model::{AddressValue, ScQueryStep, SetStateStep, TransferStep, TxExpect},
};

use crate::bonding_state::bonding_state::{
    ContractState, DATA_NFT_IDENTIFIER, FIRST_USER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER,
    ITHEUM_TOKEN_IDENTIFIER_EXPR, MINTER_CONTRACT_ADDRESS_EXPR,
    OWNER_BONDING_CONTRACT_ADDRESS_EXPR, SECOND_USER_ADDRESS_EXPR,
};

#[test]
fn renew() {
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

    state.renew(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Contract not ready")),
    );

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.renew(
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

    state.renew(
        SECOND_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Bond not found")),
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(2u64));

    state.renew(FIRST_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.get_bond(1u64))
            .expect_value(Bond {
                address: managed_address!(&first_user_address.clone()),
                bond_id: 1u64,
                token_identifier: managed_token_id!(DATA_NFT_IDENTIFIER),
                nonce: 1u64,
                lock_period: 10u64,
                bond_timestamp: 2u64,
                unbond_timestamp: 12u64,
                bond_amount: managed_biguint!(100u64),
                remaining_amount: managed_biguint!(100u64),
            }),
    );

    state.remove_lock_period_and_bond(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, 10u64, None);
    state.set_lock_period_and_bond(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, 20u64, 200u64, None);

    state.renew(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        Some(TxExpect::user_error("str:Invalid lock period")),
    );
}
