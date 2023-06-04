use md5;

fn main() {
    let input = include_str!("input");
    println!("{} {}", AdventCoin::new(input).mine(), 0);
}

struct AdventCoin<'a> {
    secret_key: &'a str,
}

impl AdventCoin<'_> {
    fn new(secret_key: &str) -> AdventCoin {
        AdventCoin {
            secret_key: secret_key,
        }
    }

    fn mine(self) -> u32 {
        let mut answer = 0;

        loop {
            let to_hash = self.secret_key.to_string() + answer.to_string().as_str();
            let hash = format!("{:x}", md5::compute(to_hash.as_bytes()));
            if hash.starts_with("00000") {
                break;
            }
            answer += 1;
        }

        answer
    }
}

#[test]
fn examples() {
    assert_eq!(
        format!("{:x}", md5::compute(b"abcdef609043")),
        "000001dbbfa3a5c83a2d506429c7b00e"
    );
    assert_eq!(AdventCoin::new("abcdef").mine(), 609043);
    assert_eq!(AdventCoin::new("pqrstuv").mine(), 1048970);
}
