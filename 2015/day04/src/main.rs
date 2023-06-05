use md5;

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
        let mut answer = 0;
        let zeroes = &str::repeat("0", self.zeroes as usize);

        loop {
            let to_hash = self.secret_key.to_string() + answer.to_string().as_str();
            let hash = format!("{:x}", md5::compute(to_hash.as_bytes()));
            if hash.starts_with(zeroes) {
                break;
            }
            answer += 1;
        }

        answer
    }
}

#[test]
#[ignore]
fn examples() {
    assert!(format!("{:x}", md5::compute(b"abcdef609043")).starts_with("000001dbbfa"));
    assert_eq!(AdventCoin::new("abcdef", 5).mine(), 609043);
    assert_eq!(AdventCoin::new("pqrstuv", 5).mine(), 1048970);
}
