
use num_bigint::{BigInt, Sign};

pub(crate) struct Base58 ();

impl Base58 {
    pub fn from_vec_u8(n: Vec<u8>) -> String {
        let fifty_eight = BigInt::from(58);
        let zero = BigInt::from(0);

        let mut zeros = Vec::new();

        let mut idx = 0;
        while n[idx] == 0 {
            zeros.push(dec_to_base58(0));
            idx += 1;
        }

        let mut text = Vec::new();
        let n = BigInt::from_bytes_be(Sign::Plus, &n[..]);
        let mut devisor = n;
        while devisor > zero {
            let remainder = *(devisor.clone() % fifty_eight.clone()).to_biguint().unwrap().to_bytes_be().last().unwrap();
            devisor /= fifty_eight.clone();
            text.push(dec_to_base58(remainder));
        }

        text.reverse();
        zeros.append(&mut text);
        zeros.into_iter().collect()
    }


    pub fn from_bigint(n: BigInt) -> String {
        let fifty_eight = BigInt::from(58);
        let zero = BigInt::from(0);
        let mut text = Vec::new();
        let mut devisor = n;
        while devisor > zero {
            // dbg!((devisor.clone() % fifty_eight.clone()).to_u32_digits());
            let remainder = *(devisor.clone() % fifty_eight.clone()).to_biguint().unwrap().to_bytes_be().last().unwrap();
            devisor /= fifty_eight.clone();
            let char = match remainder {
                0 => '1',
                1 => '2',
                2 => '3',
                3 => '4',
                4 => '5',
                5 => '6',
                6 => '7',
                7 => '8',
                8 => '9',
                9 => 'A',
                10 => 'B',
                11 => 'C',
                12 => 'D',
                13 => 'E',
                14 => 'F',
                15 => 'G',
                16 => 'H',
                17 => 'J',
                18 => 'K',
                19 => 'L',
                20 => 'M',
                21 => 'N',
                22 => 'P',
                23 => 'Q',
                24 => 'R',
                25 => 'S',
                26 => 'T',
                27 => 'U',
                28 => 'V',
                29 => 'W',
                30 => 'X',
                31 => 'Y',
                32 => 'Z',
                33 => 'a',
                34 => 'b',
                35 => 'c',
                36 => 'd',
                37 => 'e',
                38 => 'f',
                39 => 'g',
                40 => 'h',
                41 => 'i',
                42 => 'j',
                43 => 'k',
                44 => 'm',
                45 => 'n',
                46 => 'o',
                47 => 'p',
                48 => 'q',
                49 => 'r',
                50 => 's',
                51 => 't',
                52 => 'u',
                53 => 'v',
                54 => 'w',
                55 => 'x',
                56 => 'y',
                57 => 'z',
                _ => panic!("")
            };
            text.push(char);
        }
        text.reverse();
        text.into_iter().collect()
    }
}

fn dec_to_base58(n: u8) -> char {
    match n {
        0 => '1',
        1 => '2',
        2 => '3',
        3 => '4',
        4 => '5',
        5 => '6',
        6 => '7',
        7 => '8',
        8 => '9',
        9 => 'A',
        10 => 'B',
        11 => 'C',
        12 => 'D',
        13 => 'E',
        14 => 'F',
        15 => 'G',
        16 => 'H',
        17 => 'J',
        18 => 'K',
        19 => 'L',
        20 => 'M',
        21 => 'N',
        22 => 'P',
        23 => 'Q',
        24 => 'R',
        25 => 'S',
        26 => 'T',
        27 => 'U',
        28 => 'V',
        29 => 'W',
        30 => 'X',
        31 => 'Y',
        32 => 'Z',
        33 => 'a',
        34 => 'b',
        35 => 'c',
        36 => 'd',
        37 => 'e',
        38 => 'f',
        39 => 'g',
        40 => 'h',
        41 => 'i',
        42 => 'j',
        43 => 'k',
        44 => 'm',
        45 => 'n',
        46 => 'o',
        47 => 'p',
        48 => 'q',
        49 => 'r',
        50 => 's',
        51 => 't',
        52 => 'u',
        53 => 'v',
        54 => 'w',
        55 => 'x',
        56 => 'y',
        57 => 'z',
        _ => panic!("")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_from_bigint() {
        let n = BigInt::parse_bytes(b"01e99423a4ed27608a15a2616a2b0e9e52ced330ac530edcc32c8fc6a526aedd", 16).unwrap();
        
        let actual = Base58::from_bigint(n);

        let expected = "8TzBRTYxnj5SZ9k9uchuqc9W9TPXmpukCBabj4x2MmW".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_vec_u8() {
        let n = Vec::from([0x00, 0x24, 0x50, 0xAB, 0xE3, 0x83, 0x0D, 0x85, 0x08, 0xB6, 0x9E, 0xDF, 0x22, 0x96, 0x45, 0x82, 0xB7, 0x8C, 0xCC, 0x45, 0xCC, 0x55, 0xC9, 0x23, 0xA8]);

        let actual = Base58::from_vec_u8(n);

        let expected = "14K1y4Epb341duzDmWsPniLyBh9EVh8jG3".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_vec_u8_2() {
        let n = Vec::from([0x00, 0x04, 0x50, 0xAB, 0xE3, 0x83, 0x0D, 0x85, 0x08, 0xB6, 0x9E, 0xDF, 0x22, 0x96, 0x45, 0x82, 0xB7, 0x8C, 0xCC, 0x45, 0xCC, 0x55, 0xC9, 0x23, 0xA8]);
        let actual = Base58::from_vec_u8(n);

        let expected = "1PpLvCfFPVenV6Te65UjCSFENz7f3BkUF".to_string();

        assert_eq!(actual, expected);
    }
}

