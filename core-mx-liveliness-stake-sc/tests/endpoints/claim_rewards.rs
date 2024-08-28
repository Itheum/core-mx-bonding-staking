use multiversx_sc::types::{BigUint, ReturnsResult};
use multiversx_sc_scenario::ExpectError;

use multiversx_sc_scenario::*;

use core_mx_life_bonding_sc::life_bonding_sc_proxy::LifeBondingContractProxy as life_bonding_proxy;
use core_mx_liveliness_stake::liveliness_stake_proxy::CoreMxLivelinessStakeProxy as liveliness_proxy;

use crate::{
    contract_state::contract_state::{
        ContractState, BONDING_CONTRACT_ADDRESS, DATA_NFT_TOKEN_IDENTIFIER, DEFAULT_DECIMALS,
        FIRST_USER_ADDRESS, ITHEUM_TOKEN_IDENTIFIER, LIVELINESS_STAKE_CONTRACT_ADDRESS,
        OWNER_ADDRESS, SECOND_USER_ADDRESS, THIRD_USER_ADDRESS,
    },
    utils::BigIntDec,
};

#[test]
fn claim_rewards_contract_not_ready() {
    let mut state = ContractState::new();

    state.deploy();

    state.claim_rewards(
        FIRST_USER_ADDRESS,
        Some(ExpectError(4, "Contract not ready")),
    );
}

#[test]
fn claim_rewards_tests() {
    let mut state = ContractState::new();

    state.deploy_and_set(38u64, 16u32); // 0.38 rewards per block
    state.bond_deploy_and_set(7_889_400u64, 1000); // 1.000 * 10^18
    state.set_contract_state_active(OWNER_ADDRESS, None);

    state.top_up_rewards(OWNER_ADDRESS, 2_000_000u64, None);

    state.world.current_block().block_timestamp(1u64);

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        FIRST_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        1u64,
        None,
    );

    state.bond(
        FIRST_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        1u64,
        7_889_400u64,
        1_000u64,
        None,
    );

    state.bond_set_vault_nonce(FIRST_USER_ADDRESS, 1u64, None);

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        SECOND_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        2u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        SECOND_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        3u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        SECOND_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        4u64,
        None,
    );

    state.bond(
        SECOND_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        2u64,
        7_889_400u64,
        1_000u64,
        None,
    );

    state.world.current_block().block_timestamp(2_592_000u64); // 1 month passed

    state.bond(
        SECOND_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        3u64,
        7_889_400u64,
        1_000u64,
        None,
    );

    state.bond(
        SECOND_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        4u64,
        7_889_400u64,
        1_000u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        THIRD_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        5u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        THIRD_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        6u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        THIRD_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        7u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        THIRD_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        8u64,
        None,
    );

    state.bond(
        THIRD_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        5u64,
        7_889_400u64,
        1000u64,
        None,
    );

    state.bond(
        THIRD_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        6u64,
        7_889_400u64,
        1000u64,
        None,
    );

    state.bond(
        THIRD_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        7u64,
        7_889_400u64,
        1000u64,
        None,
    );

    state.bond(
        THIRD_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        8u64,
        7_889_400u64,
        1000u64,
        None,
    );

    let total = state
        .world
        .query()
        .to(BONDING_CONTRACT_ADDRESS)
        .typed(life_bonding_proxy)
        .total_bond_amount()
        .returns(ReturnsResult)
        .run();

    assert_eq!(BigIntDec::from(8000u64, DEFAULT_DECIMALS), total);

    state.world.current_block().block_timestamp(2_592_900u64); // 1 month + 15 minutes -> rewards starts

    state.start_produce_rewards(OWNER_ADDRESS, None);

    state
        .world
        .current_block()
        .block_timestamp(2_679_300u64)
        .block_nonce(14_400); // 1 month + 15 minutes + 1 day -> produced rewards for 1 day

    let contract_details = state
        .world
        .query()
        .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
        .typed(liveliness_proxy)
        .contract_details()
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        BigIntDec::from(5472u64, 18u32),
        contract_details.accumulated_rewards
    );

    let liveliness_score = state
        .world
        .query()
        .to(BONDING_CONTRACT_ADDRESS)
        .typed(life_bonding_proxy)
        .get_address_bonds_avg_score(FIRST_USER_ADDRESS)
        .returns(ReturnsResult)
        .run();

    assert_eq!(BigUint::from(6603u64), liveliness_score); // 66,03 % liveliness

    let first_user_claimable_rewards_with_liveliness_score_applied = state
        .world
        .query()
        .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
        .typed(liveliness_proxy)
        .claimable_rewards(FIRST_USER_ADDRESS, Option::<bool>::None)
        .returns(ReturnsResult)
        .run();

    let first_user_claimable_rewards_with_full_liveliness = state
        .world
        .query()
        .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
        .typed(liveliness_proxy)
        .claimable_rewards(FIRST_USER_ADDRESS, Some(true))
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        BigUint::from(684000000000000000000u128), // 684 tokens
        first_user_claimable_rewards_with_full_liveliness
    );

    assert_eq!(
        BigIntDec::from(4516452, 14u32), // 451,6452 tokens
        first_user_claimable_rewards_with_liveliness_score_applied
    );

    let second_user_address_liveliness_score_avg = state
        .world
        .query()
        .to(BONDING_CONTRACT_ADDRESS)
        .typed(life_bonding_proxy)
        .get_address_bonds_avg_score(SECOND_USER_ADDRESS)
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        BigUint::from(8793u64),
        second_user_address_liveliness_score_avg
    );

    let second_user_address_claimable_rewards_with_liveliness_score_applied = state
        .world
        .query()
        .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
        .typed(liveliness_proxy)
        .claimable_rewards(SECOND_USER_ADDRESS, Option::<bool>::None)
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        BigIntDec::from(18043236u64, 14u32), // 1.804,3236 tokens
        second_user_address_claimable_rewards_with_liveliness_score_applied
    );

    let second_user_address_claimable_rewards_with_full_liveliness = state
        .world
        .query()
        .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
        .typed(liveliness_proxy)
        .claimable_rewards(SECOND_USER_ADDRESS, Some(true))
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        BigIntDec::from(2052u64, 18u32), // 2052 tokens
        second_user_address_claimable_rewards_with_full_liveliness
    );

    state.claim_rewards(SECOND_USER_ADDRESS, None);

    state
        .world
        .check_account(SECOND_USER_ADDRESS)
        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER, BigIntDec::from(38043236u64, 14u32));
    // 2000 active balance + 1.804,3236 claimed rewards

    state.bond_renew(FIRST_USER_ADDRESS, DATA_NFT_TOKEN_IDENTIFIER, 1u64, None);

    let updated = state
        .world
        .query()
        .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
        .typed(liveliness_proxy)
        .claimable_rewards(FIRST_USER_ADDRESS, Option::<bool>::None)
        .returns(ReturnsResult)
        .run();

    assert_eq!(BigUint::from(683931600000000000000u128), updated);

    state.claim_rewards(FIRST_USER_ADDRESS, None);

    state
        .world
        .check_account(FIRST_USER_ADDRESS)
        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER, BigIntDec::from(684u64, 18u32));

    state
        .world
        .check_account(FIRST_USER_ADDRESS)
        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER, BigIntDec::from(684u64, 18u32));

    state.world.current_block().block_nonce(28_800u64);

    state.stake_rewards(FIRST_USER_ADDRESS, None);

    let x = state
        .world
        .query()
        .to(BONDING_CONTRACT_ADDRESS)
        .typed(life_bonding_proxy)
        .get_bond(1u64)
        .returns(ReturnsResult)
        .run();

    assert_eq!(BigIntDec::from(1684, 18u32), x.remaining_amount); // 1000 bond + 684 rewards
}

#[test]
fn claim_rewards_max_apr_tests() {
    let mut state = ContractState::new();

    state.deploy_and_set(38u64, 16u32); // 0.38 rewards per block
    state.bond_deploy_and_set(7_889_400u64, 1000); // 1.000 * 10^18
    state.set_contract_state_active(OWNER_ADDRESS, None);

    state.top_up_rewards(OWNER_ADDRESS, 2_000_000u64, None);

    state.world.current_block().block_nonce(1u64);

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        FIRST_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        1u64,
        None,
    );

    state.start_produce_rewards(OWNER_ADDRESS, None);

    state.bond(
        FIRST_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        1u64,
        7_889_400u64,
        1_000u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        SECOND_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        2u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        SECOND_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        3u64,
        None,
    );

    state.bond_whitelist_for_bond(
        OWNER_ADDRESS,
        SECOND_USER_ADDRESS,
        DATA_NFT_TOKEN_IDENTIFIER,
        4u64,
        None,
    );

    state.bond(
        SECOND_USER_ADDRESS,
        ITHEUM_TOKEN_IDENTIFIER,
        DATA_NFT_TOKEN_IDENTIFIER,
        2u64,
        7_889_400u64,
        1_000u64,
        None,
    );

    state.world.current_block().block_nonce(432_000u64); // 1 month passed

    let contract_details = state
        .world
        .query()
        .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
        .typed(liveliness_proxy)
        .contract_details()
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        contract_details.accumulated_rewards,
        BigIntDec::from(16415962u64, 16u32) //164159,62 tokens
    );

    state.set_max_apr(OWNER_ADDRESS, 100u64, None); // 1%

    let contract_details = state
        .world
        .query()
        .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
        .typed(liveliness_proxy)
        .contract_details()
        .returns(ReturnsResult)
        .run();

    assert_eq!(
        contract_details.accumulated_rewards,
        BigUint::from(1643831811262993949u128)
    ); // 1.643,831 tokens which is ~ 1% of rewards without apr
}
