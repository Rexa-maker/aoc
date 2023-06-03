fn main() {
    let input = include_str!("input");
    let data = input_to_data(input);
    println!("{} {}", fuel_from_masses(&data), total_fuel(&data));
}

fn input_to_data(input: &str) -> Vec<u32> {
    input.lines().map(|val| val.parse().unwrap()).collect()
}

fn fuel_from_mass(data: u32) -> u32 {
    let div_3: u32 = data / 3;
    if div_3 < 2 {
        0
    } else {
        div_3 - 2
    }
}

fn fuel_from_masses(data: &Vec<u32>) -> u32 {
    data.iter()
        .fold(0, |tally: u32, x: &u32| tally + fuel_from_mass(*x))
}

fn fuel_from_fuel(fuel: u32) -> u32 {
    let mut total_fuel = 0;
    let mut fuel_to_fuel = fuel;

    loop {
        fuel_to_fuel = fuel_from_mass(fuel_to_fuel);
        if fuel_to_fuel == 0 {
            break;
        }
        total_fuel += fuel_to_fuel;
    }

    total_fuel
}

fn total_fuel(data: &Vec<u32>) -> u32 {
    let mut total_fuel = 0;

    for mass in data {
        let fuel = fuel_from_mass(*mass);
        total_fuel += fuel + fuel_from_fuel(fuel);
    }

    total_fuel
}

#[test]
fn examples() {
    assert_eq!(fuel_from_mass(12), 2);
    assert_eq!(fuel_from_mass(14), 2);
    assert_eq!(fuel_from_mass(1969), 654);
    assert_eq!(fuel_from_mass(100756), 33583);

    assert_eq!(total_fuel(&input_to_data("14")), 2);
    assert_eq!(total_fuel(&input_to_data("1969")), 966);
    assert_eq!(total_fuel(&input_to_data("100756")), 50346);
}
