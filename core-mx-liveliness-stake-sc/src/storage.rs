multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(rewardsReserve)]
    #[storage_mapper("rewards_reserve")]
    fn rewards_reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(accumulatedRewards)]
    #[storage_mapper("accumulated_rewards")]
    fn accumulated_rewards(&self) -> SingleValueMapper<BigUint>;

    #[view(rewardsTokenIdentifier)]
    #[storage_mapper("rewards_token_identifier")]
    fn rewards_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(rewardsPerBlock)]
    #[storage_mapper("rewards_per_block")]
    fn rewards_per_block(&self) -> SingleValueMapper<BigUint>;

    #[view(lastRewardBlockNonce)]
    #[storage_mapper("last_reward_block_nonce")]
    fn last_reward_block_nonce(&self) -> SingleValueMapper<u64>;

    #[view(rewardsPerShare)]
    #[storage_mapper("rewards_per_share")]
    fn rewards_per_share(&self) -> SingleValueMapper<BigUint>;

    #[view(addressLastRewardPerShare)]
    #[storage_mapper("address_last_reward_per_share")]
    fn address_last_reward_per_share(&self, address: &ManagedAddress)
        -> SingleValueMapper<BigUint>;

    #[view(maxApr)]
    #[storage_mapper("max_apr")]
    fn max_apr(&self) -> SingleValueMapper<BigUint>;
}
