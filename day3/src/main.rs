use regex::Regex;
use std::fs;

fn find_mul(data: &str) -> Vec<i32> {
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();
    let mut mul_table = Vec::new();
    let mut enabled: bool;

    for mul_match in mul_pattern.captures_iter(data) {
        let mul_start = mul_match.get(0).unwrap().start();
        let preceding_data = &data[..mul_start];

        let last_do = do_pattern.find_iter(preceding_data).last();
        let last_dont = dont_pattern.find_iter(preceding_data).last();

        enabled = is_mul_enabled(last_do, last_dont);

        if enabled {
            if let (Some(first), Some(second)) = (mul_match.get(1), mul_match.get(2)) {
                let num1: i32 = first.as_str().parse().unwrap();
                let num2: i32 = second.as_str().parse().unwrap();
                mul_table.push(num1 * num2);
            }
        }
    }
    mul_table
}

fn is_mul_enabled(last_do: Option<regex::Match>, last_dont: Option<regex::Match>) -> bool {
    match (last_do, last_dont) {
        (None, None) => true,     // No do or don't, so enabled
        (Some(_), None) => true,  // Do, and no don't, so enabled
        (None, Some(_)) => false, // Don't, so disabled
        (Some(do_match), Some(dont_match)) => do_match.start() > dont_match.start(), // Which came last?
    }
}

fn main() {
    let corrupt_data = fs::read_to_string("./src/corrupt_input.txt").expect("File not found");
    let mul_data = find_mul(&corrupt_data);
    println!("{}", mul_data.iter().sum::<i32>());
}
