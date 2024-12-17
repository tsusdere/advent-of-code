use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut ans = 0;
    if let Ok(lines) = read_lines("input.txt") {
        let mut vec: Vec<i32> = Vec::new();
        for line in lines.flatten() {
            let nums = line.split_whitespace();
            for num in nums {
                vec.push(num.parse().unwrap());
            }
            // check if the list is ascending or descending
            if (check_ascending(&vec) || check_descending(&vec)) || safe_removing_one(&vec) {
                ans += 1;
            }
            vec.clear();
        }
    }
    println!("{}", ans);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_ascending(vec: &Vec<i32>) -> bool {
    for i in 0..vec.len() - 1 {
        // make sure the difference is at least 1 and at most 3
        if vec[i + 1] <= vec[i] || (vec[i + 1] - vec[i] < 1 || vec[i + 1] - vec[i] > 3) {
            return false;
        }
    }
    true
}

fn check_descending(vec: &Vec<i32>) -> bool {
    for i in 0..vec.len() - 1 {
        // make sure the difference is at least 1 and at most 3
        if vec[i] <= vec[i + 1] || (vec[i] - vec[i + 1] < 1 || vec[i] - vec[i + 1] > 3) {
            return false;
        }
    }
    true
}

fn safe_removing_one(vec: &Vec<i32>) -> bool {
    for i in 0..vec.len() {
        let mut temp = vec.clone();
        temp.remove(i);
        if check_ascending(&temp) || check_descending(&temp) {
            return true;
        }
    }
    return false;
}
