use bigi::Bigi;
use bigi_ecc::{point, Point};

use crate::HASH_STORAGE_BITS;

const BIGI_HEX_LENGTH: usize = HASH_STORAGE_BITS / 4;


pub fn hex_from_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X?}", b)).collect()
}


pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(
        |i| u8::from_str_radix(&hex[i..(i + 2)], 16).unwrap()
    ).collect()
}


pub fn hex_to_bigi(hex: &str) -> Bigi {
    Bigi::from_bytes(&hex_to_bytes(&hex[..BIGI_HEX_LENGTH]))
}


pub fn hex_from_bigi(b: &Bigi) -> String {
    hex_from_bytes(&b.to_bytes())[..BIGI_HEX_LENGTH].to_string()
}


pub fn hex_to_point(hex: &str) -> Point {
    point!(
        hex_to_bigi(&hex[..BIGI_HEX_LENGTH]),
        hex_to_bigi(&hex[BIGI_HEX_LENGTH..])
    )
}


pub fn hex_from_point(p: &Point) -> String {
    hex_from_bigi(&p.x) + &hex_from_bigi(&p.y)
}


pub fn hex_to_bigi_vec(hex: &str) -> Vec<Bigi> {
    (0..hex.len()).step_by(BIGI_HEX_LENGTH).map(
        |i| hex_to_bigi(&hex[i..(i + BIGI_HEX_LENGTH)])
    ).collect()
}


pub fn hex_from_bigi_vec(v: &Vec<Bigi>) -> String {
    v.iter().map(|b| hex_from_bigi(&b)).collect()
}


pub fn hex_to_point_vec(hex: &str) -> Vec<Point> {
    (0..hex.len()).step_by(2 * BIGI_HEX_LENGTH).map(
        |i| hex_to_point(&hex[i..(2 * BIGI_HEX_LENGTH)])
    ).collect()
}


pub fn hex_from_point_vec(v: &Vec<Point>) -> String {
    v.iter().map(|p| hex_from_point(&p)).collect()
}


pub fn hex_to_bigi_pair(hex: &String) -> (Bigi, Bigi) {
    (
        hex_to_bigi(&hex[..BIGI_HEX_LENGTH]),
        hex_to_bigi(&hex[BIGI_HEX_LENGTH..])
    )
}


pub fn hex_from_bigi_pair(b: &(Bigi, Bigi)) -> String {
    hex_from_bigi(&b.0) + &hex_from_bigi(&b.1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_from_bytes() {
        assert_eq!(hex_from_bytes(&vec![]), "");
        assert_eq!(hex_from_bytes(&vec![123, 12, 67, 255]), "7B0C43FF");
    }

    #[test]
    fn test_hex_to_bytes() {
        assert_eq!(hex_to_bytes(&"".to_string()), Vec::<u8>::new());
        assert_eq!(hex_to_bytes(&"7B0C43FF".to_string()), vec![123, 12, 67, 255]);
    }
}
