mod utils;

use wasm_bindgen::prelude::*;

use bincode;
use rand::rngs::OsRng; 

use elgamal_ristretto::private::SecretKey;
use elgamal_ristretto::public::PublicKey;
use sinkhole_query::elgamal::{recover_scalar, Query};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, there!!");
}

#[wasm_bindgen]
pub fn generate_secret_key() -> Vec<u8> {
    let sk = SecretKey::new(&mut OsRng);
    bincode::serialize(&sk).unwrap()
}

#[wasm_bindgen]
pub fn get_public_key(sk_encoded: Vec<u8>) -> Vec<u8> {
    let sk = bincode::deserialize(&sk_encoded).unwrap();;
    let pk = PublicKey::from(&sk);
    bincode::serialize(&pk).unwrap()
}

#[wasm_bindgen]
pub fn generate_sinkhole_query(
    pk_encoded: Vec<u8>,
    size_storage: usize,
    query_index: usize,
) -> Vec<u8> {
    let pk = bincode::deserialize(&pk_encoded).unwrap();
    let query = Query::new(pk, size_storage, query_index).unwrap();
    bincode::serialize(&query.encrypted).unwrap()
}
