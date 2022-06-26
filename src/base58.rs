
use num_bigint::{BigInt, Sign};

pub(crate) struct Base58 ();


impl Base58 {
    pub fn from_vec_u8(n: Vec<u8>) -> String {
        // https://appdevtools.com/base58-encoder-decoder

        let bytes = n.len();
        let mut remainder = BigInt::from_bytes_be(Sign::Plus, &n[..]);

        let largest = BigInt::from(256).pow(bytes as u32);
        let mut didgets = 0;
        let fifty_eight = BigInt::from(58);
        let mut text = Vec::new();

        while fifty_eight.pow(didgets) < largest {
            didgets += 1;
        }
        didgets -=1;
        if remainder < BigInt::parse_bytes(b"000AF820000000000000000000000000000000000000000000", 16).unwrap() {
            didgets -= 1;
        }

        for i in (0..didgets).rev() {
            let devisor = remainder.clone() / fifty_eight.pow(i);
            let result = devisor.to_biguint();
            text.push(dec_to_base58(result.unwrap().to_bytes_le()[0] as u8));
            remainder %= fifty_eight.pow(i);
        }
        
        text.into_iter().collect()
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

    fn hex_string_to_bytes(text: String) -> Vec<u8> {
        let a = text.chars();
        let a = a.collect::<Vec<char>>();
        let a = a.chunks(2);
        let a = a.collect::<Vec<&[char]>>(); 
        let a = a.iter().map(|&c| c.into_iter().collect::<String>());
        let a = a.map(|a| u8::from_str_radix(&a, 16).unwrap());
        let a = a.collect::<Vec<_>>();
        a
    }

    #[test]
    fn test_from_bigint() {
        let n = BigInt::parse_bytes(b"01e99423a4ed27608a15a2616a2b0e9e52ced330ac530edcc32c8fc6a526aedd", 16).unwrap();
        
        let actual = Base58::from_bigint(n);

        let expected = "8TzBRTYxnj5SZ9k9uchuqc9W9TPXmpukCBabj4x2MmW".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_vec_u8() {
        let n = hex_string_to_bytes("002450ABE3830D8508B69EDF22964582B78CCC45CC55C923A8".to_string());

        let actual = Base58::from_vec_u8(n);

        let expected = "14K1y4Epb341duzDmWsPniLyBh9EVh8jG3".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_vec_u8_2() {
        let n = hex_string_to_bytes("000ABDFB3F9CDF20EBC6EC277FCB8186D86D2C3B2095B128E4".to_string());
        let actual = Base58::from_vec_u8(n);

        let expected = "1yoM4VGhTaE4nCJgbmJoevC6ddoeQUzjM".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_vec_u8_3() {
        let n = hex_string_to_bytes("054CF3A338420E93E3C6593A9805B5F0C6EB5D16A73D8BFF08".to_string());
        let actual = Base58::from_vec_u8(n);

        let expected = "38hu9MkFKM1gBPwYR16zP76ssm8rMrHJ4f".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_vec_u8_4() {
        let n = hex_string_to_bytes("6FFCF3A338420E93E3C6593A9805B5F0C6EB5D16A73D8BFF08".to_string());
        let actual = Base58::from_vec_u8(n);

        let expected = "n4aSSdYfwXhXZCdQiKmJcjTFLTJzLGpyw5".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_vec_u8_5() {
        let n = hex_string_to_bytes("6F0CF3A338420E93E3C6593A9805B5F0C6EB5D16A73D8BFF08".to_string());
        let actual = Base58::from_vec_u8(n);

        let expected = "mghSGbGTuB1qAUwCkcpwfN8pgc8c3NDe1d".to_string();

        assert_eq!(actual, expected);
    }
}

// 002450ABE3830D8508B69EDF22964582B78CCC45CC55C923A8

// 000450ABE3830D8508B69EDF22964582B78CCC45CC55C923A8

// 054CF3A338420E93E3C6593A9805B5F0C6EB5D16A73D8BFF08

// 6FFCF3A338420E93E3C6593A9805B5F0C6EB5D16A73D8BFF08

// 6F0CF3A338420E93E3C6593A9805B5F0C6EB5D16A73D8BFF08

// 000ADDFB3F9CDF20EBC6EC277FCB8186D86D2C3B2095B128E4
// 000ADD00000000000000000000000000000000000000000000
// 0000AE10000000000000000000000000000000000000000000

// 1SiT7kyGAuaHgu7nVa25QcWGqLNQhzsGB
