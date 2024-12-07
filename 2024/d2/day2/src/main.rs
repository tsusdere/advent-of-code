use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    iterate_input(contents);
}

fn iterate_input(input: String) {
    for line in input.lines() {
        println!("{}", line);
    }
}
