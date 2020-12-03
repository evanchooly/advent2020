use std::fs;

fn main() {
    day1();
}

fn day1() {
    let contents = fs::read_to_string("input/day1.input")
        .expect("Something went wrong reading the file");
    let numbers: Vec<i32> = contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("Two numbers: {}",  numbers[i] * numbers[j]);
            }
        }
    }

    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            for k in (j + 1)..numbers.len() {
                if numbers[i] + numbers[j]  + numbers[k] == 2020 {
                    println!("Three numbers: {}", numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }
}