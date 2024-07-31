multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub struct StorageCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    sc_ref: &'a C,
    pub rewards_reserve: BigUint<C::Api>,
    pub accumulated_rewards: BigUint<C::Api>,
    pub rewards_token_identifier: TokenIdentifier<C::Api>,
    pub rewards_per_block: BigUint<C::Api>,
    pub rewards_per_share: BigUint<C::Api>,
    pub last_reward_block_nonce: u64,
    pub max_apr: BigUint<C::Api>,
}

impl<'a, C> StorageCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    pub fn new(sc_ref: &'a C) -> Self {
        StorageCache {
            sc_ref,
            rewards_reserve: sc_ref.rewards_reserve().get(),
            accumulated_rewards: sc_ref.accumulated_rewards().get(),
            rewards_token_identifier: sc_ref.rewards_token_identifier().get(),
            rewards_per_block: sc_ref.rewards_per_block().get(),
            rewards_per_share: sc_ref.rewards_per_share().get(),
            last_reward_block_nonce: sc_ref.last_reward_block_nonce().get(),
            max_apr: sc_ref.max_apr().get(),
        }
    }
}

impl<'a, C> Drop for StorageCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    fn drop(&mut self) {
        self.sc_ref
            .rewards_reserve()
            .set(self.rewards_reserve.clone());
        self.sc_ref
            .accumulated_rewards()
            .set(self.accumulated_rewards.clone());
        self.sc_ref
            .rewards_per_block()
            .set(self.rewards_per_block.clone());
        self.sc_ref
            .rewards_per_share()
            .set(self.rewards_per_share.clone());
        self.sc_ref
            .last_reward_block_nonce()
            .set(self.last_reward_block_nonce);
        self.sc_ref.max_apr().set(self.max_apr.clone());
    }
}
