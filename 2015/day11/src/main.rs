fn main() {
    static INPUT: &str = "vzbxkghb";
    let next = next_password(INPUT);
    let nextnext = next_password(&next);
    println!("{} {}", next, nextnext);
}

fn next_password(password: &str) -> String {
    let mut password = password.to_string();

    loop {
        password = increment(&password);
        if is_valid(&password) {
            return password;
        }
    }
}

fn is_valid(password: &str) -> bool {
    let mut has_straight = false;
    let mut has_two_pairs = false;
    let mut last_pair = None;

    for (i, c) in password.chars().enumerate() {
        // Check for invalid characters
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        // Check for a straight
        if i > 1
            && c as u8 == password.as_bytes()[i - 1] + 1
            && c as u8 == password.as_bytes()[i - 2] + 2
        {
            has_straight = true;
        }

        // Check for two pairs
        if i > 0 && c == password.as_bytes()[i - 1] as char {
            if last_pair.is_none() {
                last_pair = Some(c);
            } else if last_pair.unwrap() != c {
                has_two_pairs = true;
            }
        }
    }

    has_straight && has_two_pairs
}

fn increment(password: &str) -> String {
    let mut password = password.to_string();

    for i in (0..password.len()).rev() {
        let mut c = password.as_bytes()[i];

        // First, increment the character
        if c == 'z' as u8 {
            c = 'a' as u8;
        } else {
            c += 1;
        }
        password.replace_range(i..=i, &(c as char).to_string());

        // Then, check if we need to increment the next character
        if c != 'a' as u8 {
            break;
        }
    }

    password
}

#[test]
fn example() {
    assert!(is_valid("hijklmmn") == false);
    assert!(is_valid("abbceffg") == false);
    assert!(is_valid("abbcegjk") == false);
    assert_eq!(next_password("abcdefgh"), "abcdffaa");
    assert_eq!(next_password("ghijklmn"), "ghjaabcc");
}
