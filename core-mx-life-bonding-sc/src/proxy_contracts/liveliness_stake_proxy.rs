// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct CoreMxLivelinessStakeProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for CoreMxLivelinessStakeProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = CoreMxLivelinessStakeProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        CoreMxLivelinessStakeProxyMethods { wrapped_tx: tx }
    }
}

pub struct CoreMxLivelinessStakeProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> CoreMxLivelinessStakeProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> CoreMxLivelinessStakeProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> CoreMxLivelinessStakeProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn claim_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimRewards")
            .original_result()
    }

    pub fn set_address_rewards_per_share<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setAddressRewardsPerShare")
            .argument(&address)
            .original_result()
    }

    pub fn stack_rewards<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("stackRewards")
            .argument(&address)
            .original_result()
    }

    pub fn stake_rewards<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("stakeRewards")
            .argument(&token_identifier)
            .original_result()
    }

    pub fn set_contract_state_active(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setContractStateActive")
            .original_result()
    }

    pub fn set_contract_state_inactive(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setContractStateInactive")
            .original_result()
    }

    pub fn set_max_apr<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        max_apr: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setMaxApr")
            .argument(&max_apr)
            .original_result()
    }

    pub fn set_rewards_token_identifier<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setRewardsTokenIdentifier")
            .argument(&token_identifier)
            .original_result()
    }

    pub fn set_per_block_rewards<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        per_block_amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setPerBlockRewardAmount")
            .argument(&per_block_amount)
            .original_result()
    }

    pub fn top_up_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("topUpRewards")
            .original_result()
    }

    pub fn withdraw_rewards<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("withdrawRewards")
            .argument(&amount)
            .original_result()
    }

    pub fn start_produce_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("startProduceRewards")
            .original_result()
    }

    pub fn end_produce_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("endProduceRewards")
            .original_result()
    }

    pub fn set_bond_contract_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        bond_contract_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setBondContractAddress")
            .argument(&bond_contract_address)
            .original_result()
    }

    pub fn set_administrator<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        administrator: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setAdministrator")
            .argument(&administrator)
            .original_result()
    }

    pub fn contract_state(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, State> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getContractState")
            .original_result()
    }

    pub fn rewards_state(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, State> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("rewardsState")
            .original_result()
    }

    pub fn administrator(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAdministrator")
            .original_result()
    }

    pub fn bond_contract_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("bondContractAddress")
            .original_result()
    }

    pub fn generate_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("generateAggregatedRewards")
            .original_result()
    }

    pub fn rewards_reserve(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("rewardsReserve")
            .original_result()
    }

    pub fn accumulated_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("accumulatedRewards")
            .original_result()
    }

    pub fn rewards_token_identifier(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("rewardsTokenIdentifier")
            .original_result()
    }

    pub fn rewards_per_block(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("rewardsPerBlock")
            .original_result()
    }

    pub fn last_reward_block_nonce(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("lastRewardBlockNonce")
            .original_result()
    }

    pub fn rewards_per_share(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("rewardsPerShare")
            .original_result()
    }

    pub fn address_last_reward_per_share<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addressLastRewardPerShare")
            .argument(&address)
            .original_result()
    }

    pub fn address_stack_rewards<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addressStackRewards")
            .argument(&address)
            .original_result()
    }

    pub fn max_apr(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("maxApr")
            .original_result()
    }

    pub fn claimable_rewards<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<Option<bool>>,
    >(
        self,
        caller: Arg0,
        opt_bypass_liveliness: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimableRewards")
            .argument(&caller)
            .argument(&opt_bypass_liveliness)
            .original_result()
    }

    pub fn contract_details(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ContractDetails<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("contractDetails")
            .original_result()
    }

    pub fn user_data_out<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        address: Arg0,
        token_identifier: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, (ContractDetails<Env::Api>, UserData<Env::Api>)> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("userDataOut")
            .argument(&address)
            .argument(&token_identifier)
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub enum State {
    Inactive,
    Active,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct ContractDetails<Api>
where
    Api: ManagedTypeApi,
{
    pub rewards_reserve: BigUint<Api>,
    pub accumulated_rewards: BigUint<Api>,
    pub rewards_token_identifier: TokenIdentifier<Api>,
    pub rewards_per_block: BigUint<Api>,
    pub rewards_per_share: BigUint<Api>,
    pub administrator: ManagedAddress<Api>,
    pub bond_contract_address: ManagedAddress<Api>,
    pub last_reward_block_nonce: u64,
    pub max_apr: BigUint<Api>,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct UserData<Api>
where
    Api: ManagedTypeApi,
{
    pub total_staked_amount: BigUint<Api>,
    pub user_staked_amount: BigUint<Api>,
    pub liveliness_score: BigUint<Api>,
    pub accumulated_rewards: BigUint<Api>,
    pub accumulated_rewards_bypass: BigUint<Api>,
    pub vault_nonce: u64,
}
