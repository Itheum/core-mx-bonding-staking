use multiversx_sc_scenario::ExpectError;

use crate::contract_state::contract_state::{
    ContractState, ADMIN_ADDRESS, ANOTHER_TOKEN_IDENTIFIER, BONDING_CONTRACT_ADDRESS,
    FIRST_USER_ADDRESS, ITHEUM_TOKEN_IDENTIFIER, OWNER_ADDRESS,
};

#[test]
fn admin_endpoint() {
    let mut contract_state = ContractState::new();

    contract_state.deploy();
    contract_state.set_administrator(OWNER_ADDRESS, ADMIN_ADDRESS, None);

    contract_state
        .set_contract_state_active(FIRST_USER_ADDRESS, Some(ExpectError(4, "Not privileged")));

    contract_state.set_contract_state_active(ADMIN_ADDRESS, None);
    contract_state.set_contract_state_inactive(OWNER_ADDRESS, None);
    contract_state.set_contract_state_active(OWNER_ADDRESS, None);

    contract_state
        .set_contract_state_inactive(FIRST_USER_ADDRESS, Some(ExpectError(4, "Not privileged")));

    contract_state.set_contract_state_inactive(ADMIN_ADDRESS, None);

    contract_state.set_rewards_token_identifier(
        FIRST_USER_ADDRESS,
        ANOTHER_TOKEN_IDENTIFIER,
        Some(ExpectError(4, "Not privileged")),
    );

    contract_state.set_rewards_token_identifier(ADMIN_ADDRESS, ANOTHER_TOKEN_IDENTIFIER, None);
    contract_state.set_rewards_token_identifier(ADMIN_ADDRESS, ITHEUM_TOKEN_IDENTIFIER, None);

    contract_state.set_per_block_rewards(OWNER_ADDRESS, 20_000_000_u64, 18, None);
    contract_state.set_per_block_rewards(ADMIN_ADDRESS, 20_000_000_u64, 18, None);

    contract_state.set_per_block_rewards(
        FIRST_USER_ADDRESS,
        20_000_000_u64,
        18,
        Some(ExpectError(4, "Not privileged")),
    );

    contract_state.top_up_rewards(ADMIN_ADDRESS, 2_000_000u64, None);

    contract_state.top_up_rewards(OWNER_ADDRESS, 2_000_000u64, None);

    contract_state.top_up_rewards(
        FIRST_USER_ADDRESS,
        100u64,
        Some(ExpectError(4, "Not privileged")),
    );

    contract_state.withdraw_rewards(ADMIN_ADDRESS, 2_000_000u64, None);
    contract_state.withdraw_rewards(OWNER_ADDRESS, 2_000_000u64, None);

    contract_state.withdraw_rewards(
        FIRST_USER_ADDRESS,
        100u64,
        Some(ExpectError(4, "Not privileged")),
    );

    contract_state
        .start_produce_rewards(FIRST_USER_ADDRESS, Some(ExpectError(4, "Not privileged")));

    contract_state.start_produce_rewards(ADMIN_ADDRESS, None);

    contract_state.start_produce_rewards(OWNER_ADDRESS, None);

    contract_state.end_produce_rewards(ADMIN_ADDRESS, None);

    contract_state.end_produce_rewards(OWNER_ADDRESS, None);

    contract_state.end_produce_rewards(FIRST_USER_ADDRESS, Some(ExpectError(4, "Not privileged")));

    contract_state.set_bond_contract_address(
        FIRST_USER_ADDRESS,
        BONDING_CONTRACT_ADDRESS,
        Some(ExpectError(4, "Not privileged")),
    );

    contract_state.set_bond_contract_address(ADMIN_ADDRESS, BONDING_CONTRACT_ADDRESS, None);
    contract_state.set_bond_contract_address(OWNER_ADDRESS, BONDING_CONTRACT_ADDRESS, None);

    contract_state.set_max_apr(OWNER_ADDRESS, 10u64, None);
    contract_state.set_max_apr(ADMIN_ADDRESS, 10u64, None);

    contract_state.set_max_apr(
        FIRST_USER_ADDRESS,
        10u64,
        Some(ExpectError(4, "Not privileged")),
    );
}
