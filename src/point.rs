use std::cmp::Eq;
use std::ops::{Add, Mul};

use num_bigint::{BigInt, BigUint, ToBigInt};

use crate::utils;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Point {
    x: BigInt,
    y: BigInt,
    a: BigInt,
    b: BigInt,
    p: BigInt,
    n: BigInt,
}

impl Point {
    fn generator_point() -> Self {
        Point::from_xy(
            BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(), 
            BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap())
    }

    fn from_xy(x: BigInt, y: BigInt) -> Self {
        let two = BigInt::from(2);
        Self {
            x,
            y,
            a: BigInt::from(0u8),
            b: BigInt::from(7u8),
            p: two.pow(256) 
                - two.pow(32) 
                - two.pow(9) 
                - two.pow(8) 
                - two.pow(7)
                - two.pow(6)
                - two.pow(4)
                - BigInt::from(1u8),
            n: utils::to_bigint(utils::N)
        }
    }

    fn from_xyabcpn(x: BigInt, y: BigInt, a: BigInt, b: BigInt, p: BigInt, n: BigInt) -> Self {
        Self {
            x,
            y,
            a,
            b,
            p,
            n
        }
    }

    fn x(&self) -> BigInt {
        self.x.clone()
    }

    fn y(&self) -> BigInt {
        self.y.clone()
    }

    fn a(&self) -> BigInt {
        self.a.clone()
    }

    // fn b(&self) -> BigInt {
    //     self.b.clone()
    // }

    // fn p(&self) -> BigInt {
    //     self.p.clone()
    // }

    // fn n(&self) -> BigInt {
    //     self.n.clone()
    // }

    fn double_n_times(self, n: u16) -> Self {
        let mut point = self.clone();
        for _ in 0..n {
            point = point.clone() + point;
        }
        point
    }

    fn x_as_hex(&self) -> String {
        format!("{:0>64}", self.x.to_str_radix(16).to_uppercase())
    }

    fn y_as_hex(&self) -> String {
        format!("{:0>64}", self.y.to_str_radix(16).to_uppercase())
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self { 
            x: self.x.clone(), 
            y: self.y.clone(), 
            a: self.a.clone(), 
            b: self.b.clone(), 
            p: self.p.clone(), 
            n: self.n.clone() 
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
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
                    a: self.a(),
                    b: self.b.clone(),
                    p: self.p.clone(),
                    n: self.n.clone(),
                }
            }
            let dy = positive_mod(&(3 * self.x.pow(2) + self.a()), &self.p);
            let dx = modulo_inverse(&(BigInt::from(2) * self.y()), &self.p);
            dy * dx
        } else {
            positive_mod(&((other.y() - self.y()) * modulo_inverse(&(other.x() - self.x()), &self.p)), &self.p)
        };
        let x = positive_mod(&(s.pow(2) - (self.x() + other.x())), &self.p);
        let y = self.p.clone() - positive_mod(&(self.y() + s * (x.clone() - self.x())), &self.p);

        Self {
            x,
            y,
            a: self.a(),
            b: self.b.clone(),
            p: self.p.clone(),
            n: self.n.clone(),
        }
    }
}

impl Mul<BigInt> for Point {
    type Output = Self;

    fn mul(self, rhs: BigInt) -> Self {
        let powers = powers_of_two(rhs);
        let power = powers[0];
        let mut last_power = power;
        let mut point = self.clone().double_n_times(power);
        let mut point_total = point.clone();
        // let mut point_total = self.clone().double_n_times(power);
        for power in &powers[1..] {
            point = point.double_n_times(power - last_power);
            point_total = point_total + point.clone();
            last_power = *power;
        }
        point_total
    }
}

impl Mul<BigUint> for Point {
    type Output = Self;

    fn mul(self, rhs: BigUint) -> Self {
        self * rhs.to_bigint().unwrap()
    }
}

impl Mul<Point> for BigInt {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        rhs * self
    }
}

impl Mul<Point> for BigUint {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        rhs * self.to_bigint().unwrap()
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
fn powers_of_two(n: BigInt) -> Vec<u16> {
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
        let a = BigInt::from(0);
        let b = BigInt::from(7);
        let p = BigInt::from(17);
        let n = BigInt::from(18);
        Point {
            x,
            y,
            a,
            b,
            p,
            n,
        }
    }

    #[test]
    fn test_point_addition_to_itself() {
        let point1 = test_point(1, 5);
        let point2 = test_point(1, 5);

        let actual = point1 + point2;

        let expected = test_point(2, 10);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_addition_other_modulo() {
        let point1 = test_point(1, 5);
        let point2 = test_point(2, 10);

        let actual = point1 + point2;

        let expected = test_point(5, 9);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_addition_other() {
        let point1 = test_point(1, 5);
        let point2 = test_point(2, 7);

        let actual = point1 + point2;

        let expected = test_point(1, 12);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_powers_of_two() {
        let actual = powers_of_two(BigInt::from(37));
        let expected = Vec::from([0, 2, 5]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_double_n_times_0() {
        let point = test_point(1, 5);
        
        let actual = point.clone().double_n_times(0);

        let expected = point;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_double_n_times_1() {
        let point = test_point(1, 5);
        
        let actual = point.clone().double_n_times(1);

        let expected = test_point(2, 10);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_double_n_times_2() {
        let point = test_point(1, 5);
        
        let actual = point.clone().double_n_times(2);

        let expected = test_point(12, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_1() {
        let point = test_point(1, 5);
        let factor = BigInt::from(1);

        let actual = point * factor;

        let expected = test_point(1, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_1_rev() {
        let point = test_point(1, 5);
        let factor = BigInt::from(1);

        let actual = factor * point;

        let expected = test_point(1, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_2() {
        let point = test_point(1, 5);
        let factor = BigInt::from(2);

        let actual = point * factor;

        let expected = test_point(2, 10);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_2_rev() {
        let point = test_point(1, 5);
        let factor = BigInt::from(2);

        let actual = factor * point;

        let expected = test_point(2, 10);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_21() {
        let point = test_point(1, 5);
        let factor = BigInt::from(21);

        let actual = point * factor;

        let expected = test_point(5, 9);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_21_rev() {
        let point = test_point(1, 5);
        let factor = BigInt::from(21);

        let actual = factor * point;

        let expected = test_point(5, 9);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_40() {
        let point = test_point(1, 5);
        let factor = BigInt::from(40);

        let actual = point * factor;

        let expected = test_point(12, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_int_40_rev() {
        let point = test_point(1, 5);
        let factor = BigInt::from(40);

        let actual = factor * point;

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

    #[test]
    fn test_basic_stuff() {
        let private_key = BigInt::parse_bytes(b"1E99423A4ED27608A15A2616A2B0E9E52CED330AC530EDCC32C8FFC6A526AEDD", 16).unwrap();

        let point = Point::generator_point();

        let public_key = point * private_key;

        let expected_x = "F028892BAD7ED57D2FB57BF33081D5CFCF6F9ED3D3D7F159C2E2FFF579DC341A".to_string();
        let expected_y = "07CF33DA18BD734C600B96A72BBC4749D5141C90EC8AC328AE52DDFE2E505BDB".to_string();

        assert_eq!(public_key.x_as_hex(), expected_x);
        assert_eq!(public_key.y_as_hex(), expected_y);
    }

    #[test]
    fn test_lkj() {
        let point = Point::from_xy(BigInt::from(4), BigInt::from(5));
        dbg!("++++++++++++++++++++++++++++++++++++++++++++++", point.p);
        // assert!(false)
    }
}
