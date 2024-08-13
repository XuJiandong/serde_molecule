#![allow(clippy::all)]

use hex;
pub mod new;
pub mod old;
pub mod simple;

pub fn assert_eq_slice(a: &[u8], b: &[u8]) {
    let hex_a = hex::encode(a);
    let hex_b = hex::encode(b);
    assert_eq!(hex_a, hex_b);
}
