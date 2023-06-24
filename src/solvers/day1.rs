use crate::util;

pub fn solve() {
    let input = util::read_input(1, false);

    let mut amounts = Vec::new();
    let mut current_amount = 0;

    for line in input.lines() {
        if line.is_empty() {
            amounts.push(current_amount);
            current_amount = 0;
        } else {
            current_amount += line.parse::<i32>().unwrap_or_default();
        }
    }

    if amounts.len() < 3 {
        panic!("Invalid input for Day 1");
    }

    amounts.sort_unstable();

    let max_amount = *amounts.last().unwrap();
    let sum_of_top_three: i32 = amounts.iter().rev().take(3).sum();

    println!("Day 1: {}, {}", max_amount, sum_of_top_three);
}
