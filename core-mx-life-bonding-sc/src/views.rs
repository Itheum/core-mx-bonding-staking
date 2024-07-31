use crate::{
    config, events,
    storage::{self, Bond, Compensation, ContractConfiguration, Refund},
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ViewsModule:
    storage::StorageModule + config::ConfigModule + events::EventsModule
{
    #[view(getBond)]
    fn get_bond(&self, bond_id: u64) -> Bond<Self::Api> {
        Bond {
            bond_id,
            address: self.bond_address(bond_id).get(),
            token_identifier: self.bond_token_identifier(bond_id).get(),
            nonce: self.bond_nonce(bond_id).get(),
            lock_period: self.bond_lock_period(bond_id).get(),
            bond_timestamp: self.bond_timestamp(bond_id).get(),
            unbond_timestamp: self.unbond_timestamp(bond_id).get(),
            bond_amount: self.bond_amount(bond_id).get(),
            remaining_amount: self.remaining_amount(bond_id).get(),
        }
    }

    #[view(getCompensation)]
    fn get_compensation(&self, compensation_id: u64) -> Compensation<Self::Api> {
        Compensation {
            compensation_id,
            token_identifier: self.compensation_token_identifer(compensation_id).get(),
            nonce: self.compensation_nonce(compensation_id).get(),
            accumulated_amount: self.compensation_accumulated_amount(compensation_id).get(),
            proof_amount: self.compensation_proof_amount(compensation_id).get(),
            end_date: self.compensation_end_date(compensation_id).get(),
        }
    }

    #[view(getCompensations)]
    fn get_compensations(
        &self,
        input: MultiValueEncoded<MultiValue2<TokenIdentifier, u64>>,
    ) -> ManagedVec<Compensation<Self::Api>> {
        input
            .into_iter()
            .filter_map(|value| {
                let (token_identifier, nonce) = value.into_tuple();
                let compensation_id = self.compensations_ids().get_id((token_identifier, nonce));
                if compensation_id != 0 {
                    Some(self.get_compensation(compensation_id))
                } else {
                    None
                }
            })
            .collect::<ManagedVec<Compensation<Self::Api>>>()
    }

    #[view(getPagedCompensations)]
    fn get_paged_compensations(
        &self,
        start_index: u64,
        end_index: u64,
    ) -> ManagedVec<Compensation<Self::Api>> {
        self.compensations()
            .into_iter()
            .skip(start_index as usize)
            .take((end_index - start_index + 1) as usize)
            .map(|compensation_id| self.get_compensation(compensation_id))
            .collect()
    }

    #[view(getAddressRefundForCompensation)]
    fn get_address_refund_for_compensation(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
    ) -> Option<Refund<Self::Api>> {
        let compensation_id = self.compensations_ids().get_id((token_identifier, nonce));
        if compensation_id == 0 {
            None
        } else if self.address_refund(&address, compensation_id).is_empty() {
            None
        } else {
            self.address_refund(&address, compensation_id).get();
            let refund = self.address_refund(&address, compensation_id).get();
            Some(refund)
        }
    }

    #[view(getAddressRefundForCompensations)]
    fn get_address_refund_for_compensations(
        &self,
        address: ManagedAddress,
        compensation_ids: MultiValueEncoded<u64>,
    ) -> ManagedVec<Refund<Self::Api>> {
        compensation_ids
            .into_iter()
            .filter_map(|compensation_id| {
                if self.address_refund(&address, compensation_id).is_empty() {
                    None
                } else {
                    Some(self.address_refund(&address, compensation_id).get())
                }
            })
            .collect::<ManagedVec<Refund<Self::Api>>>()
    }

    #[view(getBondsByTokenIdentifierNonce)]
    fn get_bonds_by_token_identifier_nonce(
        &self,
        input: MultiValueEncoded<MultiValue2<TokenIdentifier, u64>>,
    ) -> ManagedVec<Bond<Self::Api>> {
        input
            .into_iter()
            .filter_map(|value| {
                let (token_identifier, nonce) = value.into_tuple();
                let bond_id = self.bonds_ids().get_id((token_identifier, nonce));
                if bond_id != 0 {
                    Some(self.get_bond(bond_id))
                } else {
                    None
                }
            })
            .collect::<ManagedVec<Bond<Self::Api>>>()
    }

    #[view(getBonds)]
    fn get_bonds(&self, bond_ids: MultiValueEncoded<u64>) -> ManagedVec<Bond<Self::Api>> {
        bond_ids
            .into_iter()
            .map(|bond_id| self.get_bond(bond_id))
            .collect::<ManagedVec<Bond<Self::Api>>>()
    }

    #[view(getAddressBonds)]
    fn get_address_bonds(&self, address: ManagedAddress<Self::Api>) -> ManagedVec<Bond<Self::Api>> {
        self.address_bonds(&address)
            .into_iter()
            .map(|bond_id| self.get_bond(bond_id))
            .collect::<ManagedVec<Bond<Self::Api>>>()
    }

    #[view(getAddressBondsAvgScore)]
    fn get_address_bonds_avg_score(&self, address: ManagedAddress) -> BigUint<Self::Api> {
        let timestamp = self.blockchain().get_block_timestamp();
        let bonds = self.address_bonds(&address);

        if bonds.is_empty() {
            return BigUint::zero();
        }

        let mut total_score = BigUint::zero();
        let mut bond_count = BigUint::zero();

        for bond_id in bonds.iter() {
            let bond: Bond<<Self as ContractBase>::Api> = self.get_bond(bond_id);
            bond_count += &bond.remaining_amount;
            if timestamp >= bond.unbond_timestamp {
                continue;
            }
            let difference = bond.unbond_timestamp - timestamp;

            let bond_score = ((BigUint::from(10_000_000_000_000u64) / bond.lock_period)
                * difference)
                / BigUint::from(1_000_000_000u64);

            total_score += bond_score * &bond.remaining_amount;
        }

        // Calculate the weighted average bond score

        total_score / bond_count
    }

    #[view(getAddressBondsTotalValue)]
    fn get_address_bonds_total_value(&self, address: &ManagedAddress<Self::Api>) -> BigUint {
        self.address_bonds(address)
            .into_iter()
            .fold(BigUint::zero(), |acc, bond_id| {
                acc + self.remaining_amount(bond_id).get()
            })
    }

    #[view(getAddressBondsInfo)]
    fn get_address_bonds_info(&self, address: ManagedAddress) -> (BigUint, BigUint, BigUint) {
        let address_bonds_value = self.get_address_bonds_total_value(&address);
        let address_bonds_avg_score = self.get_address_bonds_avg_score(address);
        let total_bond_amount = self.total_bond_amount().get();

        (
            total_bond_amount,
            address_bonds_value,
            address_bonds_avg_score,
        )
    }

    #[view(getAllBonds)]
    fn get_all_bonds(&self) -> ManagedVec<Bond<Self::Api>> {
        self.bonds()
            .into_iter()
            .map(|bond_id| self.get_bond(bond_id))
            .collect()
    }

    #[view(getPagedBonds)]
    fn get_paged_bonds(&self, start_index: u64, end_index: u64) -> ManagedVec<Bond<Self::Api>> {
        self.bonds()
            .into_iter()
            .skip(start_index as usize)
            .take((end_index - start_index + 1) as usize)
            .map(|bond_id| self.get_bond(bond_id))
            .collect()
    }

    #[view(getBondsLen)]
    fn get_bonds_len(&self) -> usize {
        self.bonds().len()
    }

    #[view(getCompensationsLen)]
    fn get_compensations_len(&self) -> usize {
        self.compensations().len()
    }

    #[view(getLockPeriodsBonds)]
    fn get_lock_periods_bonds(&self) -> (ManagedVec<u64>, ManagedVec<BigUint>) {
        let lock_periods = self.lock_periods().into_iter().collect::<ManagedVec<u64>>();
        let bond_amounts = self
            .lock_periods()
            .into_iter()
            .map(|lock_period| self.lock_period_bond_amount(lock_period).get())
            .collect::<ManagedVec<BigUint>>();

        (lock_periods, bond_amounts)
    }

    #[view(getContractConfiguration)]
    fn get_contract_configuration(&self) -> ContractConfiguration<Self::Api> {
        let (lock_periods, bond_amounts) = self.get_lock_periods_bonds();
        ContractConfiguration {
            contract_state: self.contract_state().get(),
            bond_payment_token_identifier: self.bond_payment_token().get(),
            lock_periods,
            bond_amounts,
            minimum_penalty: self.minimum_penalty().get(),
            withdraw_penalty: self.withdraw_penalty().get(),
            maximum_penalty: self.maximum_penalty().get(),
            accepted_callers: self.accepted_callers().into_iter().collect(),
        }
    }
}
