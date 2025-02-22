use std::fs;

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

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
        // Expect strictly decreasing values with difference in -3..=-1
        for pair in report.windows(2) {
            let diff = pair[1] - pair[0];
            if !(-3..=-1).contains(&diff) {
                return false;
            }
        }
    }
    true
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }
    // Try removing one level at a time.
    for i in 0..report.len() {
        let new_report: Vec<i32> = report[..i]
            .iter()
            .chain(report[i + 1..].iter())
            .copied()
            .collect();
        if new_report.len() >= 2 && is_safe(&new_report) {
            return true;
        }
    }
    false
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
    println!(
        "Number of safe reports: {}",
        v.iter().filter(|r| is_safe(r)).count()
    );

    println!(
        "Number of safe reports (w/Dampener): {}",
        v.iter()
            .filter(|report| is_safe_with_dampener(report))
            .count()
    );
}
