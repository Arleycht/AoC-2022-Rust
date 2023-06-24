use crate::util;

pub fn solve() {
    let input = util::read_input(2, false);

    let mut score_1 = 0;
    let mut score_2 = 0;

    for line in input.lines() {
        let (a, b) = get_choices(line);
        
        score_1 += b + 1;

        if a == b {
            score_1 += 3; // Draw
        } else if b == (a + 1).rem_euclid(3) {
            score_1 += 6; // Win
        }

        match b {
            0 => {
                // Must lose
                score_2 += (a - 1).rem_euclid(3) + 1;
            },
            1 => {
                // Must draw
                score_2 += a + 4;
            },
            2 => {
                // Must win
                score_2 += (a + 1).rem_euclid(3) + 7;
            },
            _ => panic!("Invalid input"),
        }
    }

    println!("Day 2: {}, {}", score_1, score_2);
}

fn to_choice(c: char) -> i32 {
    match c {
        'A' | 'X' => 0,
        'B' | 'Y' => 1,
        'C' | 'Z' => 2,
        _ => panic!("Invalid input"),
    }
}

fn get_choices(line: &str) -> (i32, i32) {
    let mut line = line.split_whitespace();
    let a = to_choice(line.next().unwrap().chars().next().unwrap());
    let b = to_choice(line.next().unwrap().chars().next().unwrap());
    (a, b)
}
