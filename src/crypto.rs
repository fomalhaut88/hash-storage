extern crate rand;

use rand::Rng;
use sha2::{Sha256, Digest};
use bigi::Bigi;
use bigi_ecc::schemas;
use bigi_ecc::Point;
use bigi_ecc::ecdsa::check_signature;

use crate::HASH_STORAGE_BITS;


pub fn generate_secret() -> Vec<u8> {
    /* Generates 32 bytes randomly (256 bits) */
    let mut rng = rand::thread_rng();
    (0..(HASH_STORAGE_BITS / 8)).map(|_| rng.gen::<u8>()).collect()
}


pub fn check_data_block_size(data_block: &String) -> bool {
    data_block.len() < 16777216  // 2^24 bytes (or 16 MB)
}


pub fn check_data_signature(public_key: &Point,
                            data_group: &String,
                            data_key: &String,
                            data_block: &String,
                            data_version: &String,
                            signature: &(Bigi, Bigi)) -> bool {
    let hash = {
        let mut hasher = Sha256::new();
        hasher.input(data_group);
        hasher.input(data_key);
        hasher.input(data_block);
        hasher.input(data_version);
        hasher.result().to_vec()
    };

    check_signature(&schemas::load_secp256k1(), public_key, &hash, signature)
}


pub fn check_secret_signature(public_key: &Point,
                              secret: &Vec<u8>,
                              secret_signature: &(Bigi, Bigi)) -> bool {
    check_signature(&schemas::load_secp256k1(), public_key,
                    secret, secret_signature)
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use bigi_ecc::ecdsa::build_signature;
    // use crate::utils::*;

    #[test]
    fn test_generate_secret() {
        let secret = generate_secret();
        assert_eq!(secret.len(), HASH_STORAGE_BITS / 8);
    }

    #[test]
    fn test_check_data_block_size() {
        assert_eq!(
            check_data_block_size(&String::from_utf8(vec![65; 100]).unwrap()),
            true
        );
        assert_eq!(
            check_data_block_size(&String::from_utf8(vec![65; 30000000]).unwrap()),
            false
        );
    }

    #[test]
    fn test_check_data_signature() {
        // Initialization
        let mut rng = rand::thread_rng();
        let schema = schemas::load_secp256k1();

        // Generating keys
        let (private_key, public_key) = schema.generate_pair(&mut rng);

        // Initial data
        let data_group: String = "My group".to_string();
        let data_key: String = "My data key".to_string();
        let data_block: String = "My shared data block".to_string();
        let data_version: String = "1".to_string();

        // Building signature
        let signature = {
            let mut hasher = Sha256::new();
            hasher.input(&data_group);
            hasher.input(&data_key);
            hasher.input(&data_block);
            hasher.input(&data_version);
            let hash = hasher.result().to_vec();

            build_signature(&mut rng, &schema, &private_key, &hash)
        };

        // let secret = generate_secret();
        // println!("private_key = {:?}", hex_from_bigi(&private_key));
        // println!("public_key = {:?}", hex_from_point(&public_key));
        // println!("data_group = {:?}", data_group);
        // println!("data_key = {:?}", data_key);
        // println!("data_block = {:?}", data_block);
        // println!("data_version = {:?}", data_version);
        // println!("signature = {:?}", hex_from_bigi_pair(&signature));
        // println!("secret = {:?}", hex_from_bytes(&secret));
        // println!("secret_signature = {:?}", hex_from_bigi_pair(&build_signature(&mut rng, &schema, &private_key, &secret)));

        // Checking
        assert_eq!(
            check_data_signature(&public_key, &data_group, &data_key,
                                 &data_block, &data_version, &signature),
            true
        );
    }

    #[test]
    fn test_check_secret_signature() {
        // Initialization
        let mut rng = rand::thread_rng();
        let schema = schemas::load_secp256k1();

        // Generating keys
        let (private_key, public_key) = schema.generate_pair(&mut rng);

        // Initial data
        let secret = generate_secret();

        // Building secret_signature
        let secret_signature = build_signature(&mut rng, &schema, &private_key, &secret);

        // Checking
        assert_eq!(
            check_secret_signature(&public_key, &secret, &secret_signature),
            true
        );
    }

    #[bench]
    fn bench_generate_secret(b: &mut Bencher) {
        b.iter(|| generate_secret());
    }

    #[bench]
    fn bench_check_data_block_size(b: &mut Bencher) {
        let data_block = &String::from_utf8(vec![65; 1000000]).unwrap();
        b.iter(|| check_data_block_size(&data_block));
    }

    #[bench]
    fn bench_check_data_signature(b: &mut Bencher) {
        // Initialization
        let mut rng = rand::thread_rng();
        let schema = schemas::load_secp256k1();

        // Generating keys
        let (private_key, public_key) = schema.generate_pair(&mut rng);

        // Initial data
        let data_group: String = "My group".to_string();
        let data_key: String = "My data key".to_string();
        let data_block: String = "My shared data block".to_string();
        let data_version: String = "1".to_string();

        // Building signature
        let signature = {
            let mut hasher = Sha256::new();
            hasher.input(&data_group);
            hasher.input(&data_key);
            hasher.input(&data_block);
            hasher.input(&data_version);
            let hash = hasher.result().to_vec();

            build_signature(&mut rng, &schema, &private_key, &hash)
        };

        // Benchmark
        b.iter(||
            check_data_signature(&public_key, &data_group, &data_key,
                                 &data_block, &data_version, &signature)
        );
    }

    #[bench]
    fn bench_check_secret_signature(b: &mut Bencher) {
        // Initialization
        let mut rng = rand::thread_rng();
        let schema = schemas::load_secp256k1();

        // Generating keys
        let (private_key, public_key) = schema.generate_pair(&mut rng);

        // Initial data
        let secret = generate_secret();

        // Building secret_signature
        let secret_signature = build_signature(&mut rng, &schema, &private_key, &secret);

        // Benchmark
        b.iter(|| check_secret_signature(&public_key, &secret, &secret_signature));
    }
}
