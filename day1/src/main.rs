use std::fs;

fn main() {
    let mut right_list: Vec<i32> = vec![];
    let mut left_list: Vec<i32> = vec![];
    let mut discrepancy_list: Vec<i32> = vec![];
    let file_content = fs::read_to_string("list.txt").expect("Failed to open file");

    for line in file_content.lines() {
        let mut tokens = line.split_whitespace();
        let left: i32 = tokens.next().unwrap().parse().unwrap();
        let right: i32 = tokens.next().unwrap().parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    for (left, right) in left_list.iter().zip(right_list.iter()) {
        discrepancy_list.push((left - right).abs());
    }
    let _sum = discrepancy_list.iter().sum::<i32>();

    // -- Pt. 2 ---
    let mut similarity_score = vec![];

    for left in left_list {
        if right_list.contains(&left) {
            let count: i32 = right_list
                .iter()
                .filter(|&x| x == &left)
                .count()
                .try_into()
                .unwrap();
            similarity_score.push(&left * count);
        }
    }

    let siml_total = similarity_score.iter().sum::<i32>();
    println!("{:?}", siml_total);
}
