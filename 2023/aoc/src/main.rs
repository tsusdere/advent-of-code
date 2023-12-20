use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input2").unwrap();
    part1(&input);
}

fn part1(input: &str) -> i32 {
    for line in input.lines() {
        let game = &line[0..line.find(':').unwrap_or(line.len())];
        let parts: Vec<&str> = game.split(' ').collect();
        if let Some(last_part) = parts.last() {
            if let Ok(number) = last_part.parse::<i32>() {
                println!("{}", number);
            }
        }
    }
    0
}
