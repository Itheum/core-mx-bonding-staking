use multiversx_sc::{imports::SingleValue, types::BigUint};
use multiversx_sc_scenario::{
    imports::{
        Account, AddressValue, BytesValue, CheckAccount, CheckStateStep,
        ScQueryStep, SetStateStep, TransferStep, TxExpect,
    },
    managed_address, managed_token_id,
};

use core_mx_life_bonding_sc::storage::{Bond, ProxyTrait};
use core_mx_life_bonding_sc::views::ProxyTrait as _;

use crate::bonding_state::bonding_state::{
    ContractState, DATA_NFT_IDENTIFIER, DATA_NFT_IDENTIFIER_EXPR,
    FIRST_USER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER, ITHEUM_TOKEN_IDENTIFIER_EXPR,
    MINTER_CONTRACT_ADDRESS_EXPR, OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
};

#[test]
fn vault_tests() {
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

    state.world.set_state_step(
        SetStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            Account::new()
                .nonce(1)
                .balance("1_000")
                .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "300")
                .esdt_nft_balance(DATA_NFT_IDENTIFIER_EXPR, 1u64, 2u64, None::<BytesValue>),
        ),
    );

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, None);

    state.world.transfer_step(
        // mocks the mint call in minter and transfers the bond amount
        TransferStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .to(MINTER_CONTRACT_ADDRESS_EXPR)
            .esdt_transfer(ITHEUM_TOKEN_IDENTIFIER, 0u64, 100u64),
    );

    state.set_vault_nonce(
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
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64), // bond amount
        None,
    );

    state.set_vault_nonce(FIRST_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.contract.address_vault_nonce(
                AddressValue::from(FIRST_USER_ADDRESS_EXPR).to_address(),
                DATA_NFT_IDENTIFIER,
            ))
            .expect_value(SingleValue::from(1u64)), // vault nonce 1
    );

    state.top_up_vault(
        FIRST_USER_ADDRESS_EXPR,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        DATA_NFT_IDENTIFIER,
        1u64,
        None,
    );

    state.top_up_vault(
        FIRST_USER_ADDRESS_EXPR,
        (ITHEUM_TOKEN_IDENTIFIER_EXPR, 0u64, 100u64),
        DATA_NFT_IDENTIFIER,
        1u64,
        None,
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
                bond_amount: BigUint::from(300u64),
                unbond_timestamp: 10u64,
                remaining_amount: BigUint::from(300u64), // 100 (bond) + 200 top up
            }),
    );

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(11u64));

    state.withdraw(FIRST_USER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER, 1u64, None);

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "300"), //withdraw full amount after lock
        ));
}
