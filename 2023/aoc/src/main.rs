use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(ip) = line {
                let numbers = extract_numbers(&ip);
                // get the first number
                let first = numbers[0];
                // get the seond number if it exists else set it to 0
                let second = if numbers.len() > 1 { numbers[1] } else { 0 };

                if second == 0 {
                    total += first * 11;
                } else {
                    let num = first.to_string() + &second.to_string();
                    total += num.parse::<i32>().unwrap();
                }
            }
        }
    }
    println!("Total: {}", total);
}

// function to read line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// function to extract numbers
fn extract_numbers(input: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            numbers.push(digit as i32);
        }
    }
    numbers
}
