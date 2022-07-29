use crate::{circom::CircomBuilder, circom::CircomConfig};
use ark_std::rand::thread_rng;

use ark_bn254::Bn254;
use ark_groth16::{
    create_random_proof as prove, generate_random_parameters, prepare_verifying_key, verify_proof,
};

use wasm_bindgen::prelude::*;

// TODO: inputs are filenames for wasm/r1cs, witness bigints
// TODO: JsValue is a wrapper around a rust value, so we can use it to pass in the witness
#[wasm_bindgen]
pub fn groth16_prove(wasm_file: String, r1cs_file: String) -> Result<String, str> {
    let cfg = CircomConfig::<Bn254>::new(wasm_file, r1cs_file)?;
    let mut builder = CircomBuilder::new(cfg);

    let circom = builder.setup();
    // builder.set_witness(witness);

    let mut rng = thread_rng();
    let params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng)?;

    let circom = builder.build()?;

    let proof = prove(circom, &params, &mut rng)?;

    Ok("foo");
}
