multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub struct CompensationCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    sc_ref: &'a C,
    pub compensation_id: u64,
    pub token_identifier: TokenIdentifier<C::Api>,
    pub nonce: u64,
    pub accumulated_amount: BigUint<C::Api>,
    pub proof_amount: BigUint<C::Api>,
    pub end_date: u64,
}

impl<'a, C> CompensationCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    pub fn new(sc_ref: &'a C, compensation_id: u64) -> Self {
        CompensationCache {
            sc_ref,
            compensation_id,
            token_identifier: sc_ref.compensation_token_identifer(compensation_id).get(),
            nonce: sc_ref.compensation_nonce(compensation_id).get(),
            accumulated_amount: sc_ref
                .compensation_accumulated_amount(compensation_id)
                .get(),
            proof_amount: sc_ref.compensation_proof_amount(compensation_id).get(),
            end_date: sc_ref.compensation_end_date(compensation_id).get(),
        }
    }

    pub fn clear(&mut self) {
        self.sc_ref
            .compensation_token_identifer(self.compensation_id)
            .clear();
        self.sc_ref.compensation_nonce(self.compensation_id).clear();
        self.sc_ref
            .compensation_accumulated_amount(self.compensation_id)
            .clear();
        self.sc_ref
            .compensation_proof_amount(self.compensation_id)
            .clear();
        self.sc_ref
            .compensation_end_date(self.compensation_id)
            .clear();
    }
}

impl<'a, C> Drop for CompensationCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    fn drop(&mut self) {
        self.sc_ref
            .compensation_accumulated_amount(self.compensation_id)
            .set(&self.accumulated_amount);
        self.sc_ref
            .compensation_proof_amount(self.compensation_id)
            .set(&self.proof_amount);
        self.sc_ref
            .compensation_end_date(self.compensation_id)
            .set(self.end_date);
    }
}
