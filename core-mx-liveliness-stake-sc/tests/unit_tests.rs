use core_mx_liveliness_stake::{
    config::{ConfigModule, State},
    storage::StorageModule,
    CoreMxLivelinessStake,
};
use multiversx_sc_scenario::{
    api::SingleTxApi, imports::AddressValue, managed_address, managed_biguint, managed_token_id,
};

mod contract_state;
mod endpoints;
pub mod utils;

#[test]
fn contract_ready_test() {
    let contract = core_mx_liveliness_stake::contract_obj::<SingleTxApi>();

    contract.init();

    assert!(!contract.contract_is_ready());

    contract.administrator().set(managed_address!(
        &AddressValue::from("address:admin").to_address()
    ));

    assert!(!contract.contract_is_ready());

    contract.contract_state().set(State::Active);

    assert!(!contract.contract_is_ready());

    contract.bond_contract_address().set(managed_address!(
        &AddressValue::from("address:bond").to_address()
    ));

    assert!(!contract.contract_is_ready());

    contract
        .rewards_token_identifier()
        .set(managed_token_id!(b"Token-124"));

    assert!(!contract.contract_is_ready());

    contract.rewards_reserve().set(managed_biguint!(100u64));

    assert!(!contract.contract_is_ready());

    contract.rewards_per_block().set(managed_biguint!(100u64));

    assert!(contract.contract_is_ready());
}
