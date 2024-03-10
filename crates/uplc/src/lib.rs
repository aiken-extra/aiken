pub mod ast;
pub mod builder;
pub mod builtins;
mod debruijn;
pub mod flat;
pub mod machine;
pub mod optimize;
pub mod parser;
mod pretty;
pub mod tx;

pub use pallas::codec::utils::KeyValuePairs;
pub use pallas::crypto::hash::Hash;
pub use pallas::ledger::primitives::{
    alonzo::{BigInt, Constr, PlutusData},
    babbage::{PostAlonzoTransactionOutput, TransactionInput, TransactionOutput, Value},
    Error, Fragment,
};

pub fn plutus_data(bytes: &[u8]) -> Result<PlutusData, Error> {
    PlutusData::decode_fragment(bytes)
}

pub fn plutus_data_to_bytes(data: &PlutusData) -> Result<Vec<u8>, Error> {
    PlutusData::encode_fragment(data)
}
