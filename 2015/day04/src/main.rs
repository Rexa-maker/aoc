use md5;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input");
    println!(
        "{} {}",
        AdventCoin::new(input, 5).mine(),
        AdventCoin::new(input, 6).mine()
    );
}

struct AdventCoin<'a> {
    secret_key: &'a str,
    zeroes: u8,
}

impl AdventCoin<'_> {
    fn new(secret_key: &str, zeroes: u8) -> AdventCoin {
        AdventCoin {
            secret_key: secret_key,
            zeroes: zeroes,
        }
    }

    fn mine(self) -> u32 {
        let zeroes = &str::repeat("0", self.zeroes as usize);

        (1..u32::max_value())
            .into_par_iter()
            .find_first(|&number| {
                let to_hash = format!("{}{}", self.secret_key, number);
                let hash = format!("{:x}", md5::compute(to_hash.as_bytes()));
                hash.starts_with(zeroes)
            })
            .unwrap()
    }
}

#[test]
fn examples() {
    assert!(format!("{:x}", md5::compute(b"abcdef609043")).starts_with("000001dbbfa"));
    assert_eq!(AdventCoin::new("abcdef", 5).mine(), 609043);
    assert_eq!(AdventCoin::new("pqrstuv", 5).mine(), 1048970);
}
