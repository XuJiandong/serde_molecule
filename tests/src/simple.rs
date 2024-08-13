#[cfg(test)]
use crate::new::test1::Table1 as NewTable1;
#[cfg(test)]
use crate::new::test1_default as new_default;
#[cfg(test)]
use crate::old::test1::Table1 as OldTable1;
#[cfg(test)]
use crate::old::test1_default as old_default;
#[cfg(test)]
use molecule::prelude::*;
#[cfg(test)]
use serde_molecule::to_vec;

#[test]
fn test_struct1() {
    let old_value = (*old_default::DEFAULT_STRUCT1).clone();
    let new_value = (*new_default::DEFAULT_STRUCT1).clone();
    let old = old_value.as_slice();
    let new = to_vec(&new_value, true).unwrap();
    assert_eq!(old, &new);
}

#[test]
fn test_table1() {
    let old_value = OldTable1::new_builder()
        .f1((*old_default::DEFAULT_BYTE).clone())
        .f2((*old_default::DEFAULT_U16).clone())
        .f3((*old_default::DEFAULT_U32).clone())
        .f4((*old_default::DEFAULT_U64).clone())
        .f5((*old_default::DEFAULT_U128).clone())
        .fixvec((*old_default::DEFAULT_FIXVEC).clone())
        .dynvec((*old_default::DEFAULT_DYNVEC).clone())
        .struct1((*old_default::DEFAULT_STRUCT1).clone())
        .option(Default::default())
        .array3((*old_default::DEFAULT_ARRAY3).clone())
        .build();
    let new_value = NewTable1 {
        f1: new_default::DEFAULT_BYTE,
        f2: new_default::DEFAULT_U16,
        f3: new_default::DEFAULT_U32,
        f4: new_default::DEFAULT_U64,
        f5: new_default::DEFAULT_U128,
        fixvec: (*new_default::DEFAULT_FIXVEC).clone(),
        dynvec: (*new_default::DEFAULT_DYNVEC).clone(),
        struct1: (*new_default::DEFAULT_STRUCT1).clone(),
        option: None,
        array3: (*new_default::DEFAULT_ARRAY3).clone(),
    };
    let old = old_value.as_slice();
    let new = to_vec(&new_value, false).unwrap();
    assert_eq!(old, new.as_slice());
}
