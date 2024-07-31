use multiversx_sc::{api::ManagedTypeApi, types::BigUint};

pub struct BigIntDec;

impl BigIntDec {
    /// Creates a new `BigUint` from a value and decimals
    pub fn from<M: ManagedTypeApi>(value: u64, decimals: u32) -> BigUint<M> {
        let factor = BigUint::from(10u64).pow(decimals);
        
        BigUint::from(value) * factor
    }
}
