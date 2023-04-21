pub mod bls;
pub mod bn256;
pub mod merkle;
pub mod rmd160;
pub mod kvpair;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalHostCallEntryTable(pub Vec<ExternalHostCallEntry>);

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalHostCallEntry {
    pub op: usize,
    pub value: u64,
    pub is_ret: bool,
}
