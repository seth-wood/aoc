use std::fs;

fn is_safe(report: &[i32]) -> bool {
    let first_diff = report[1] - report[0];
    if first_diff == 0 {
        return false;
    }

    if first_diff > 0 {
        // Expect strictly increasing values with difference in 1..=3
        for pair in report.windows(2) {
            let diff = pair[1] - pair[0];
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
    } else {
        // Expect strictly decreasing values with difference in -2..=-1
        for pair in report.windows(2) {
            let diff = pair[1] - pair[0];
            if !(-3..=-1).contains(&diff) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let content = fs::read_to_string("./src/input.txt").expect("File not found");
    let mut v: Vec<Vec<i32>> = vec![];

    for line in content.lines() {
        let reports: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        v.push(reports);
    }
    println!("{}", v.iter().filter(|r| is_safe(r)).count());

    let safe_reports_count = v.iter().filter(|report| is_safe(report)).count();
    println!("Number of safe reports: {}", safe_reports_count);
}
