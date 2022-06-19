#![feature(const_option)]

use num_bigint::{BigInt, BigUint};

// static MAX_BIGUINT: BigUint = BigUint::from_bytes_be(&vec![0xFFu8;32][..]);

pub(crate) const MAX_BIGUINT: &[u8; 64] = b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";

pub(crate) const N: &[u8; 64] = b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

pub(crate) fn to_bigint(num: &[u8; 64]) -> BigInt {
    BigInt::parse_bytes(num, 16).unwrap()
}
