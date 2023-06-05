fn main() {
    let input = include_str!("input");
    println!(
        "{} {}",
        count_if(input.lines(), is_nice1),
        count_if(input.lines(), is_nice2)
    );
}

fn count_if<'a, I, F>(input: I, f: F) -> u32
where
    I: Iterator<Item = &'a str>,
    F: Fn(&str) -> bool,
{
    let mut count = 0;
    for line in input {
        if f(line) {
            count += 1;
        }
    }
    count
}

fn is_nice1(input: &str) -> bool {
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

fn is_nice2(input: &str) -> bool {
    let mut has_pair_twice = false;
    'outer: for i in 0..input.len() - 3 {
        let pair = &input[i..i + 2];
        for j in i + 2..input.len() - 1 {
            let other_pair = &input[j..j + 2];
            if pair == other_pair {
                has_pair_twice = true;
                break 'outer;
            }
        }
    }

    let mut repeat_with_one_between = false;
    for i in 0..input.len() - 2 {
        if &input[i..i + 1] == &input[i + 2..i + 3] {
            repeat_with_one_between = true;
            break;
        }
    }

    has_pair_twice && repeat_with_one_between
}

#[test]
fn examples() {
    assert_eq!(is_nice1("ugknbfddgicrmopn"), true);
    assert_eq!(is_nice1("aaa"), true);
    assert_eq!(is_nice1("jchzalrnumimnmhp"), false);
    assert_eq!(is_nice1("haegwjzuvuyypxyu"), false);
    assert_eq!(is_nice1("dvszwmarrgswjxmb"), false);

    assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_nice2("xxyxx"), true);
    assert_eq!(is_nice2("uurcxstgmygtbstg"), false);
    assert_eq!(is_nice2("ieodomkazucvgmuy"), false);
}
