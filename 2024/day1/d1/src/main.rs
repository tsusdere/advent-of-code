use std::fs;

fn main() {
    let contents =
        fs::read_to_string("../input.txt").expect("Should have been able toread the file");
    iterate_input(contents);
}

fn iterate_input(input: String) {
    println!("{}", input);
}
