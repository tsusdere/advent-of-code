use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    iterate_input(contents);
}

fn iterate_input(input: String) {
    let mut vec: Vec<i32> = Vec::new();
    let mut increaing = true;
    let mut decreasing = false;

    let mut prev = None;
    for line in input.lines() {
        for num in line.split_whitespace() {
            vec.push(num.parse().unwrap());
        }
    }
}
