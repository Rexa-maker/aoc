fn main() {
    static INPUT: &str = "1113122113";
    println!(
        "{} {}",
        look_and_say(INPUT, 40).len(),
        look_and_say(INPUT, 50).len()
    );
}

fn look_and_say(input: &str, iterations: usize) -> String {
    let mut output = String::from(input);

    for _ in 0..iterations {
        output = look_and_say_once(&output);
    }

    output
}

fn look_and_say_once(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars();
    let mut current = chars.next().unwrap();
    let mut count = 1;

    for c in chars {
        if c == current {
            count += 1;
        } else {
            output.push_str(&count.to_string());
            output.push(current);
            current = c;
            count = 1;
        }
    }
    output.push_str(&count.to_string());
    output.push(current);

    output
}

#[test]
fn examples() {
    assert_eq!(look_and_say("1", 1), "11");
    assert_eq!(look_and_say("1", 2), "21");
    assert_eq!(look_and_say("1", 3), "1211");
    assert_eq!(look_and_say("1", 4), "111221");
}
