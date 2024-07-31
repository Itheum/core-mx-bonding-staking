use multiversx_sc_scenario::imports::*;

use crate::utils::BigIntDec;

use core_mx_life_bonding_sc::life_bonding_sc_proxy::LifeBondingContractProxy as life_bonding_proxy;
use core_mx_liveliness_stake::liveliness_stake_proxy::CoreMxLivelinessStakeProxy as liveliness_proxy;

pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const ADMIN_ADDRESS: TestAddress = TestAddress::new("admin");
pub const LIVELINESS_STAKE_CONTRACT_ADDRESS: TestSCAddress =
    TestSCAddress::new("liveliness_stake_contract");
pub const BONDING_CONTRACT_ADDRESS: TestSCAddress = TestSCAddress::new("bonding_contract");

pub const LIVELINESS_CODE_PATH: MxscPath =
    MxscPath::new("output/core-mx-liveliness-stake.mxsc.json");

pub const BOND_CODE_PATH: MxscPath =
    MxscPath::new("test_external_contracts/core-mx-life-bonding-sc.mxsc.json");

pub const FIRST_USER_ADDRESS: TestAddress = TestAddress::new("first_user");
pub const SECOND_USER_ADDRESS: TestAddress = TestAddress::new("second_user");
pub const THIRD_USER_ADDRESS: TestAddress = TestAddress::new("third_user");

pub const ITHEUM_TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("ITHEUM-fce905");
pub const ANOTHER_TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("ANOTDD-clvb5s");
pub const DATA_NFT_TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("DATA-ac56b");

pub const DEFAULT_DECIMALS: u32 = 18;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        LIVELINESS_CODE_PATH,
        core_mx_liveliness_stake::ContractBuilder,
    );

    blockchain.register_contract(BOND_CODE_PATH, core_mx_life_bonding_sc::ContractBuilder);

    blockchain
}

pub struct ContractState {
    pub world: ScenarioWorld,
}

impl ContractState {
    pub fn new() -> Self {
        let mut world = world();

        world
            .account(OWNER_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(100u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(100_000_000u64, DEFAULT_DECIMALS),
            );

        world
            .account(ADMIN_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(100u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(100_000_000u64, DEFAULT_DECIMALS),
            );

        world
            .account(FIRST_USER_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(10u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(1_000u64, DEFAULT_DECIMALS),
            );

        world
            .account(SECOND_USER_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(10u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(5_000u64, DEFAULT_DECIMALS),
            );

        world
            .account(THIRD_USER_ADDRESS)
            .nonce(1)
            .balance(BigIntDec::from(10u64, DEFAULT_DECIMALS))
            .esdt_balance(
                ITHEUM_TOKEN_IDENTIFIER,
                BigIntDec::from(10_000u64, DEFAULT_DECIMALS),
            );

        Self { world }
    }

    pub fn deploy(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(liveliness_proxy)
            .init()
            .code(LIVELINESS_CODE_PATH)
            .new_address(LIVELINESS_STAKE_CONTRACT_ADDRESS)
            .run();
    }

    pub fn deploy_and_set(&mut self, rewards_per_block: u64, decimals: u32) {
        self.deploy();
        self.set_bond_contract_address(OWNER_ADDRESS, BONDING_CONTRACT_ADDRESS, None);
        self.set_rewards_token_identifier(OWNER_ADDRESS, ITHEUM_TOKEN_IDENTIFIER, None);
        self.set_administrator(OWNER_ADDRESS, ADMIN_ADDRESS, None);
        self.set_per_block_rewards(OWNER_ADDRESS, rewards_per_block, decimals, None);
    }

    pub fn set_administrator(
        &mut self,
        caller: TestAddress,
        administrator: TestAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_administrator(administrator)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_administrator(administrator)
                .run();
        }
    }

    pub fn set_contract_state_active(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_contract_state_active()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_contract_state_active()
                .run();
        }
    }

    pub fn set_contract_state_inactive(
        &mut self,
        caller: TestAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_contract_state_inactive()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_contract_state_inactive()
                .run();
        }
    }

    pub fn start_produce_rewards(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .start_produce_rewards()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .start_produce_rewards()
                .run();
        }
    }

    pub fn end_produce_rewards(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .end_produce_rewards()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .end_produce_rewards()
                .run();
        }
    }

    pub fn set_max_apr(&mut self, caller: TestAddress, max_apr: u64, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_max_apr(max_apr)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_max_apr(max_apr)
                .run();
        }
    }

    pub fn set_rewards_token_identifier(
        &mut self,
        caller: TestAddress,
        token_identifier: TestTokenIdentifier,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_rewards_token_identifier(token_identifier)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_rewards_token_identifier(token_identifier)
                .run();
        }
    }

    pub fn set_per_block_rewards(
        &mut self,
        caller: TestAddress,
        per_block_amount: u64,
        decimals: u32,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_per_block_rewards(BigIntDec::from(per_block_amount, decimals))
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_per_block_rewards(BigIntDec::from(per_block_amount, decimals))
                .run();
        }
    }

    pub fn top_up_rewards(
        &mut self,
        caller: TestAddress,
        payment: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .top_up_rewards()
                .esdt(EsdtTokenPayment {
                    token_identifier: ITHEUM_TOKEN_IDENTIFIER.into(),
                    amount: BigIntDec::from(payment, DEFAULT_DECIMALS),
                    token_nonce: 0u64,
                })
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .top_up_rewards()
                .esdt(EsdtTokenPayment {
                    token_identifier: ITHEUM_TOKEN_IDENTIFIER.into(),
                    amount: BigIntDec::from(payment, DEFAULT_DECIMALS),
                    token_nonce: 0u64,
                })
                .run();
        }
    }

    pub fn withdraw_rewards(
        &mut self,
        caller: TestAddress,
        amount: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .withdraw_rewards(BigIntDec::from(amount, DEFAULT_DECIMALS))
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .withdraw_rewards(BigIntDec::from(amount, DEFAULT_DECIMALS))
                .run();
        }
    }

    pub fn set_bond_contract_address(
        &mut self,
        caller: TestAddress,
        bond_contract_address: TestSCAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_bond_contract_address(bond_contract_address)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .set_bond_contract_address(bond_contract_address)
                .run();
        }
    }

    pub fn claim_rewards(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        let c = OptionalValue::<ManagedAddress<_>>::None;
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .claim_rewards(c)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .claim_rewards(c)
                .run();
        }
    }

    pub fn stake_rewards(&mut self, caller: TestAddress, expect: Option<ExpectError>) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .stake_rewards(DATA_NFT_TOKEN_IDENTIFIER)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .typed(liveliness_proxy)
                .stake_rewards(DATA_NFT_TOKEN_IDENTIFIER)
                .run();
        }
    }

    pub fn bond_deploy_and_set(&mut self, lock_period: u64, bond: u64) {
        self.bond_deploy();
        self.bond_set_administrator(OWNER_ADDRESS, ADMIN_ADDRESS, None);
        self.bond_set_accepted_caller(ADMIN_ADDRESS, ADMIN_ADDRESS, None);
        self.bond_set_bond_token(ADMIN_ADDRESS, ITHEUM_TOKEN_IDENTIFIER, None);
        self.bond_set_lock_period_and_bond(ADMIN_ADDRESS, lock_period, bond, None);
        self.bond_set_contract_state_active(OWNER_ADDRESS, None);
        self.bond_set_liveliness_stake_address(OWNER_ADDRESS, None);
        self.bond_set_top_up_admin_address(OWNER_ADDRESS, None);
    }

    // bonding contract functions
    pub fn bond_deploy(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(life_bonding_proxy)
            .init()
            .code(BOND_CODE_PATH)
            .new_address(BONDING_CONTRACT_ADDRESS)
            .run();
    }

    pub fn bond_set_top_up_admin_address(
        &mut self,
        caller: TestAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_top_up_administrator(ADMIN_ADDRESS)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_top_up_administrator(ADMIN_ADDRESS)
                .run();
        }
    }

    pub fn bond_set_liveliness_stake_address(
        &mut self,
        caller: TestAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_liveliness_stake_address(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_liveliness_stake_address(LIVELINESS_STAKE_CONTRACT_ADDRESS)
                .run();
        }
    }

    pub fn bond_set_contract_state_active(
        &mut self,
        caller: TestAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_contract_state_active()
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_contract_state_active()
                .run();
        }
    }

    pub fn bond_set_administrator(
        &mut self,
        caller: TestAddress,
        administrator: TestAddress,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_administrator(administrator)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_administrator(administrator)
                .run();
        }
    }

    pub fn bond_set_accepted_caller(
        &mut self,
        caller: TestAddress,
        accepted_caller: TestAddress,
        expect: Option<ExpectError>,
    ) {
        let mut multivalue = MultiValueEncoded::new();
        multivalue.push(accepted_caller.to_managed_address());

        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_accepted_callers(multivalue)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_accepted_callers(multivalue)
                .run();
        }
    }

    pub fn bond_set_bond_token(
        &mut self,
        caller: TestAddress,
        token: TestTokenIdentifier,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_bond_token(token)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_bond_token(token)
                .run();
        }
    }

    pub fn bond_set_lock_period_and_bond(
        &mut self,
        caller: TestAddress,
        lock_period: u64,
        bond: u64,
        expect: Option<ExpectError>,
    ) {
        let mut multivalue = MultiValueEncoded::new();

        multivalue.push(MultiValue2((
            lock_period,
            BigIntDec::from(bond, DEFAULT_DECIMALS),
        )));

        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .add_lock_periods_with_bonds(multivalue)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .add_lock_periods_with_bonds(multivalue)
                .run();
        }
    }

    pub fn bond_whitelist_for_bond(
        &mut self,
        caller: TestAddress,
        user: TestAddress,
        token: TestTokenIdentifier,
        nonce: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .initiate_bond_for_address(user, token, nonce)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .initiate_bond_for_address(user, token, nonce)
                .run();
        }
    }
    pub fn bond_set_vault_nonce(
        &mut self,
        caller: TestAddress,
        nonce: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_vault_nonce(DATA_NFT_TOKEN_IDENTIFIER, nonce)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .set_vault_nonce(DATA_NFT_TOKEN_IDENTIFIER, nonce)
                .run();
        }
    }

    pub fn bond(
        &mut self,
        caller: TestAddress,
        payment_token_identifier: TestTokenIdentifier,
        token_identifier: TestTokenIdentifier,
        nonce: u64,
        lock_period: u64,
        bond: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .bond(caller, token_identifier, nonce, lock_period)
                .esdt(EsdtTokenPayment {
                    token_identifier: payment_token_identifier.into(),
                    amount: BigIntDec::from(bond, DEFAULT_DECIMALS),
                    token_nonce: 0u64,
                })
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .bond(caller, token_identifier, nonce, lock_period)
                .esdt(EsdtTokenPayment {
                    token_identifier: payment_token_identifier.into(),
                    amount: BigIntDec::from(bond, DEFAULT_DECIMALS),
                    token_nonce: 0u64,
                })
                .run();
        }
    }

    pub fn bond_renew(
        &mut self,
        caller: TestAddress,
        token_identifier: TestTokenIdentifier,
        nonce: u64,
        expect: Option<ExpectError>,
    ) {
        if let Some(expect) = expect {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .renew(token_identifier, nonce)
                .with_result(expect)
                .run();
        } else {
            self.world
                .tx()
                .from(caller)
                .to(BONDING_CONTRACT_ADDRESS)
                .typed(life_bonding_proxy)
                .renew(token_identifier, nonce)
                .run();
        }
    }
}
