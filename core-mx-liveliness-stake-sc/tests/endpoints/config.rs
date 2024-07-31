use multiversx_sc_scenario::ExpectError;

use crate::contract_state::contract_state::{
    ContractState, ADMIN_ADDRESS, FIRST_USER_ADDRESS, OWNER_ADDRESS,
};

#[test]
fn config_test() {
    let mut state = ContractState::new();

    state.deploy();

    state.set_administrator(
        FIRST_USER_ADDRESS,
        ADMIN_ADDRESS,
        Some(ExpectError(4, "Endpoint can only be called by owner")),
    );
    state.set_administrator(OWNER_ADDRESS, ADMIN_ADDRESS, None);
}
