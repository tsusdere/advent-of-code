use regex::Regex;

fn main() {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut total = 0;
    let data = std::fs::read_to_string("input.txt").expect("Unable to read file");

    for m in pattern.captures_iter(&data) {
        if let Some(cap) = m.get(1) {
            let a = cap.as_str().parse::<i32>().unwrap();
            if let Some(cap) = m.get(2) {
                let b = cap.as_str().parse::<i32>().unwrap();
                total += a * b;
            }
        }
    }
    println!("Total: {}", total);
}
