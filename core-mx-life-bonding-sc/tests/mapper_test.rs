use core_mx_life_bonding_sc::contexts::mappers::object_to_id_mapper::ObjectToIdMapper;
use multiversx_sc::{
    storage::{mappers::StorageMapper, StorageKey},
    types::TokenIdentifier,
};
use multiversx_sc_scenario::api::SingleTxApi;

pub const TOKEN_IDENTIFIER: &[u8] = b"TEST-1234";

#[test]
fn test_mapper() {
    // Create an instance of ObjectToIdMapper with mock storage API
    let mapper: ObjectToIdMapper<SingleTxApi, (TokenIdentifier<SingleTxApi>, u64)> =
        ObjectToIdMapper::new(StorageKey::new(b"object_to_id_mapper"));

    let token_identifier = TokenIdentifier::<SingleTxApi>::from(TOKEN_IDENTIFIER);
    let nonce = 1u64;

    let id = mapper.get_id((token_identifier.clone(), nonce));

    assert_eq!(id, 0u64); // is not in the storage

    let id = mapper.get_id_or_insert((token_identifier.clone(), nonce));

    assert_eq!(id, 1u64);

    let id = mapper.get_id_or_insert((token_identifier.clone(), nonce));

    assert_eq!(id, 1u64); // same id as before

    assert!(mapper.contains_id(1u64));
    assert!(!mapper.contains_id(2u64));

    let (token_identifier_from_storage, nonce_from_storage) = mapper.get_object(1u64).unwrap();

    let response = mapper.get_object(0u64);

    assert_eq!(response, Option::None);

    let response_none = mapper.get_object(10u64);

    assert_eq!(response_none, Option::None);

    print!(
        "token_identifier_from_storage: {:?}",
        token_identifier_from_storage
    );

    assert_eq!(token_identifier_from_storage, token_identifier.clone());
    assert_eq!(nonce_from_storage, nonce);

    // new entry to the storage

    let another_token_identifier = TokenIdentifier::<SingleTxApi>::from(TOKEN_IDENTIFIER);
    let another_nonce = 3u64;

    let id = mapper.insert_new((another_token_identifier.clone(), another_nonce));

    assert_eq!(id, 2u64);

    let same_id = mapper.get_id_or_insert((another_token_identifier.clone(), another_nonce));

    assert_eq!(same_id, 2u64);

    assert!(mapper.contains_id(2u64));

    // remove by id

    let removed = mapper.remove_by_id(2u64);

    assert_eq!(
        removed,
        Option::Some((another_token_identifier.clone(), another_nonce))
    );

    let removed_again = mapper.remove_by_id(2u64);

    assert_eq!(removed_again, Option::None);

    // remove by object

    let removed_by_object = mapper.remove_by_object((token_identifier.clone(), nonce));

    assert_eq!(removed_by_object, 1u64);

    let removed_by_object_again = mapper.remove_by_object((token_identifier.clone(), nonce));

    assert_eq!(removed_by_object_again, 0u64);
}

#[test]
#[should_panic(expected = "Object already registered")]
fn test_insert_new_error_handling() {
    // Create an instance of ObjectToIdMapper with mock storage API
    let mapper: ObjectToIdMapper<SingleTxApi, (TokenIdentifier<SingleTxApi>, u64)> =
        ObjectToIdMapper::new(StorageKey::new(b"object_to_id_mapper"));

    // Create a test TokenIdentifier and nonce
    let token_identifier = TokenIdentifier::<SingleTxApi>::from(TOKEN_IDENTIFIER);
    let nonce = 3u64;

    // Attempt to insert a new object, expect a panic if insertion fails
    let _id = mapper.insert_new((token_identifier.clone(), nonce));
    let _same_id = mapper.get_id_or_insert((token_identifier.clone(), nonce));
    let _another_id = mapper.insert_new((token_identifier.clone(), nonce)); // This should panic
}

#[test]
#[should_panic(expected = "Unknown object")]
fn test_get_id_non_zero() {
    let mapper: ObjectToIdMapper<SingleTxApi, (TokenIdentifier<SingleTxApi>, u64)> =
        ObjectToIdMapper::new(StorageKey::new(b"object_to_id_mapper"));

    let token_identifier = TokenIdentifier::<SingleTxApi>::from(TOKEN_IDENTIFIER);
    let nonce = 3u64;

    let id = mapper.get_id_or_insert((token_identifier.clone(), nonce));
    let found_id = mapper.get_id_non_zero((token_identifier.clone(), nonce));

    assert_eq!(id, found_id);

    mapper.get_id_non_zero((token_identifier.clone(), 4u64)); // This should panic
}
