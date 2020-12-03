use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day2.input")
        .expect("Something went wrong reading the file");
    let entries = contents
        .lines();

    let mut _count1 = 0;
    let mut count2 = 0;

    for line in entries {
        let split= line.split(" ").collect::<Vec<&str>>();
        let character = split[1].chars().collect::<Vec<char>>()[0];
        if valid1(String::from(split[0]), character, String::from(split[2])) {
            _count1 += 1;
        }
        if valid2(String::from(split[0]), character, String::from(split[2])) {
            count2 += 1;
        }
    }
    println!("policy 1 valid passwords: {}", _count1);
    println!("policy 2 valid passwords: {}", count2);
}

fn valid2(policy: String, character: char, password: String) -> bool {
    let split= policy.split("-").collect::<Vec<&str>>();
    let pos1: usize = split[0].parse::<usize>().unwrap() - 1;
    let pos2: usize = split[1].parse::<usize>().unwrap() - 1;

    let vec = password.chars().collect::<Vec<char>>();
    let first = vec[pos1].eq(&character);
    let second = vec[pos2].eq(&character);

    return first ^ second;
}

fn valid1(policy: String, character: char, password: String) -> bool {
    let mut count = 0;

    password.chars().for_each(|c| if c == character {
        count += 1
    });

    let split= policy.split("-").collect::<Vec<&str>>();

    return count >= split[0].parse().unwrap() &&
        count <= split[1].parse().unwrap();
}
