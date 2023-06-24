use std::fmt::Display;

use crate::util;

pub fn solve() {
    let input = util::read_input(4, false);

    let mut either_count = 0;
    let mut overlap_count = 0;

    for line in input.lines() {
        let segments = line.split(',').collect::<Vec<_>>();
        assert!(segments.len() == 2, "Invalid input");
        let a = *segments.first().unwrap();
        let b = *segments.last().unwrap();

        let a = Section::from(a);
        let b = Section::from(b);

        if a.is_subset(&b) || b.is_subset(&a) {
            either_count += 1;
        }

        if a.overlaps(&b) {
            overlap_count += 1;
        }
    }

    println!("Day 4: {}, {}", either_count, overlap_count);
}

struct Section {
    start: i32,
    end: i32,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Section [{}, {}]", self.start, self.end)
    }
}

impl From<&str> for Section {
    fn from(item: &str) -> Self {
        let segments = item.split('-').collect::<Vec<_>>();
        assert!(segments.len() == 2, "Invalid input");
        if let Ok(start) = segments.first().unwrap().parse() {
            if let Ok(end) = segments.last().unwrap().parse() {
                return Section{ start, end };
            }
        }

        panic!("Invalid input");
    }
}

impl Section {
    fn is_subset(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.end >= other.start && self.start <= other.end
    }
}
