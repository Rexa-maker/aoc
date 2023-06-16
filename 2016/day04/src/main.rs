use regex;

fn main() {
    let input = include_str!("input");
    println!("{} {}", sum_of_sector_ids(input), find_north_pole_object_storage(input));
}

fn sum_of_sector_ids(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (name, sector_id, checksum) = parse_line(line);
            if is_real_room(name, checksum) {
                sector_id
            } else {
                0
            }
        })
        .sum()
}

fn parse_line(line: &str) -> (String, u32, String) {
    let re = regex::Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();
    let caps = re.captures(line).unwrap();
    (
        caps[1].to_string(),
        caps[2].parse::<u32>().unwrap(),
        caps[3].to_string(),
    )
}

fn is_real_room(name: String, checksum: String) -> bool {
    let mut letters = name
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();
    letters.sort();
    letters.dedup();

    let mut counts = Vec::new();
    for letter in letters {
        counts.push((letter, name.matches(letter).count()));
    }

    counts.sort_by(|a, b| 
        // Largest count first, then alphabetically,
        // meaning that the sort is in reverse order for the count only
        b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let mut calculated_checksum = String::new();
    for (letter, _) in counts.iter().take(5) {
        calculated_checksum.push(*letter);
    }

    checksum == calculated_checksum
}

fn decipher_room(name: String, index: u32) -> String {
    let mut deciphered = String::new();
    for c in name.chars() {
        if c == '-' {
            deciphered.push(' ');
        } else {
            static A: u32 = 'a' as u32;
            deciphered.push(
                ((c as u32 - A + index) % 26 + A) as u8 as char
            );
        }
    }
    deciphered
}

fn find_north_pole_object_storage(input: &str) -> u32 {
    for line in input.lines() {
        let (name, sector_id, checksum) = parse_line(line);
        if is_real_room(name.clone(), checksum.clone()) {
            let deciphered = decipher_room(name, sector_id);
            if deciphered.contains("north") {
                return sector_id;
            }
        }
    }
    
    0
}

#[test]
fn examples() {
    assert_eq!(
        parse_line("aaaaa-bbb-z-y-x-123[abxyz]"),
        ("aaaaa-bbb-z-y-x".to_string(), 123, "abxyz".to_string())
    );
    assert_eq!(
        parse_line("a-b-c-d-e-f-g-h-987[abcde]"),
        ("a-b-c-d-e-f-g-h".to_string(), 987, "abcde".to_string())
    );
    assert_eq!(
        parse_line("not-a-real-room-404[oarel]"),
        ("not-a-real-room".to_string(), 404, "oarel".to_string())
    );
    assert_eq!(
        parse_line("totally-real-room-200[decoy]"),
        ("totally-real-room".to_string(), 200, "decoy".to_string())
    );
    assert_eq!(
        is_real_room("aaaaa-bbb-z-y-x".to_string(), "abxyz".to_string()),
        true
    );
    assert_eq!(
        is_real_room("a-b-c-d-e-f-g-h".to_string(), "abcde".to_string()),
        true
    );
    assert_eq!(
        is_real_room("not-a-real-room".to_string(), "oarel".to_string()),
        true
    );
    assert_eq!(
        is_real_room("totally-real-room".to_string(), "decoy".to_string()),
        false
    );

    assert!(decipher_room("qzmt-zixmtkozy-ivhz".to_string(), 343).contains("very encrypted name"));
}
