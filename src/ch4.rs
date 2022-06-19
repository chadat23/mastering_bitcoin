// page 83 for interacting with bitcoin-cli over the internet
// use modulo::mod;

use std::str::FromStr;

use num_bigint::{BigInt, BigUint, RandBigInt};
// use num_traits::{Zero, One};
// use rand::prelude::Distribution;
// use rand::{thread_rng, Rng};
// use rand::distributions::Uniform;

/// Returns the positive module
fn positive_mod_i32(v: i32, p: i32) -> i32 {
    ((v % p) + p) % p
}

fn make_private_key(n: &BigUint) -> BigUint {
    // Or sha256 ... something
    let mut ran = rand::thread_rng();

    let mut private_key = ran.gen_biguint(256);
    
    while private_key > *n {
        private_key = ran.gen_biguint(256);
    }
    private_key
}

/// Calculates the powers of two that sum to the input number.
/// Returns a Vec where each cell contains a u8 representing
/// a power of 2 that must be summed to get the number.
/// 157 = 2^0 + 2^2 + 2^3 + 2^4 + 2^7
/// so
/// ```
/// let a = powers_of_two(157);
/// assert_eq!(a, vec![0, 2, 3, 4, 7]);
/// ```
fn powers_of_two(mut n: u8) -> Vec<u8> {
    let mut output = Vec::new();
    let mut idx = 0u8;
    while n > 0 {
        if n & 1 == 1 {
            output.push(idx);
        }
        n >>= 1;
        idx += 1;
    }
    output
}

// rust-midinverse
/// Performs the euclidean algerythom an a curve and point
fn extended_euclidean_algeorithm(a: i32, p: i32, xa: i32, ya: i32, xp: i32, yp: i32, target: i32) -> (i32, i32) {
    // https://youtu.be/IwRtISxAHY4
    
    let a = positive_mod_i32(a, p);
    let d = p / a;
    let r = p % a;
    let xr = xp - d * xa;
    let yr = yp - d * ya;
    if r == target {
        (xr as i32, yr as i32)
    } else {
        extended_euclidean_algeorithm(r, a, xr, yr, xa, ya, target)
    }
}

// Calculates the modulo inverse of a number with a given p
fn modulo_inverse(a: i32, p: i32) -> i32 {
    if a == 1 {
        return 1
    }
    let (_, y) = extended_euclidean_algeorithm(a, p, 0, 1, 1, 0, 1);
    positive_mod_i32(y, p)
}

fn modulo_slope_dy(dy: i32, p: i32) -> i32 {
    let (_, y) = extended_euclidean_algeorithm(dy, p, 0, 1, 1, 0, 0);
    positive_mod_i32(y, p)
}

/// Multiplies a point on a euclidean curve by 2^power.
fn curve_power_of_two(p: u8, a: u8, b: u8, gx: u8, gy: u8, n: u8, power: u8) -> (u8, u8) {
    // https://youtu.be/F3zzNa42-tQ

    let p = p as i32;
    let a = a as i32;
    let mut gx = gx as i32;
    let mut gy = gy as i32;
    let power = power as i32;

    for _ in 0..power {
        let s_num = (3 * gx.pow(2) + a) % p;
        let s_denom = modulo_inverse(2 * gy, p) as i32;

        let s = positive_mod_i32(s_num * s_denom, p);
        let gxi = positive_mod_i32(s.pow(2) - 2 * gx, p);
        gy = p - positive_mod_i32(gy + s * (gxi - gx), p);
        gx = gxi;
    }

    (gx as u8, gy as u8)
}

/// Adds two numbers on a euclidean curve.
fn curve_addition(x0: u8, y0: u8, x1: u8, y1: u8, p: u8) -> (u8, u8) {
    // https://www.graui.de/code/elliptic2/elliptic.js
    // https://andrea.corbellini.name/ecc/interactive/modk-add.html
    // https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/
    // https://andrea.corbellini.name/2015/05/23/elliptic-curve-cryptography-finite-fields-and-discrete-logarithms/#algebraic-sum

    let x0 = x0 as i32;
    let y0 = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;
    let p = p as i32;

    let s = positive_mod_i32((y1 - y0) * modulo_inverse(x1 - x0, p), p);

    let x2 = positive_mod_i32(s.pow(2) - (x1 + x0), p);
    let y2 = p - positive_mod_i32(y1 + s * (x2 - x1), p);

    (x2 as u8, y2 as u8)
}

/// Multiplyies a point a euclidean curve by a number.
fn curve_multiplication(p: u8, a: u8, b: u8, gx: u8, gy: u8, n: u8, multiplyier: u8, ) -> (u8, u8) {
    // https://andrea.corbellini.name/ecc/interactive/modk-mul.html
    let mut powers = powers_of_two(multiplyier);
    println!("******************** {:?}", &powers);
    let power = powers.pop().unwrap();
    let mut x_y_total = curve_power_of_two(p, a, b, gx, gy, n, power);
    for power in powers {
        let (dx, dy) = curve_power_of_two(p, a, b, gx, gy, n, power);
        x_y_total = curve_addition(dx, dy, x_y_total.0, x_y_total.1, p);
    }
    x_y_total
}

pub fn ch4() {
    // https://youtu.be/F3zzNa42-tQ

    let two = BigUint::from(2u32);
    let max = BigUint::from_bytes_be(&vec![0xFFu8;32][..]);
    println!("Max 256 bit number:  {:?}", max);
    let n = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();
    println!("Max n (private key): {:?}", n);
    
    // the order the the modulo of the field of the eliptic curve
    let p = two.pow(256) 
        - two.pow(32) 
        - two.pow(9) 
        - two.pow(8) 
        - two.pow(7)
        - two.pow(6)
        - two.pow(4)
        - BigUint::from(1u8);
    println!("p: curve order:      {:?}", p);

    let a = 1i32;
    let b = 7i32;

    let private_key = make_private_key(&n);
    println!("private key:         {}", private_key);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_powers_of_two() {
        let actual = powers_of_two(37);
        let expected = Vec::from([0, 2, 5]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_addition() {
        let actual = curve_addition(16, 20, 41, 120, 127);
        let expected = (86, 81);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_addition_2() {
        let actual = curve_addition(2, 7, 1, 5, 17);
        let expected = (1, 12);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_addition_3    () {
        let actual = curve_addition(3, 0, 1, 5, 17);
        let expected = (15, 13);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_addition_4    () {
        let actual = curve_addition(5, 8, 1, 5, 17);
        let expected = (2, 7);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_power_of_two_0() {
        let actual = curve_power_of_two(17, 0, 7, 1, 5, 1, 0);
        let expected = (1, 5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_power_of_two_1() {
        let actual = curve_power_of_two(17, 0, 7, 1, 5, 1, 1);
        let expected = (2, 10);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_power_of_two_2() {
        let actual = curve_power_of_two(17, 0, 7, 1, 5, 1, 2);
        let expected = (12, 1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_multiplication_1() {
        let actual = curve_multiplication(17, 0, 7, 1, 5, 1, 1);
        let expected = (1, 5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_multiplication_2() {
        let actual = curve_multiplication(17, 0, 7, 1, 5, 1, 2);
        let expected = (2, 10);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_multiplication_21_basically_3() {
        let actual = curve_multiplication(17, 0, 7, 1, 5, 1, 3);
        let expected = (5, 9);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_curve_multiplication_40_basically_4() {
        let actual = curve_multiplication(17, 0, 7, 1, 5, 1, 40);
        let expected = (12, 1);
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn test_greatest_common_denominator() {
    //     let actual = greatest_common_devisor(10, 45);
    //     let expected = 5;
    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn test_greatest_common_denominator_1() {
    //     let actual = greatest_common_devisor(26, 3);
    //     let expected = 1;
    //     assert_eq!(actual, expected);
    // }


    // https://www.techiedelight.com/find-general-solution-to-given-linear-congruence-equation/
    #[test]
    fn test_modulo_inverse3() {
        let actual = modulo_inverse(3, 26);
        let expected = 9;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_modulo_inverse2() {
        let actual = modulo_inverse(7, 19);
        let expected = 11;
        assert_eq!(actual, expected);
    }
}
