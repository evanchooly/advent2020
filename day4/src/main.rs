use substring::Substring;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Sample passports: {}", read_passports("input/sample.input"));
    println!("Invalid sample passports: {}", read_passports("input/sample-invalid.input"));
    println!("Valid sample passports: {}", read_passports("input/sample-valid.input"));
    println!("Valid actual passports: {}", read_passports("input/actual.input"));
}

fn validate(map: &HashMap<String, String>) -> bool {
    let mut valid = map.len() == 8 || (!map.contains_key("cid") && map.len() == 7);
    if valid {
        for (key, value) in map.iter() {
            match key.as_str() {
                "byr" => {
                    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
                    // assert!(re.is_match("2014-01-01"));
                    // (Birth Year) - four digits; at least 1920 and at most 2002.
                    let year = value.parse::<i32>().unwrap();
                    valid &= year >= 1920 && year <= 2002;
                }
                "iyr" => {
                    // (Issue Year) - four digits; at least 2010 and at most 2020.
                    let year = value.parse::<i32>().unwrap();
                    valid &= year >= 2010 && year <= 2020;
                }
                "eyr" => {
                    //  (Expiration Year) - four digits; at least 2020 and at most 2030.
                    let year = value.parse::<i32>().unwrap();
                    valid &= year >= 2020 && year <= 2030;
                }
                "hgt" => {
                    // (Height) - a number followed by either cm or in:
                    // If cm, the number must be at least 150 and at most 193.
                    // If in, the number must be at least 59 and at most 76.
                    if value.ends_with("in") {
                        let number: i32 = value.substring(0, value.len() - 2).parse().unwrap();
                        valid &= number >= 59 && number <= 76;
                    } else if value.ends_with("cm") {
                        let number: i32 = value.substring(0, value.len() - 2).parse().unwrap();
                        valid &= number >= 150 && number <= 193;
                    } else {
                        valid = false;
                    }
                }
                "hcl" => {
                    //  (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                    valid &= re.is_match(value);
                }
                "ecl" => {
                    //  (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                    let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    valid &= colors.contains(&value.as_str());
                }
                "pid" => {
                    //  (Passport ID) - a nine-digit number, including leading zeroes.
                    let re = Regex::new(r"^[0-9]{9}$").unwrap();
                    valid &= re.is_match(value);
                }
                "cid" => {
                    //  (Country ID) - ignored, missing or not.
                }
                _ => valid = false
            }
        }
    }
    return valid;
}

fn read_passports(input: &str) -> i32 {
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    let mut vec: Vec<HashMap<String, String>> = vec![];
    let mut map = HashMap::new();

    for line in contents.lines() {
        if line != "" {
            let items = line.split(" ").collect::<Vec<&str>>();
            for item in items {
                let entry = item.split(":").collect::<Vec<&str>>();
                map.insert(entry[0].to_string(), entry[1].to_string());
            }
        } else {
            vec.push(map);
            map = HashMap::new();
        }
    }
    if !map.is_empty() {
        vec.push(map);
    }

    let mut valid = 0;

    for map in vec.iter() {
        if validate(map) {
            valid += 1;
        }
    }

    valid
}