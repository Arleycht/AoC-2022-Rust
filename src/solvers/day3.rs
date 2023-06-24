use crate::util;

pub fn solve() {
    let input = util::read_input(3, false);

    let mut priority_sum_1 = 0;

    let mut current_group: Vec<i64> = Vec::new();
    let mut priority_sum_2 = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mid = chars.len() / 2;

        let mut mask_a = 0i64;
        let mut mask_b = 0i64;

        for i in 0..mid {
            let a = chars[i];
            let b = chars[i + mid];

            let p_a = get_priority(&a);
            let p_b = get_priority(&b);

            let a = 1 << (p_a - 1);
            let b = 1 << (p_b - 1);

            mask_a |= a;
            mask_b |= b;
        }

        let intersection = mask_a & mask_b;

        if intersection != 0 {
            let priority = (intersection & (!intersection + 1))
                .trailing_zeros() + 1;

            priority_sum_1 += priority;
        }

        current_group.push(mask_a | mask_b);

        if current_group.len() == 3 {
            let group_mask = current_group.iter()
                .fold(!0i64, |acc, x| acc & x);
            
            println!("{}", group_mask);

            if group_mask != 0 {
                let priority = (group_mask & (!group_mask + 1))
                    .trailing_zeros() + 1;
                priority_sum_2 += priority;
            }

            current_group.clear();
        }
    }

    println!("Day 3: {}, {}", priority_sum_1, priority_sum_2);
}

fn get_priority(c: &char) -> i32 {
    if !c.is_alphabetic() {
        panic!("Invalid input");
    }

    if c.is_lowercase() {
        (*c as i32) - ('a' as i32) + 1
    } else {
        (*c as i32) - ('A' as i32) + 27
    }
}
