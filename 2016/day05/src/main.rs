use md5;

fn main() {
    static INPUT: &str = "abbhdwsy";
    println!("{} {}", password(INPUT), password2(INPUT));
}

fn password(input: &str) -> String {
    let mut password = String::new();
    let mut index: u32 = 0;

    while password.len() < 8 {
        let hash = md5::compute(format!("{}{}", input, index));
        let hash = format!("{:x}", hash);
        if hash.starts_with("00000") {
            password.push(hash.chars().nth(5).unwrap());
        }
        index += 1;
    }

    password
}

fn password2(input: &str) -> String {
    let mut password = vec![' '; 8];
    let mut index: u32 = 0;

    while password.iter().any(|&c| c == ' ') {
        let hash = md5::compute(format!("{}{}", input, index));
        let hash = format!("{:x}", hash);
        if hash.starts_with("00000") {
            let pos = hash.chars().nth(5).unwrap();
            if let Some(pos) = pos.to_digit(10) {
                let pos = pos as usize;
                if pos < 8 && password[pos] == ' ' {
                    password[pos] = hash.chars().nth(6).unwrap();
                }
            }
        }
        index += 1;
    }

    password.into_iter().collect()
}

#[test]
#[ignore] // Takes ~20s for password only!
fn examples() {
    assert_eq!(password("abc"), "18f47a30");
    assert_eq!(password2("abc"), "05ace8e3");
}
