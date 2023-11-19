use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let checksum = checksum(input);
    println!("Checksum: {checksum}");

    let common = common_letters_in_prototype_boxes(input);
    println!("Common letters: {common}");
}

fn has_pairs_n_triples(line: &str) -> (bool, bool) {
    // Of course it'd be more efficient to use 26 bits out of a u32 but I'm lazy
    let mut hs: HashMap<char, u32> = HashMap::new();
    for c in line.chars() {
        hs.entry(c).and_modify(|value| *value += 1).or_insert(1);
    }

    let mut found_pair = false;
    let mut found_triple = false;
    for v in hs.values() {
        if *v == 2 {
            found_pair = true;
        } else if *v == 3 {
            found_triple = true;
        }
    }

    (found_pair, found_triple)
}

fn checksum(input: &str) -> u32 {
    let mut pairs = 0;
    let mut triples = 0;

    for line in input.lines() {
        let (found_pair, found_triple) = has_pairs_n_triples(line);
        if found_pair {
            pairs += 1
        }
        if found_triple {
            triples += 1
        }
    }

    pairs * triples
}

fn common_letters_in_prototype_boxes(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    for i in 0..lines.len() {
        'compare_with_loop: for j in i + 1..lines.len() {
            let mut diff = 0;
            let mut common = String::new();
            for (c1, c2) in lines[i].chars().zip(lines[j].chars()) {
                if c1 != c2 {
                    diff += 1;
                    if diff > 1 {
                        continue 'compare_with_loop;
                    }
                } else {
                    common.push(c1);
                }
            }
            if diff == 1 {
                return common;
            }
        }
    }
    panic!("No solution found");
}

#[test]
fn examples() {
    assert_eq!(has_pairs_n_triples("abcdef"), (false, false));
    assert_eq!(has_pairs_n_triples("bababc"), (true, true));
    assert_eq!(has_pairs_n_triples("abbcde"), (true, false));
    assert_eq!(has_pairs_n_triples("abcccd"), (false, true));
    assert_eq!(has_pairs_n_triples("aabcdd"), (true, false));
    assert_eq!(has_pairs_n_triples("abcdee"), (true, false));
    assert_eq!(has_pairs_n_triples("ababab"), (false, true));

    assert_eq!(
        checksum("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"),
        12
    );

    assert_eq!(
        common_letters_in_prototype_boxes("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"),
        "fgij"
    );
}
