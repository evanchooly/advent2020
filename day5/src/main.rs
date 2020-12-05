use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::fs;

fn main() {
    // read_passes("input/sample.input");
    read_passes("input/actual.input");
}

fn read_passes(input: &str) {
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    let mut max_id = -100;
    let mut min_row = 100000;
    let mut max_row = -100;
    let mut ids = BTreeSet::new();
    let open = ' ';
    let taken = 'X';
    let mut array: [[char; 8]; 127] = [[open; 8]; 127];

    for line in contents.lines() {
        if line != "" {
            let (row, column, seat_id) = process(line);
            array[row as usize][column as usize] = taken;
            max_id = max(max_id, seat_id);
            max_row = max(max_row, row);
            min_row = min(min_row, row);
            ids.insert(seat_id);
            // println!("line: {}, row : {} , column : {}, seat_id: {}", line, row, column, seat_id);
        }
    }

    let mut last_id = -1;

    for id in ids {
        if (last_id != -1) && (id != (last_id + 1)) {
            println!("missing id: {}", id)
        }
        last_id = id
    }
/*    for row in &array {
        for col in row {
            print!("{}", col)
        }
        println!();
    }
    println!();
*/
    println!("max seat_id: {}, max row: {}, min row: {}", max_id, max_row, min_row);
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
        // println!("letter : {}, row : ({}, {}) , column : ({}, {})", letter, row_range.0, row_range.1,
        //          column_range.0, column_range.1);
    }

    let seat_id = row * 8 + column;
    return (row, column, seat_id)
}

fn _show(name: &str, tuple: (i32, i32)) {
    println!("{}.0 : {}, {}.1: {}", name, tuple.0, name, tuple.1);
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