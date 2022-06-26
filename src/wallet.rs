use std::fmt::format;

use num_bigint::{BigInt, Sign};
use sha2::Sha256;
use sha2::Digest as ShaDigest;
use ripemd::Ripemd160;
use ripemd::Digest as RipDigest;

use crate::point::Point;
use crate::base58::Base58;

// https://gobittest.appspot.com/

struct Wallet {
    private_key: BigInt,
    public_key_point: Point,
    a: BigInt,
    b: BigInt,
    p: BigInt,
    n: BigInt,
}

impl From<BigInt> for Wallet {
    fn from(private_key: BigInt) -> Self {
        let two = BigInt::from(2);
        let p = two.pow(256) 
            - two.pow(32) 
            - two.pow(9) 
            - two.pow(8) 
            - two.pow(7)
            - two.pow(6)
            - two.pow(4)
            - BigInt::from(1u8);
        let a = BigInt::from(0);
        let n = BigInt::from_bytes_be(Sign::Plus, b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141");


        let public_key_point = Wallet::generator_point().multiply(&private_key, &p, &a);
        // let public_key_point = private_key.clone() * Wallet::generator_point().multiply();

        Self { 
            private_key,
            public_key_point,
            a,
            b: BigInt::from(7),
            p,
            n,
        }
    }
}

impl From<String> for Wallet {
    fn from(private_key: String) -> Self {
        let two = BigInt::from(2);
        let p = two.pow(256) 
            - two.pow(32) 
            - two.pow(9) 
            - two.pow(8) 
            - two.pow(7)
            - two.pow(6)
            - two.pow(4)
            - BigInt::from(1u8);
        let a = BigInt::from(0);
        let n = BigInt::from_bytes_be(Sign::Plus, b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141");
        let private_key = BigInt::parse_bytes(private_key.as_bytes(), 16).unwrap();
        let public_key_point = Wallet::generator_point().multiply(&private_key, &p, &a);

        Self { 
            private_key,
            public_key_point,
            a,
            b: BigInt::from(7),
            p,
            n,
        }
    }
}

impl Wallet {
    fn generator_point() -> Point {
            Point::from_xy(
                BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(), 
                BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap()
            )
    }

    fn private_key(&self) -> String {
        self.private_key.to_str_radix(16).to_uppercase().to_string()
    }

    fn decoded_private_key(&self) -> String {
        self.private_key.to_str_radix(10).to_uppercase().to_string()
    }

    fn wif_encoded_private_key(&self) -> String {
        
    }

    fn pub_key_compressed_string(&self) -> String {
        // https://learnmeabitcoin.com/technical/public-key
        let prefix = if self.public_key_point.y_is_even() {
            "02".to_string()
        } else {
            "03".to_string()
        };
        format!("{}{}", prefix, self.public_key_point.x_to_hex_string())
    }

    fn pub_key_uncompressed_string(&self) -> String {
        format!("04{}{}", self.public_key_point.x_to_hex_string(), self.public_key_point.y_to_hex_string())
    }

    fn pub_key_compressed_bytes(&self) -> Vec<u8> {
        // https://learnmeabitcoin.com/technical/public-key

        let mut thing = if self.public_key_point.y_is_even() {
            Vec::from([2])
        } else {
            Vec::from([3])
        };
        thing.append(&mut self.public_key_point.x_bytes());
        thing
    }

    fn pub_key_uncompressed_bytes(&self) -> Vec<u8> {
        let mut thing = Vec::from([4]);
        thing.append(&mut self.public_key_point.x_bytes());
        thing.append(&mut self.public_key_point.y_bytes());
        thing
    }

    fn address(&self) -> String {

        let private_key = BigInt::parse_bytes(b"038109007313a5807b2eccc082c8c3fbb988a973cacf1a7df9ce725c31b14776", 16).unwrap();
        let wallet = Wallet::from(private_key);

        // let public_address = wallet.pub_key_compressed_bytes();

        let public_key = wallet.pub_key_compressed_bytes();
        let mut sha_hasher = Sha256::new();
        sha_hasher.update(public_key);
        let sha_result = sha_hasher.finalize();
        let mut rip_hasher = Ripemd160::new();
        rip_hasher.update(&sha_result[..]);
        let rip_result = rip_hasher.finalize();

        let b58 = Base58::from_bigint(BigInt::from_bytes_be(Sign::Plus, &rip_result[..]));

        dbg!(sha_result);
        dbg!(rip_result);

        "hello".to_string()
    }

    // fn private_key_base58(&self) {
    //     let n = base58::ToBase58("hello");
    // }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // #[test]
    // fn test_start_public_key_calculations() {
    //     let private_key = BigInt::parse_bytes(b"1E99423A4ED27608A15A2616A2B0E9E52CED330AC530EDCC32C8FFC6A526AEDD", 16).unwrap();

    //     let point = Point::generator_point();

    //     // K = kG
    //     // public_key = private_key * generater_point
    //     let public_key = point * private_key;

    //     let expected_x = "F028892BAD7ED57D2FB57BF33081D5CFCF6F9ED3D3D7F159C2E2FFF579DC341A".to_string();
    //     // let expected_y = "07CF33DA18BD734C600B96A72BBC4749D5141C90EC8AC328AE52DDFE2E505BDB".to_string();

    //     assert_eq!(public_key.x_to_hex_string(), expected_x);
    //     // assert_eq!(public_key.y_to_hex_string(), expected_y);
    // }

    #[test]
    fn test_generate_compressed_public_key_even() {
        let private_key = "038109007313a5807b2eccc082c8c3fbb988a973cacf1a7df9ce725c31b14776".to_string();
        let wallet = Wallet::from(private_key);

        let public_key = wallet.pub_key_compressed_string();

        let expected_public_key = "0202A406624211F2ABBDC68DA3DF929F938C3399DD79FAC1B51B0E4AD1D26A47AA".to_string().to_uppercase();

        assert_eq!(public_key, expected_public_key);
    }

    #[test]
    fn test_generate_compressed_public_key_odd() {
        let private_key = "038109007313a5807b2eccc082c8c3fbb988a973cacf1a7df9ce725c31b14777".to_string();
        let wallet = Wallet::from(private_key);

        let public_key = wallet.pub_key_compressed_string();

        let expected_public_key = "032e294c59fd0b721437a76b8f133e6bb79a222e6488a5296eae96599750f75120".to_string().to_uppercase();

        assert_eq!(public_key, expected_public_key);
    }

    #[test]
    fn test_generate_uncompressed_public_key() {
        let private_key = "038109007313a5807b2eccc082c8c3fbb988a973cacf1a7df9ce725c31b14776".to_string();
        let wallet = Wallet::from(private_key);

        let public_key = wallet.pub_key_uncompressed_string();

        let expected_public_key = "0402a406624211f2abbdc68da3df929f938c3399dd79fac1b51b0e4ad1d26a47aa9f3bc9f3948a19dabb796a2a744aae50367ce38a3e6b60ae7d72159caeb0c102".to_string().to_uppercase();
 
        assert_eq!(public_key, expected_public_key);
    }

    #[test]
    fn test_p69_follow_along() {
        let prvate_key = "038109007313a5807b2eccc082c8c3fbb988a973cacf1a7df9ce725c31b14776".to_string();
        let wallet = Wallet::from(prvate_key);

        let public_key = wallet.pub_key_compressed_string();
        let expected_pub_key = "0202a406624211f2abbdc68da3df929f938c3399dd79fac1b51b0e4ad1d26a47aa".to_uppercase().to_string();

        assert_eq!(public_key, expected_pub_key);

        let mut sha_hasher = Sha256::new();
        sha_hasher.update(public_key);
        let sha_result = sha_hasher.finalize();
        // dbg!("Public key shaed 1 time:", sha_result);

        let mut rip_hasher = Ripemd160::new();
        rip_hasher.update(&sha_result[..]);
        let rip_result = rip_hasher.finalize();
        // dbg!("RIPEMD160ed:", &rip_result);

        let mut added_00_byte = Vec::from([0]);
        added_00_byte.append(&mut rip_result.to_vec());
        // dbg!("zeros added:", &added_00_byte);

        let mut sha_hasher = Sha256::new();
        sha_hasher.update(added_00_byte.clone());
        let sha_result = sha_hasher.finalize();
        // dbg!("Added zeros hashed once", &sha_result);

        let mut sha_hasher = Sha256::new();
        sha_hasher.update(&sha_result[..]);
        let sha_result = sha_hasher.finalize();
        // dbg!("Added zeros hashed twice", sha_result);

        let first_four_bytes = &sha_result[0..4];

        added_00_byte.append(&mut first_four_bytes.to_vec());
        // dbg!("stuff with checksum:", &added_00_byte);

        let base58_address = Base58::from_vec_u8(added_00_byte);
        let expected_address = "1PRTTaJesdNovgne6Ehcdu1fpEdX7913CK".to_string();

        assert_eq!(base58_address, expected_address);


        // let b58 = Base58::from_bigint(BigInt::from_bytes_be(Sign::Plus, &added_00_byte[..]));


    }

    #[test]
    fn test_p76_follow_along() {
        let prvate_key = "3aba4162c7251c891207b747840551a71939b0de081f85c4e44cf7c13e41daa6".to_string();
        let wallet = Wallet::from(prvate_key);

        let private_key = "3aba4162c7251c891207b747840551a71939b0de081f85c4e44cf7c13e41daa6".to_string();
        assert_eq!(wallet.private_key().to_lowercase(), private_key);

        let decoded_private_key = "26563230048437957592232553826663696440606756685920117476832299673293013768870".to_string();
        assert_eq!(wallet.decoded_private_key().to_lowercase(), decoded_private_key);


    }

    #[test]
    fn test_generate_address_from_public_key() {
        // https://gobittest.appspot.com/Address
        let private_key = BigInt::parse_bytes(b"038109007313a5807b2eccc082c8c3fbb988a973cacf1a7df9ce725c31b14776", 16).unwrap();
        // dbg!("Private key:", &private_key);
        let wallet = Wallet::from(private_key);

        // let public_address = wallet.pub_key_compressed_bytes();

        let public_key = wallet.pub_key_uncompressed_bytes();
        // dbg!("Public key:", &public_key);

        let mut sha_hasher = Sha256::new();
        sha_hasher.update(public_key);
        let sha_result = sha_hasher.finalize();
        // dbg!("Public key shaed 1 time:", sha_result);

        let mut rip_hasher = Ripemd160::new();
        rip_hasher.update(&sha_result[..]);
        let rip_result = rip_hasher.finalize();
        // dbg!("RIPEMD160ed:", &rip_result);

        let mut added_00_byte = Vec::from([0]);
        added_00_byte.append(&mut rip_result.to_vec());
        // dbg!("zeros added:", &added_00_byte);

        let mut sha_hasher = Sha256::new();
        sha_hasher.update(added_00_byte.clone());
        let sha_result = sha_hasher.finalize();
        // dbg!("Added zeros hashed once", &sha_result);

        let mut sha_hasher = Sha256::new();
        sha_hasher.update(&sha_result[..]);
        let sha_result = sha_hasher.finalize();
        // dbg!("Added zeros hashed twice", sha_result);

        let first_four_bytes = &sha_result[0..4];

        added_00_byte.append(&mut first_four_bytes.to_vec());
        // dbg!("stuff with checksum:", &added_00_byte);


        let b58 = Base58::from_vec_u8(added_00_byte);

        // dbg!(b58);
        // dbg!(rip_result);

        let expected_b58 = "14K1y4Epb341duzDmWsPniLyBh9EVh8jG3".to_string();

        assert_eq!(b58, expected_b58);
    }
}