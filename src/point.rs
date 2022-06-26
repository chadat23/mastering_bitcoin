use std::cmp::Eq;
use std::ops::{Add, Mul};

use num_bigint::{BigInt, BigUint, ToBigInt};

use crate::utils;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Point {
    x: BigInt,
    y: BigInt,
}

impl Point {
    pub(crate) fn add(self, other: Self, p: &BigInt, a: &BigInt) -> Self {
        // let this = &self;
        // let other = &other;

        // let zero = BigInt::from(0);
        // if self.x < zero {
        //     return other
        // }
        // if other.x < zero {
        //     return self
        // }
        let s = if self.x == other.x {
            if self.y != other.y {
                return Self {
                    x: BigInt::from(-1),
                    y: BigInt::from(-1),
                }
            }
            let dy = positive_mod(&(3 * self.x.pow(2) + a), &p);
            let dx = modulo_inverse(&(BigInt::from(2) * self.y()), &p);
            dy * dx
        } else {
            positive_mod(&((other.y() - self.y()) * modulo_inverse(&(other.x() - self.x()), &p)), &p)
        };
        let x = positive_mod(&(s.pow(2) - (self.x() + other.x())), &p);
        let y = p - positive_mod(&(self.y() + s * (x.clone() - self.x())), &p);

        Self {
            x,
            y,
        }
    }

    pub(crate) fn multiply(self, rhs: &BigInt, p: &BigInt, a: &BigInt) -> Self {
        let powers = powers_of_two(rhs);
        let power = powers[0];
        let mut last_power = power;
        let mut point = self.double_n_times(power, &p, &a);
        let mut point_total = point.clone();
        for power in &powers[1..] {
            point = point.clone().double_n_times(power - last_power, &p, &a);
            point_total = point_total.add(point.clone(), &p, &a);
            last_power = *power;
        }
        point_total
    }

    pub(crate) fn from_xy(x: BigInt, y: BigInt) -> Self {
        Self {
            x,
            y,
        }
    }

    fn x(&self) -> BigInt {
        self.x.clone()
    }

    fn y(&self) -> BigInt {
        self.y.clone()
    }

    fn double_n_times(self, n: u16, p: &BigInt, a: &BigInt) -> Self {
        let mut point = self.clone();
        for _ in 0..n {
            point = point.clone().add(point, &p, &a)
        }
        point
    }

    pub fn x_to_hex_string(&self) -> String {
        format!("{:0>64}", self.x.to_str_radix(16).to_uppercase())
    }

    pub fn y_to_hex_string(&self) -> String {
        format!("{:0>64}", self.y.to_str_radix(16).to_uppercase())
    }

    pub fn x_bytes(&self) -> Vec<u8> {
        self.x.to_biguint().unwrap().to_bytes_be()
    }

    pub fn y_bytes(&self) -> Vec<u8> {
        self.y.to_biguint().unwrap().to_bytes_be()
    }

    pub fn y_is_even(&self) -> bool {
        self.y.clone() % BigInt::from(2) == BigInt::from(0)
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self { 
            x: self.x.clone(), 
            y: self.y.clone(), 
        }
    }
}

pub(crate) fn positive_mod(num: &BigInt, p: &BigInt) -> BigInt {
    ((num % p) + p) % p
}

// rust-midinverse
/// Performs the euclidean algerythom an a curve and point
fn extended_euclidean_algeorithm(num: &BigInt, p: &BigInt, i_num: &BigInt, j_num: &BigInt, i_p: &BigInt, j_p: &BigInt, target: &BigInt) -> (BigInt, BigInt) {
    // https://youtu.be/IwRtISxAHY4
    
    let num = positive_mod(&num, &p);
    let multiples = p / num.clone();
    let remainder = p % num.clone();
    let i_remainder = i_p - multiples.clone() * i_num;
    let j_remainder = j_p - multiples * j_num;
    if remainder == *target {        
        (i_remainder, j_remainder)
    } else {
        extended_euclidean_algeorithm(&remainder, &num, &i_remainder, &j_remainder, &i_num, &j_num, &target)
    }
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
fn powers_of_two(n: &BigInt) -> Vec<u16> {
    let mut n = n.to_biguint().unwrap();
    let mut output = Vec::new();
    let mut idx = 0u16;
    let zero = BigUint::from(0u8);
    let one = BigUint::from(1u8);
    while n > zero {
        if &n & &one == one {
            output.push(idx);
        }
        n >>= 1;
        idx += 1;
    }
    output
}

// Calculates the modulo inverse of a number with a given p
fn modulo_inverse(num: &BigInt, p: &BigInt) -> BigInt {
    if *num == BigInt::from(1) {
        return num.clone()
    }
    let (_, j) = extended_euclidean_algeorithm(&num, &p, &BigInt::from(0), &BigInt::from(1), &BigInt::from(1), &BigInt::from(0), &BigInt::from(1));
    positive_mod(&j, &p)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_point(x: u32, y: u32) -> Point {
        let x = BigInt::from(x);
        let y = BigInt::from(y);
        Point {
            x,
            y,
        }
    }

    #[test]
    fn test_point_addition_to_itself() {
        let point1 = test_point(1, 5);
        let point2 = test_point(1, 5);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point1.add(point2, &p, &a);

        let expected = test_point(2, 10);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_addition_other_modulo() {
        let point1 = test_point(1, 5);
        let point2 = test_point(2, 10);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point1.add(point2, &p, &a);

        let expected = test_point(5, 9);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_addition_other() {
        let point1 = test_point(1, 5);
        let point2 = test_point(2, 7);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point1.add(point2, &p, &a);

        let expected = test_point(1, 12);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_powers_of_two() {
        let actual = powers_of_two(&BigInt::from(37));
        let expected = Vec::from([0, 2, 5]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_double_n_times_0() {
        let point = test_point(1, 5);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);
        
        let actual = point.clone().double_n_times(0, &p, &a);

        let expected = point;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_double_n_times_1() {
        let point = test_point(1, 5);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);
        
        let actual = point.clone().double_n_times(1, &p, &a);

        let expected = test_point(2, 10);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_double_n_times_2() {
        let point = test_point(1, 5);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);
        
        let actual = point.clone().double_n_times(2, &p, &a);

        let expected = test_point(12, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_1() {
        let point = test_point(1, 5);
        let factor = BigInt::from(1);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point.multiply(&factor, &p, &a);

        let expected = test_point(1, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_1_rev() {
        let point = test_point(1, 5);
        let factor = BigInt::from(1);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point.multiply(&factor, &p, &a);

        let expected = test_point(1, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_2() {
        let point = test_point(1, 5);
        let factor = BigInt::from(2);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point.multiply(&factor, &p, &a);

        let expected = test_point(2, 10);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_2_rev() {
        let point = test_point(1, 5);
        let factor = BigInt::from(2);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point.multiply(&factor, &p, &a);

        let expected = test_point(2, 10);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_21() {
        let point = test_point(1, 5);
        let factor = BigInt::from(21);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point.multiply(&factor, &p, &a);

        let expected = test_point(5, 9);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_21_rev() {
        let point = test_point(1, 5);
        let factor = BigInt::from(21);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point.multiply(&factor, &p, &a);

        let expected = test_point(5, 9);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_40() {
        let point = test_point(1, 5);
        let factor = BigInt::from(40);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point.multiply(&factor, &p, &a);

        let expected = test_point(12, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_40_rev() {
        let point = test_point(1, 5);
        let factor = BigInt::from(40);
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);

        let actual = point.multiply(&factor, &p, &a);

        let expected = test_point(12, 1);

        assert_eq!(actual, expected);
    }

    // // #[test]
    // // fn test_greatest_common_denominator() {
    // //     let actual = greatest_common_devisor(10, 45);
    // //     let expected = 5;
    // //     assert_eq!(actual, expected);
    // // }

    // // #[test]
    // // fn test_greatest_common_denominator_1() {
    // //     let actual = greatest_common_devisor(26, 3);
    // //     let expected = 1;
    // //     assert_eq!(actual, expected);
    // // }


    // // https://www.techiedelight.com/find-general-solution-to-given-linear-congruence-equation/
    #[test]
    fn test_modulo_inverse3() {
        let actual = modulo_inverse(&BigInt::from(3), &BigInt::from(26));
        let expected = BigInt::from(9);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_modulo_inverse2() {
        let actual = modulo_inverse(&BigInt::from(7), &BigInt::from(19));
        let expected = BigInt::from(11);
        assert_eq!(actual, expected);
    }



    // #[test]
    // fn test_start_public_key_calculations() {
    //     let private_key = BigInt::parse_bytes(b"1E99423A4ED27608A15A2616A2B0E9E52CED330AC530EDCC32C8FFC6A526AEDD", 16).unwrap();

    //     let point = Point::generator_point();

    //     // K = kG
    //     // public_key = private_key * generater_point
    //     let public_key = point * private_key;

    //     let expected_x = "F028892BAD7ED57D2FB57BF33081D5CFCF6F9ED3D3D7F159C2E2FFF579DC341A".to_string();
    //     let expected_y = "07CF33DA18BD734C600B96A72BBC4749D5141C90EC8AC328AE52DDFE2E505BDB".to_string();

    //     assert_eq!(public_key.x_to_hex_string(), expected_x);
    //     assert_eq!(public_key.y_to_hex_string(), expected_y);
    // }

    // #[test]
    // fn test_lkj() {
    //     let point = Point::from_xy(BigInt::from(4), BigInt::from(5));
    //     dbg!("++++++++++++++++++++++++++++++++++++++++++++++", point.p);
    //     // assert!(false)
    // }
}
