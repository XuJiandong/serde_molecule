use crate::ckb_types::CellOutput;
use crate::ckb_types::Script;
use crate::ckb_types::Transaction;
use crate::ckb_types::WitnessArgs;
use ckb_gen_types::packed;
use ckb_gen_types::packed::CellOutputVec;
use ckb_gen_types::prelude::*;
use molecule::bytes;
use serde_molecule::to_vec;

#[test]
fn test_default_transaction() {
    let old_tx = packed::Transaction::default();
    let new_tx = Transaction::default();
    let old = old_tx.as_slice();
    let new = to_vec(&new_tx, false).unwrap();
    assert_eq!(old, new.as_slice());
}

#[test]
fn test_simple_transaction() {
    let args = vec![0u8; 20];

    // old style building a transaction
    let old_tx = packed::Transaction::default();
    let old_script = packed::Script::default();
    let old_script = old_script.as_builder().args(args.clone().pack()).build();
    let cell_output = packed::CellOutput::default()
        .as_builder()
        .lock(old_script)
        .build();
    let cell_outputs = CellOutputVec::default()
        .as_builder()
        .push(cell_output)
        .build();
    let raw = old_tx.raw().as_builder().outputs(cell_outputs).build();
    let old_tx = old_tx.as_builder().raw(raw).build();

    // new style building a transaction
    let mut new_tx = Transaction::default();
    let mut new_script = Script::default();
    new_script.args = args.clone();
    new_tx.raw.outputs.push(CellOutput {
        capacity: 0,
        lock: new_script,
        type_: None,
    });

    let old = old_tx.as_slice();
    let new = to_vec(&new_tx, false).unwrap();
    assert_eq!(old, new.as_slice());
}

#[test]
fn test_simple_witnessargs() {
    let lock = vec![0u8; 20];

    let old_witnessargs = packed::WitnessArgs::default();
    let lock_bytes: bytes::Bytes = lock.clone().into();
    let old_witnessargs = old_witnessargs
        .as_builder()
        .lock(Some(lock_bytes).pack())
        .build();

    let mut new_witnessargs = WitnessArgs::default();
    new_witnessargs.lock = Some(lock.clone());

    let old = old_witnessargs.as_slice();
    let new = to_vec(&new_witnessargs, false).unwrap();
    assert_eq!(old, new.as_slice());
}
