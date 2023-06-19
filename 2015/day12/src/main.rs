use json;

fn main() {
    static INPUT: &str = include_str!("input");
    println!("{} {}", json_sum(INPUT), json_sum_no_red(INPUT));
}

fn traverse_json(json: &json::JsonValue) -> i64 {
    match json {
        json::JsonValue::Object(obj) => obj
            .iter()
            .fold(0, |acc, (_, value)| acc + traverse_json(value)),
        json::JsonValue::Array(arr) => arr.iter().fold(0, |acc, value| acc + traverse_json(value)),
        _ => {
            if let Some(number) = json.as_i64() {
                number
            } else {
                0 // Strings don't count towards the sum
            }
        }
    }
}

fn json_sum(input: &str) -> i64 {
    let json = json::parse(input).unwrap();

    traverse_json(&json)
}

fn traverse_json_no_red(json: &json::JsonValue) -> i64 {
    match json {
        json::JsonValue::Object(obj) => {
            if obj.iter().any(|(_, value)| value == "red") {
                0
            } else {
                obj.iter()
                    .fold(0, |acc, (_, value)| acc + traverse_json_no_red(value))
            }
        }
        json::JsonValue::Array(arr) => arr
            .iter()
            .fold(0, |acc, value| acc + traverse_json_no_red(value)),
        _ => {
            if let Some(number) = json.as_i64() {
                number
            } else {
                0 // Strings don't count towards the sum
            }
        }
    }
}

fn json_sum_no_red(input: &str) -> i64 {
    let json = json::parse(input).unwrap();

    traverse_json_no_red(&json)
}

#[test]
fn examples() {
    assert_eq!(json_sum("[1,2,3]"), 6);
    assert_eq!(json_sum("{\"a\":2,\"b\":4}"), 6);
    assert_eq!(json_sum("[[[3]]]"), 3);
    assert_eq!(json_sum("{\"a\":{\"b\":4},\"c\":-1}"), 3);
    assert_eq!(json_sum("{\"a\":[-1,1]}"), 0);
    assert_eq!(json_sum("[-1,{\"a\":1}]"), 0);
    assert_eq!(json_sum("[]"), 0);
    assert_eq!(json_sum("{}"), 0);

    assert_eq!(json_sum_no_red("[1,2,3]"), 6);
    assert_eq!(json_sum_no_red("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
    assert_eq!(
        json_sum_no_red("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"),
        0
    );
    assert_eq!(json_sum_no_red("[1,\"red\",5]"), 6);
}
