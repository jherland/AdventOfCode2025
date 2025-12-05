use std::cmp::max;
use std::io;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn parse(line: &str) -> Self {
        let (start, end) = line.split_at(line.find('-').unwrap());
        Self {
            start: start.parse::<usize>().unwrap(),
            end: end[1..].parse::<usize>().unwrap(),
        }
    }

    fn contains(&self, n: usize) -> bool {
        n >= self.start && n <= self.end
    }

    fn combine(&self, other: &Self) -> Option<Self> {
        assert!(self.start <= other.start);
        if other.start > self.end + 1 {
            None
        }
        else {
            Some(Self { start: self.start, end: max(self.end, other.end) })
        }
    }

    fn size(&self) -> usize {
        self.end + 1 - self.start
    }
}

#[derive(Debug)]
struct World {
    ranges: Vec<Range>,
    ingredients: Vec<usize>,
}

impl World {
    fn parse(lines: impl Iterator<Item = String>) -> Self
    {
        let mut ranges: Vec<Range> = Vec::new();
        let mut ingredients: Vec<usize> = Vec::new();
        let mut ranges_done = false;
        for line in lines {
            if !ranges_done {
                if line.trim() == "" {
                    ranges_done = true;
                }
                else {
                    ranges.push(Range::parse(line.trim()));
                }
            }
            else {
                ingredients.push(line.trim().parse().unwrap());
            }
        }
        Self { ranges, ingredients }
    }

    fn in_range(&self, n: usize) -> bool {
        for range in self.ranges.iter() {
            if range.contains(n) { return true; }
        }
        false
    }

    fn optimized(&self) -> Self {
        let mut sorted_ranges = self.ranges.clone();
        sorted_ranges.sort();
        let mut collapsed_ranges: Vec<Range> = Vec::new();
        let mut cur = sorted_ranges[0];
        for next in sorted_ranges[1..].iter() {
            match cur.combine(next) {
                None => {
                    collapsed_ranges.push(cur);
                    cur = *next;
                }
                Some(new_range) => {
                    cur = new_range;
                }
            }
        }
        collapsed_ranges.push(cur);
        Self { ranges: collapsed_ranges, ingredients: self.ingredients.clone() }
    }
}

fn main() {
    let world = World::parse(io::stdin().lines().map(Result::unwrap));
    println!("Part 1: {}", world.ingredients.iter().filter(|n| world.in_range(**n)).count()); // 511

    let world2 = world.optimized();
    println!("Part 2: {}", world2.ranges.iter().map(|r| r.size()).sum::<usize>()); // 350939902751909
}
