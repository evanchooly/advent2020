use std::fs;

fn main() {
    let right = 3;
    let down = 1;

    println!("Sample data hits:  {}\n", plot_course("input/sample.input", right, down));
    println!("Full data hits:  {}\n", plot_course("input/day3.input", right, down));

    println!("Sample data product:  {}\n", compute_product("input/sample.input"));
    println!("Full data product:  {}", compute_product("input/day3.input"));
}

fn compute_product(input: &str) -> i64 {
    let routes: [(i32, i32); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product: i64 = 1;
    for route in routes.iter() {
        product *= plot_course(input, route.0, route.1);
    }
    product
}

fn plot_course(input: &str, right: i32, down: i32) -> i64 {
    let mut trees: i64 = 0;
    let mut line = 0;
    let mut position: i32 = 0;

    for row in read_map(input) {
        if line % down == 0 {
            if row[position as usize] == '#' {
                trees += 1
            }
            position += right;
            position %= row.len() as i32
        }
        line += down;
    }

    // println!("trees: {}", trees);
    trees
}

fn read_map(input: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    let mut map: Vec<Vec<char>> = vec![];
    for line in contents.lines() {
        map.push(line.chars().collect());
    }


    map
}