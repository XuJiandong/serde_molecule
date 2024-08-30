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
use serde::{Serialize, Deserialize};
use serde_molecule::{to_vec, from_slice};

#[derive(Serialize, Deserialize)]
pub struct Table1 {
    pub f1: u8,
    pub f2: u16,
}

fn main() {
    let t1 = Table1{f1: 0, f2: 0};
    // serialize
    let bytes = to_vec(&t1, false).unwrap();
    // deserialize
    let t2: Table1 = from_slice(&bytes, false).unwrap();
    assert_eq!(t1.f1, t2.f1);
}
```
First step is to add dependency in Cargo.toml:
```toml
serde = { version = "???", features = ["derive"] }
serde_molecule = { version = "???" }
```
Then to annotate the types with `#[derive(Serialize, Deserialize)]`. After that,
use `to_vec` or `from_slice` to serialize/deserialize. 

## Types mapping

Rust types are mapping to molecule types, according to the [RFC](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md):

| Rust Type | Molecule Type | Fixed Size |
| --------- | ------------- | ---------- |
| i8,u8     | byte          | yes |
| i16, u16, i32, u32, i64, u64, i128, u128 | array | yes |
| f32       | array([byte; 4]) | yes |
| f64       | array([byte; 8]) | yes |
| [u8; N]   | array([byte; N]) | yes |
| [T; N]    | array([T; N]) | yes |
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

```rust,ignore
use serde_molecule::dynvec_serde;
#[derive(Serialize, Deserialize)]
struct RawTransaction {
    // ...
    #[serde(with = "dynvec_serde")]
    pub outputs: Vec<CellOutput>,
}
```

By default, every field is considered as molecule `table`. If it is molecule
`struct`, we should annotate it explicitly.
```rust,ignore
use serde_molecule::struct_serde;

#[derive(Serialize, Deserialize)]
pub struct CellInput {
    pub since: u64,
    #[serde(with = "struct_serde")]
    pub previous_output: OutPoint,
}
```
If the top-level type is a molecule `struct`, the second argument to `to_vec` or
`from_slice` should be `true`. If the value is `false`, the top-level type is
considered a molecule `table`.

For all Molecule `struct`s, their inner and descendant fields should be "fixed
size" (see the table above).

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
For molecule union with customized id, see [example](https://github.com/XuJiandong/serde_molecule/tree/main/examples/serde_molecule_customized_union_id).

## no_std support

To use this library in a `no_std` environment:

1. Disable default features for both `serde_molecule` and `serde`
2. Enable the `alloc` feature for `serde_molecule`
3. Enable the `derive` feature for `serde`

Add the following to your `Cargo.toml`:

```toml
serde_molecule = { version = "x.x.x", default-features = false, features = ["alloc"] }
serde = { version = "x.x.x", default-features = false, features = ["derive"] }
```

See the [no_std example](https://github.com/XuJiandong/serde_molecule/tree/main/examples/serde_molecule_nostd) for more details.

## Big Array Support

The Serde framework doesn't support arrays with element sizes greater than 32.
See [this solution](https://github.com/est31/serde-big-array). This limitation
can be addressed using a new `serde with` annotation (`big_array_serde`):

```rust
use serde::{Deserialize, Serialize};
use serde_molecule::big_array_serde;

#[derive(Serialize, Deserialize)]
struct BigArray {
    f1: u8,
    #[serde(with = "big_array_serde")]
    f2: [u8; 33],
    #[serde(with = "big_array_serde")]
    f3: [u8; 64],
}
```

## Drawback of Deserialization

Compared to the [Rust version of the Molecule
implementation](https://github.com/nervosnetwork/molecule), deserialization with
`serde_molecule` consumes at least double the memory. In memory-limited
scenarios, such as on-chain scripts, it's not recommended to use.


## Example
Here is an example definition of [CKB types](https://github.com/XuJiandong/serde_molecule/tree/main/tests/src/ckb_types.rs).