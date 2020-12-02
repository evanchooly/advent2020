use std::fs;

fn main() {
    // let array = [1721, 979, 366, 299, 675, 1456];

    let contents = fs::read_to_string("day1/input")
        .expect("Something went wrong reading the file");

    println!("Two numbers: {}", sum2(contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()));

    println!("Three numbers: {}", sum3(contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()));
}

fn sum2(numbers: Vec<i32>) -> i32 {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return numbers[i] * numbers[j];
            }
        }
    }

    return -1;
}

fn sum3(numbers: Vec<i32>) -> i32 {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            for k in (j + 1)..numbers.len() {
                if numbers[i] + numbers[j]  + numbers[k] == 2020 {
                    return numbers[i] * numbers[j] * numbers[k];
                }
            }
        }
    }

    return -1;
}