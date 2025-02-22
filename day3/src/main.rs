use regex::Regex;
use std::fs;

fn find_mul(data: &str) -> Vec<i32> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut mul_table = Vec::new();

    for capture in regex.captures_iter(&data) {
        if let (Some(first), Some(second)) = (capture.get(1), capture.get(2)) {
            let num1: i32 = first.as_str().parse().unwrap();
            let num2: i32 = second.as_str().parse().unwrap();
            mul_table.push(num1 * num2);
        }
    }
    mul_table
}

fn main() {
    let corrupt_data = fs::read_to_string("./src/corrupt_input.txt").expect("File not found");
    let mul_data = find_mul(&corrupt_data);

    println!("{:?}", mul_data);
}
