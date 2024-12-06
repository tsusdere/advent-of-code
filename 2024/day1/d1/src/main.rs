use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::vec;

fn main() {
    let contents =
        fs::read_to_string("../input.txt").expect("Should have been able toread the file");
    iterate_input(contents);
}

fn iterate_input(input: String) {
    let mut h1: BinaryHeap<i32> = BinaryHeap::new();
    let mut h2: BinaryHeap<i32> = BinaryHeap::new();

    // part 2
    let mut mappings: HashMap<i32, i32> = HashMap::new();
    let mut vec: Vec<i32> = Vec::new();
    // read the input line by line
    for line in input.lines() {
        // split the line by white space
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        if parts.len() != 2 {
            continue;
        }

        // check if the num is in the hashmap
        let num = parts[1].parse::<i32>().unwrap();
        vec.push(parts[0].parse::<i32>().unwrap());
        let count = mappings.entry(num).or_insert(0);
        *count += 1;

        // push into the heaps
        h1.push(parts[0].parse::<i32>().unwrap());
        h2.push(parts[1].parse::<i32>().unwrap());
    }

    calculate_ans(&mut h1, &mut h2);
    part2(&mappings, &vec)
}

fn calculate_ans(h1: &mut BinaryHeap<i32>, h2: &mut BinaryHeap<i32>) {
    let mut ans = 0;

    while !h1.is_empty() && !h2.is_empty() {
        let a = h1.pop().unwrap();
        let b = h2.pop().unwrap();
        ans += (a - b).abs();
    }
    print!("{}", ans);
}

fn part2(map: &HashMap<i32, i32>, vec: &Vec<i32>) {
    let mut ans = 0;

    for num in vec {
        if map.contains_key(&num) {
            ans += num * map.get(&num).unwrap();
        }
    }

    print!("Part 2 : {}", ans);
}
