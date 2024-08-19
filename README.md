## Introduction
The [molecule](https://github.com/nervosnetwork/molecule) is a serialization
format used on CKB. It has several implementations in different languages (C,
Rust, JavaScript). In Rust, the implementation introduces numerous types and
functions, making it difficult to remember. However, with the power of the
[serde](https://github.com/serde-rs/serde) framework, this process can be
greatly simplified. This project offers an implementation of Molecule using
Serde.

## How to use
Here is a simple case about how to use:
```rust
use serde::Serialize;
use serde_molecule::to_vec;

#[derive(Serialize)]
pub struct Table1 {
    pub f1: u8,
    pub f2: u16,
}

fn main() {
    let s = Table1::new();
    let data = to_vec(&s, false).unwrap();
}
```

The first step is to annotate the types with `#[derive(Serialize)]`. After that,
use `to_vec` to serialize. 

## Types mapping

Rust types are mapping to molecule types, according to the [RFC](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md):

| Rust Type | Molecule Type | Fixed Size |
| --------- | ------------- | ---------- |
| i8,u8     | byte          | yes |
| i16, u16, i32, u32, i64, u64, i128, u128 | array | yes |
| [u8; N]   | array | yes |
| [T; N]    | array | yes |
| Vec<T>    | fixvec | no |
| struct     | table | no |
| #[serde(with = "struct_serde")] | struct | yes |
| #[serde(with = "dynvec_serde")] | dynvec | no |
| Option<T>  | option | no |
| enum       | union | no |
| String     | fixvec | no |
| BTreeMap   | dynvec | no |
| HashMap    | dynvec | no |
| BinaryHeap | fixvec | no |
| LinkedList | fixvec | no |
| VecDeque   | fixvec | no |
| HashSet    | fixvec | no |

By default, `Vec`-like containers (such as `Vec`, `BinaryHeap`, etc.) are
serialized into `fixvec`. Every element in the `Vec` must have a fixed size
(like a molecule struct, array, or primitive type). If the element is not of a
fixed size, it should be annotated with `#[serde(with = "dynvec_serde")]`:

```rust
use serde_molecule::dynvec_serde;
#[derive(Serialize)]
struct RawTransaction {
    // ...
    #[serde(with = "dynvec_serde")]
    pub outputs: Vec<CellOutput>,
}
```

By default, every field is considered as molecule table. If it is molecule
struct, we should annotate it explicitly.
```rust
use serde_molecule::struct_serde;

#[derive(Serialize)]
pub struct CellInput {
    pub since: u64,
    #[serde(with = "struct_serde")]
    pub previous_output: OutPoint,
}
```

If the top-level type is a molecule struct, the second argument to `to_vec`
should be `true`. If the value is `false`, the top-level type is considered a
molecule table.


## Map
The Rust map types (like `BTreeMap` and `HashMap`) can be mapped to the following Molecule schemas:
```text
table MapEntry {
    key: KEY_TYPE,
    value: VALUE_TYPE,
}

vector Map <MapEntry>;
```
It is not recommended to use HashMap because its key-value pairs are stored in
arbitrary order.

## Union with customized id
For molecule union with customized id, see [example with EnumCustomizedId](./tests/src/new/test1.rs).

## Example
Here is an example of [CKB types](./tests/src/ckb_types.rs)
