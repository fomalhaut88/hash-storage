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
        |i| hex_to_point(&hex[i..(i + 2 * BIGI_HEX_LENGTH)])
    ).collect()
}


pub fn hex_from_point_vec(v: &Vec<Point>) -> String {
    v.iter().map(|p| hex_from_point(&p)).collect()
}


pub fn hex_to_bigi_pair(hex: &str) -> (Bigi, Bigi) {
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
    use test::Bencher;

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

    #[bench]
    fn bench_hex_from_bytes(b: &mut Bencher) {
        let bytes: Vec<u8> = (0..256).map(|_| { rand::random::<u8>() }).collect();
        b.iter(|| hex_from_bytes(&bytes));
    }

    #[bench]
    fn bench_hex_to_bytes(b: &mut Bencher) {
        let bytes: Vec<u8> = (0..256).map(|_| { rand::random::<u8>() }).collect();
        let hex = hex_from_bytes(&bytes);
        b.iter(|| hex_to_bytes(&hex));
    }

    #[bench]
    fn bench_hex_from_bigi(b: &mut Bencher) {
        let hex = "9FC4E198E0AB8F0E4D240DD39F892BFA85809F3406EBE105800014E7C7E39445";
        let x = hex_to_bigi(hex);
        b.iter(|| hex_from_bigi(&x));
    }

    #[bench]
    fn bench_hex_to_bigi(b: &mut Bencher) {
        let hex = "9FC4E198E0AB8F0E4D240DD39F892BFA85809F3406EBE105800014E7C7E39445";
        b.iter(|| hex_to_bigi(&hex));
    }

    #[bench]
    fn bench_hex_from_point(b: &mut Bencher) {
        let hex = "604CE6D82472A1D921BE694155A2C76E02F33330E6CD9045B5AD4A6BD6778F657560BCAD1C18397063E46155EC684151A59E1AAE0AA4F43DBB09525C0DD768ED";
        let x = hex_to_point(hex);
        b.iter(|| hex_from_point(&x));
    }

    #[bench]
    fn bench_hex_to_point(b: &mut Bencher) {
        let hex = "604CE6D82472A1D921BE694155A2C76E02F33330E6CD9045B5AD4A6BD6778F657560BCAD1C18397063E46155EC684151A59E1AAE0AA4F43DBB09525C0DD768ED";
        b.iter(|| hex_to_point(&hex));
    }

    #[bench]
    fn bench_hex_from_bigi_pair(b: &mut Bencher) {
        let hex = "604CE6D82472A1D921BE694155A2C76E02F33330E6CD9045B5AD4A6BD6778F657560BCAD1C18397063E46155EC684151A59E1AAE0AA4F43DBB09525C0DD768ED";
        let x = hex_to_bigi_pair(hex);
        b.iter(|| hex_from_bigi_pair(&x));
    }

    #[bench]
    fn bench_hex_to_bigi_pair(b: &mut Bencher) {
        let hex = "604CE6D82472A1D921BE694155A2C76E02F33330E6CD9045B5AD4A6BD6778F657560BCAD1C18397063E46155EC684151A59E1AAE0AA4F43DBB09525C0DD768ED";
        b.iter(|| hex_to_bigi_pair(&hex));
    }

    #[bench]
    fn bench_hex_from_bigi_vec(b: &mut Bencher) {
        let hex = "9FC4E198E0AB8F0E4D240DD39F892BFA85809F3406EBE105800014E7C7E39445";
        let x = hex_to_bigi_vec(hex);
        b.iter(|| hex_from_bigi_vec(&x));
    }

    #[bench]
    fn bench_hex_to_bigi_vec(b: &mut Bencher) {
        let hex = "9FC4E198E0AB8F0E4D240DD39F892BFA85809F3406EBE105800014E7C7E39445";
        b.iter(|| hex_to_bigi_vec(&hex));
    }

    #[bench]
    fn bench_hex_from_point_vec(b: &mut Bencher) {
        let hex = "604CE6D82472A1D921BE694155A2C76E02F33330E6CD9045B5AD4A6BD6778F657560BCAD1C18397063E46155EC684151A59E1AAE0AA4F43DBB09525C0DD768ED";
        let x = hex_to_point_vec(hex);
        b.iter(|| hex_from_point_vec(&x));
    }

    #[bench]
    fn bench_hex_to_point_vec(b: &mut Bencher) {
        let hex = "604CE6D82472A1D921BE694155A2C76E02F33330E6CD9045B5AD4A6BD6778F657560BCAD1C18397063E46155EC684151A59E1AAE0AA4F43DBB09525C0DD768ED";
        b.iter(|| hex_to_point_vec(&hex));
    }
}
