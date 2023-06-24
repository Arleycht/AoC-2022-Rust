use crate::util;

pub fn solve() {
    let input = util::read_input(5, false);

    let stack_count = (input.lines().next().unwrap().len() + 1) / 4;

    let mut stacks_1: Vec<Vec<char>> = vec![vec![]; stack_count];
    let mut stacks_2 = stacks_1.clone();
    let mut processing_stacks = true;

    let mut result_1 = String::new();
    let mut result_2 = String::new();

    for line in input.lines() {
        if line.is_empty() {
            processing_stacks = false;

            for i in 0..stack_count {
                stacks_1[i].reverse();
                stacks_2[i].reverse();
            }

            continue;
        }

        if processing_stacks {
            let mut chars = line.chars();
            chars.next();
            for i in 0..stack_count {
                let c = chars.next().unwrap();

                if c.is_alphabetic() {
                    stacks_1[i].push(c);
                    stacks_2[i].push(c);
                }

                chars.nth(2);
            }
        } else {
            let args = line.split(' ').collect::<Vec<_>>();

            let n: usize = args[1].parse().unwrap();
            let a = args[3].parse::<usize>().unwrap() - 1;
            let b = args[5].parse::<usize>().unwrap() - 1;

            let m = stacks_2[a].len() - n;

            for _ in 0..n {
                let item = stacks_1[a].pop().unwrap();
                stacks_1[b].push(item);

                let item = stacks_2[a].remove(m);
                stacks_2[b].push(item);
            }
        }
    }

    for i in 0..stack_count {
        result_1.push(*stacks_1[i].last().unwrap());
        result_2.push(*stacks_2[i].last().unwrap());
    }

    println!("Day 5: {}, {}", result_1, result_2);
}
