use crate::ckb_types::CellDep;
use crate::ckb_types::CellInput;
use crate::ckb_types::CellOutput;
use crate::ckb_types::OutPoint;
use crate::ckb_types::Script;
use crate::ckb_types::Transaction;
use crate::ckb_types::WitnessArgs;
use crate::test_once;
use ckb_gen_types::packed;
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
    test_once(&new_tx);
}

#[test]
fn test_simple_transaction() {
    let mut args = vec![0u8; 20];
    args[0] = 100;
    args[1] = 101;
    args[2] = 102;

    // old style building a transaction
    let old_tx = packed::Transaction::default();
    let old_script = packed::Script::default();
    let old_cell_dep = packed::CellDep::default();

    let old_script = old_script.as_builder().args(args.clone().pack()).build();
    let cell_output = packed::CellOutput::default()
        .as_builder()
        .lock(old_script)
        .capacity(42u64.pack())
        .build();
    let cell_outputs = packed::CellOutputVec::default()
        .as_builder()
        .push(cell_output)
        .build();
    let cell_deps = packed::CellDepVec::default()
        .as_builder()
        .push(old_cell_dep)
        .build();
    let raw = old_tx
        .raw()
        .as_builder()
        .cell_deps(cell_deps)
        .outputs(cell_outputs)
        .build();
    let old_tx = old_tx.as_builder().raw(raw).build();

    // new style building a transaction
    let mut new_tx = Transaction::default();
    let mut new_script = Script::default();
    let new_cell_dep = CellDep::default();

    new_script.args = args.clone();
    new_tx.raw.outputs.push(CellOutput {
        capacity: 42,
        lock: new_script,
        type_: None,
    });
    new_tx.raw.cell_deps = vec![new_cell_dep];

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

    test_once(&new_witnessargs);
}

#[test]
fn test_variable_tx() {
    let mut tx = Transaction::default();
    test_once(&tx);

    // Script and CellOutput
    let mut script = Script::default();
    script.args = vec![1, 2, 3];
    script.code_hash = [5u8; 32];
    script.hash_type = 1;
    let cell_output = CellOutput {
        capacity: 42,
        lock: script.clone(),
        type_: Some(script.clone()),
    };

    // OutPoint
    let out_point = OutPoint {
        tx_hash: [1u8; 32],
        index: 42,
    };

    // Test with one output
    tx.raw.outputs.push(cell_output.clone());
    test_once(&tx);

    // Test with multiple outputs
    tx.raw.outputs.push(cell_output.clone());
    tx.raw.outputs.push(CellOutput {
        capacity: 100,
        lock: script.clone(),
        type_: None,
    });
    test_once(&tx);

    // Test with empty witnesses
    tx.witnesses = vec![vec![]];
    test_once(&tx);

    // Test with various witnesses
    tx.witnesses = vec![vec![], vec![1, 2, 3], vec![], vec![4, 5, 6, 7]];
    test_once(&tx);

    // Test with cell deps
    let cell_dep = CellDep {
        out_point: out_point.clone(),
        dep_type: 1,
    };
    tx.raw.cell_deps = vec![cell_dep.clone(), cell_dep.clone()];
    test_once(&tx);

    // Test with inputs
    let cell_input = CellInput {
        since: 42,
        previous_output: out_point.clone(),
    };
    tx.raw.inputs = vec![cell_input.clone(), cell_input.clone()];
    test_once(&tx);

    // Test with more complex scenario
    tx.raw.version = 2;
    tx.raw.cell_deps.push(CellDep {
        out_point: OutPoint {
            tx_hash: [2u8; 32],
            index: 0,
        },
        dep_type: 0,
    });
    tx.raw.inputs.push(CellInput {
        since: 100,
        previous_output: OutPoint {
            tx_hash: [3u8; 32],
            index: 1,
        },
    });
    tx.raw.outputs.push(CellOutput {
        capacity: 200,
        lock: Script {
            code_hash: [4u8; 32],
            hash_type: 2,
            args: vec![7, 8, 9],
        },
        type_: None,
    });
    tx.witnesses.push(vec![10, 11, 12]);
    test_once(&tx);

    // Test with maximum values
    tx.raw.version = u32::MAX;
    tx.raw.cell_deps[0].dep_type = u8::MAX;
    tx.raw.inputs[0].since = u64::MAX;
    tx.raw.outputs[0].capacity = u64::MAX;
    test_once(&tx);

    // Test with empty vectors
    tx.raw.cell_deps.clear();
    tx.raw.inputs.clear();
    tx.raw.outputs.clear();
    tx.witnesses.clear();
    test_once(&tx);
}
