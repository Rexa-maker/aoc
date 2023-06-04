fn main() {
    let input = include_str!("input");
    println!(
        "{} {}",
        input
            .lines()
            .fold(0, |tally, x| if is_nice(x) { tally + 1 } else { tally }),
        0
    );
}

fn is_nice(input: &str) -> bool {
    for forbidden in ["ab", "cd", "pq", "xy"] {
        if input.contains(forbidden) {
            return false;
        }
    }

    let mut vowels_met = 0;
    let mut met_repeat = false;
    let mut last = '\0';
    for c in input.chars() {
        if last == c {
            met_repeat = true;
        }
        last = c;

        if "aeiou".contains(c) {
            vowels_met += 1;
        }
    }

    met_repeat && vowels_met >= 3
}

#[test]
fn examples() {
    assert_eq!(is_nice("ugknbfddgicrmopn"), true);
    assert_eq!(is_nice("aaa"), true);
    assert_eq!(is_nice("jchzalrnumimnmhp"), false);
    assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
    assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
}
