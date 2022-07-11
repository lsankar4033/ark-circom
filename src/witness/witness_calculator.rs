use super::fnv;
use color_eyre::Result;
use num_bigint::BigInt;
use num_traits::Zero;
use std::cell::Cell;

#[cfg(feature = "circom-2")]
use num::ToPrimitive;

#[derive(Clone, Debug)]
pub struct WitnessCalculator {}

// Error type to signal end of execution.
// From https://docs.wasmer.io/integrations/examples/exit-early
#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("{0}")]
struct ExitCode(u32);

#[cfg(feature = "circom-2")]
fn from_array32(arr: Vec<u32>) -> BigInt {
    let mut res = BigInt::zero();
    let radix = BigInt::from(0x100000000u64);
    for &val in arr.iter() {
        res = res * &radix + BigInt::from(val);
    }
    res
}

#[cfg(feature = "circom-2")]
fn to_array32(s: &BigInt, size: usize) -> Vec<u32> {
    let mut res = vec![0; size as usize];
    let mut rem = s.clone();
    let radix = BigInt::from(0x100000000u64);
    let mut c = size;
    while !rem.is_zero() {
        c -= 1;
        res[c] = (&rem % &radix).to_u32().unwrap();
        rem /= &radix;
    }

    res
}

impl WitnessCalculator {
    // NOTE: could insert witness here
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self {}
    }

    pub fn calculate_witness_element<E: ark_ec::PairingEngine>(
        &mut self,
        witness: Vec<BigInt>,
        sanity_check: bool,
    ) -> Result<Vec<E::Fr>> {
        use ark_ff::{FpParameters, PrimeField};

        let modulus = <<E::Fr as PrimeField>::Params as FpParameters>::MODULUS;

        // convert it to field elements
        use num_traits::Signed;
        let witness = witness
            .into_iter()
            .map(|w| {
                let w = if w.sign() == num_bigint::Sign::Minus {
                    // Need to negate the witness element if negative
                    modulus.into() - w.abs().to_biguint().unwrap()
                } else {
                    w.to_biguint().unwrap()
                };
                E::Fr::from(w)
            })
            .collect::<Vec<_>>();

        Ok(witness)
    }
}
