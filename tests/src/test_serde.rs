use crate::test_once;
use serde::{Deserialize, Serialize};
use serde_molecule::{dynvec_serde, from_slice, struct_serde, to_vec};
use std::collections::{BTreeMap, LinkedList};

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
struct StructInner {
    f0: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
struct Struct0 {
    f0: u8,
    f1: u64,
    f2: [u8; 3],
    #[serde(with = "struct_serde")]
    f3: StructInner,
    f4: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum Union0 {
    A(u32),
    B(String),
    C([u8; 3]),
}

impl Default for Union0 {
    fn default() -> Self {
        Union0::A(100)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
struct Struct1 {
    pub f1: u8,
    pub f2: u16,
    pub f3: [u8; 3],
    pub f4: [[u8; 5]; 2],
    pub f5: Vec<u8>,
    pub f6: String,
    pub f7: Option<u32>,
    #[serde(with = "dynvec_serde")]
    pub f8: Vec<Vec<u8>>,
    pub f9: LinkedList<[u8; 3]>,
    #[serde(with = "struct_serde")]
    pub f10: Struct0,
    pub f11: BTreeMap<u32, String>,
    pub f12: Union0,
}

#[test]
fn test_serde_1() {
    let mut value = Struct1::default();

    // Test case 1: Basic initialization
    value.f1 = 100;
    value.f2 = 200;
    value.f3 = [1, 2, 3];
    value.f4 = [[1, 2, 3, 4, 5], [1, 2, 3, 4, 5]];
    value.f5 = vec![1, 2, 3];
    value.f6 = String::from("hello");
    value.f7 = Some(42);
    value.f8 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![]];
    value.f9 = LinkedList::from([[1, 2, 3], [4, 5, 6]]);
    value.f10.f0 = 1;
    value.f10.f1 = 1000;
    value.f10.f2 = [10, 20, 30];
    value.f10.f3.f0 = 12345;
    value.f10.f4 = 54321;
    value.f11.insert(1, "hi".into());
    value.f11.insert(2, "hi2".into());
    value.f11.insert(100, "hi100".into());
    value.f12 = Union0::B("hello".into());
    test_once(&value);

    // Test case 2: Modifying Union0
    value.f12 = Union0::A(42);
    test_once(&value);
    value.f12 = Union0::C([1, 2, 3]);
    test_once(&value);

    // Test case 3: Empty and nested vectors
    value.f8 = vec![];
    test_once(&value);
    value.f8 = vec![vec![]];
    test_once(&value);
    value.f8 = vec![vec![], vec![1, 2, 3]];
    test_once(&value);
    value.f8 = vec![vec![1], vec![1, 2], vec![1, 2, 3]];
    test_once(&value);

    // Test case 4: Modifying Option<u32>
    value.f7 = None;
    test_once(&value);
    value.f7 = Some(0);
    test_once(&value);

    // Test case 5: Complex LinkedList
    value.f9 = LinkedList::new();
    test_once(&value);
    value.f9.push_back([0, 0, 0]);
    value.f9.push_back([255, 255, 255]);
    test_once(&value);

    // Test case 6: Complex BTreeMap
    value.f11.clear();
    test_once(&value);
    value.f11.insert(0, "".into());
    value.f11.insert(u32::MAX, "max".into());
    test_once(&value);

    // Test case 7: Edge cases for numeric fields
    value.f1 = u8::MAX;
    value.f2 = u16::MAX;
    value.f10.f0 = u8::MIN;
    value.f10.f1 = u64::MAX;
    value.f10.f4 = u32::MIN;
    test_once(&value);

    // Test case 8: Long string
    value.f6 = "a".repeat(1000);
    test_once(&value);

    // Test case 9: Mixed modifications
    value.f3 = [255, 0, 255];
    value.f4 = [[0, 1, 2, 3, 4], [5, 6, 7, 8, 9]];
    value.f5 = vec![100, 101, 102, 103, 104];
    value.f7 = Some(999999);
    value.f8 = vec![vec![255; 10], vec![128; 5], vec![0; 3]];
    value.f12 = Union0::B("Mixed test case".into());
    test_once(&value);
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum Union1 {
    A,
    B,
    C,
}
#[test]
fn test_unit_variant() {
    let u = Union1::C;
    let bytes = to_vec(&u, false).unwrap();
    assert_eq!(bytes, vec![2, 0, 0, 0]);
    let u2: Union1 = from_slice(&bytes, false).unwrap();
    assert_eq!(u, u2);
}

#[derive(Serialize, Deserialize, PartialEq)]
struct SkipField {
    f1: u8,
    #[serde(skip)]
    ignore: u8,
    f2: u32,
}

#[test]
fn test_skip_field() {
    let s = SkipField {
        f1: 1,
        ignore: 2,
        f2: 3,
    };
    let bytes = to_vec(&s, false).unwrap();
    assert_eq!(
        bytes,
        vec![17, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 1, 3, 0, 0, 0]
    );
    let s2: SkipField = from_slice(&bytes, false).unwrap();
    assert_eq!(s.f1, s2.f1);
    assert_eq!(s.f2, s2.f2);
    // the ignored value is default one
    assert_eq!(s.ignore, 2);
    assert_eq!(s2.ignore, 0);
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
struct Struct2 {
    #[serde(with = "struct_serde")]
    pub s0: Struct0,
    pub f1: u32,
}

#[test]
fn test_nested_struct() {
    let s2 = Struct2 {
        s0: Struct0 {
            f0: 0x12,
            f1: 0x34,
            f2: [5, 6, 7],
            f3: StructInner { f0: 0x87654321 },
            f4: 0x1234,
        },
        f1: 0x12345678,
    };
    let bytes = to_vec(&s2, false).unwrap();
    let value2: Struct2 = from_slice(&bytes, false).unwrap();
    assert_eq!(value2.s0.f4, s2.s0.f4);
    assert_eq!(value2.s0.f4, 0x1234);
    assert_eq!(value2.f1, 0x12345678);
}

#[derive(Serialize, Deserialize)]
enum UnionWithDynvec {
    #[serde(with = "dynvec_serde")]
    DynvecVariant(Vec<String>),
    VariantA(u8),
}

#[test]
fn test_union_with_dynvec() {
    let vs: Vec<String> = vec!["hello".into(), "world".into(), "".into()];
    let variant = UnionWithDynvec::DynvecVariant(vs);
    test_once(&variant);
}

#[derive(Serialize)]
struct StructWithDynVec {
    #[serde(serialize_with = "dynvec_serde::serialize")]
    pub v: Vec<Vec<u8>>,
}

#[test]
fn test_result() {
    let mut value = Struct1::default();
    value.f1 = 100;
    value.f2 = 200;
    value.f3 = [1, 2, 3];
    value.f4 = [[1, 2, 3, 4, 5], [1, 2, 3, 4, 5]];

    let result: Result<Struct1, ()> = Ok(value);
    test_once(&result);
}

#[derive(Serialize, Deserialize)]
enum StructVariant {
    Method1 { arg1: u64, arg2: String },
    Method2 { arg1: u8, arg2: u16 },
}

#[test]
fn test_struct_variant() {
    let var = StructVariant::Method1 {
        arg1: 123,
        arg2: "hello".into(),
    };
    test_once(&var);
}

#[derive(Serialize, Deserialize)]
struct TupleStruct(u8, u16, String);

#[derive(Serialize, Deserialize)]
struct NewType(String);

#[test]
fn test_tuple_struct() {
    let ts = TupleStruct(1, 2, "hello".into());
    test_once(&ts);
    let nt = NewType("hello".into());
    test_once(&nt);
}

#[derive(Serialize, Deserialize)]
enum TupleVariant {
    TupleVariant1(u8, u16, String),
    TupleVariant2(u32, u64),
}

#[test]
fn test_tuple_variant() {
    let tv = TupleVariant::TupleVariant1(1, 2, "hello".into());
    test_once(&tv);
}

#[test]
fn test_unit() {
    let u = ();
    test_once(&u);
    let res: Result<(), String> = Ok(());
    test_once(&res);
    let res2: Result<(), String> = Err("error".into());
    test_once(&res2);
}

#[test]
fn test_char() {
    let c = 'a';
    test_once(&c);
}
