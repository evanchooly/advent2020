use std::fs;

fn main() {
    read_surveys("input/sample.input");
    read_surveys("input/actual.input");
}

fn read_surveys(input: &str) {
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    let mut answers: [i32; 26] = [0; 26];
    let mut count = 0;
    let mut group_size = 0;

    for line in contents.lines() {
        if line != "" {
            group_size += 1;
            for letter in line.chars() {
                let index = letter as u32 - 'a' as u32;
                answers[index as usize] += 1;
            }
        } else {
            for response in answers.iter() {
                if *response == group_size {
                    count += 1
                }
            }
            answers = [0; 26];
            group_size = 0;
        }
    }

    println!("sum : {}", count);
}
