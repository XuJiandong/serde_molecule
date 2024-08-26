// Rust serde_molecule implementation of following schemas:
// https://github.com/nervosnetwork/ckb/blob/develop/util/gen-types/schemas/blockchain.mol

use serde::{Deserialize, Serialize};
use serde_molecule::{dynvec_serde, struct_serde};

type ProposalShortId = [u8; 10];

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Script {
    pub code_hash: [u8; 32],
    pub hash_type: u8,
    pub args: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct OutPoint {
    pub tx_hash: [u8; 32],
    pub index: u32,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct CellInput {
    pub since: u64,
    // By default, every field is considered as molecule table.
    // If it is molecule struct, we should annotate it explicitly.
    #[serde(with = "struct_serde")]
    pub previous_output: OutPoint,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct CellOutput {
    pub capacity: u64,
    pub lock: Script,
    pub type_: Option<Script>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct CellDep {
    #[serde(with = "struct_serde")]
    pub out_point: OutPoint,
    pub dep_type: u8,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct RawTransaction {
    pub version: u32,
    // By default, the `Vec` is serialized into `fixvec`.
    // Every element in `Vec` should be fixed size(molecule struct, array or primitive types)
    pub cell_deps: Vec<CellDep>,
    pub header_deps: Vec<[u8; 32]>,
    pub inputs: Vec<CellInput>,
    // By default, the `Vec` is serialized into `fixvec`.
    // If element in `Vec` isn't fixed size, it should be annotated as "dynvec"
    #[serde(with = "dynvec_serde")]
    pub outputs: Vec<CellOutput>,
    #[serde(with = "dynvec_serde")]
    pub outputs_data: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Transaction {
    pub raw: RawTransaction,
    #[serde(with = "dynvec_serde")]
    pub witnesses: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct RawHeader {
    pub version: u32,
    pub compact_target: u32,
    pub timestamp: u64,
    pub number: u64,
    pub epoch: u64,
    pub parent_hash: [u8; 32],
    pub transactions_root: [u8; 32],
    pub proposals_hash: [u8; 32],
    pub extra_hash: [u8; 32],
    pub dao: [u8; 32],
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Header {
    #[serde(with = "struct_serde")]
    pub raw: RawHeader,
    pub nonce: u128,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct UncleBlock {
    pub header: Header,
    pub proposals: Vec<ProposalShortId>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Block {
    #[serde(with = "struct_serde")]
    pub header: Header,
    #[serde(with = "dynvec_serde")]
    pub uncles: Vec<UncleBlock>,
    #[serde(with = "dynvec_serde")]
    pub transactions: Vec<Transaction>,
    pub proposals: Vec<ProposalShortId>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct BlockV1 {
    #[serde(with = "struct_serde")]
    pub header: Header,
    #[serde(with = "dynvec_serde")]
    pub uncles: Vec<UncleBlock>,
    #[serde(with = "dynvec_serde")]
    pub transactions: Vec<Transaction>,
    pub proposals: Vec<ProposalShortId>,
    pub extension: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct CellbaseWitness {
    pub lock: Script,
    pub message: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct WitnessArgs {
    pub lock: Option<Vec<u8>>,
    pub input_type: Option<Vec<u8>>,
    pub output_type: Option<Vec<u8>>,
}
