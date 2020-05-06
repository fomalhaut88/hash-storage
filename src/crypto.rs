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


pub fn check_data_block_size(data_block: &Vec<u8>) -> bool {
    data_block.len() < 1048576  // 2^20 bytes (or 1 MB)
}


pub fn check_data_signature(public_key: &Point,
                            data_key: &Vec<u8>,
                            data_block: &Vec<u8>,
                            signature: &(Bigi, Bigi)) -> bool {
    let hash = {
        let mut hasher = Sha256::new();
        hasher.input(data_key);
        hasher.input(data_block);
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
    use bigi_ecc::mapping::Mapper;
    use bigi_ecc::elgamal;
    use bigi_ecc::ecdsa::build_signature;
    use crate::utils::*;
    use super::*;

    #[test]
    fn test_generate_secret() {
        let secret = generate_secret();
        assert_eq!(secret.len(), HASH_STORAGE_BITS / 8);
    }

    #[test]
    fn test_check_data_block_size() {
        assert_eq!(
            check_data_block_size(&vec![5; 100]),
            true
        );
        assert_eq!(
            check_data_block_size(&vec![5; 3000000]),
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
        let data_key = hex_to_bytes("20EFF6E29B0ABCE86454A675A5FB4D64F5F48A9B8BCFA4E483A61EB7D4C9B8FF");
        let message = b"hash-storage";

        // Encrypting data
        let data_block = {
            let mapper = Mapper::new(256);
            let points = mapper.pack(&message.to_vec(), &schema.curve);
            let (c1, c2) = elgamal::encrypt(&mut rng, &schema, &public_key, &points);
            let mut res: Vec<u8> = hex_to_bytes(&hex_from_point(&c1));
            for p in c2 {
                res.extend(&hex_to_bytes(&hex_from_point(&p)));
            }
            res
        };

        // Building signature
        let signature = {
            let mut hasher = Sha256::new();
            hasher.input(&data_key);
            hasher.input(&data_block);
            let hash = hasher.result().to_vec();

            build_signature(&mut rng, &schema, &private_key, &hash)
        };

        // Checking
        assert_eq!(
            check_data_signature(&public_key, &data_key, &data_block, &signature),
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
}
