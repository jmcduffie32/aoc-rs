use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("day_1_input.txt")?;
    let mut total_fuel: i32 = 0;
    for line in io::BufReader::new(file).lines() {
        total_fuel += calculate_fuel(line?.parse::<i32>().unwrap());
    }
    println!("{}", total_fuel);
    Ok(())
}

fn calculate_fuel(mass: i32) -> i32 {
    match mass {
        0 => mass,
        _ => {
            let fuel = mass / 3 - 2;
            if fuel >= 0 {
                fuel + calculate_fuel(fuel)
            } else {
                0
            }
        }
    }
}
