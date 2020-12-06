use std::cmp::max;
use std::collections::BTreeSet;
use std::fs;

fn main() {
    read_passes("input/actual.input");
}

fn read_passes(input: &str) {
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    let mut max_id = -100;
    let mut ids = BTreeSet::new();

    for line in contents.lines() {
        if line != "" {
            let (_row, _column, seat_id) = process(line);
            max_id = max(max_id, seat_id);
            ids.insert(seat_id);
        }
    }

    let mut last_id = -1;
    let mut missing_id = -1;

    for id in ids {
        if (last_id != -1) && (id != (last_id + 1)) {
            missing_id = id - 1;
        }
        last_id = id
    }
    println!("max seat_id: {}, missing ID: {}", max_id, missing_id);
}

fn process(line: &str) -> (i32, i32, i32) {
    let mut row_range = (0, 127);
    let mut column_range = (0, 7);
    let mut row = 0;
    let mut column = 0;

    for letter in line.chars().collect::<Vec<char>>() {
        match letter {
            'F' => {
                row_range = lower(row_range);
                row = row_range.0
            }
            'B' => {
                row_range = upper(row_range);
                row = row_range.1
            }
            'L' => {
                column_range = lower(column_range);
                column = column_range.0;
            }
            'R' => {
                column_range = upper(column_range);
                column = column_range.1;
            }
            _ => {}
        }
    }

    return (row, column, row * 8 + column)
}

fn upper(range: (i32, i32)) -> (i32, i32) {
    return (mid(range) + range.0 + 1, range.1);
}

fn lower(range: (i32, i32)) -> (i32, i32) {
    return (range.0, range.1 - mid(range) - 1);
}

fn mid(range: (i32, i32)) -> i32 {
    (range.1 - range.0) / 2
}