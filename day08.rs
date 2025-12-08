use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;
use std::ops::Sub;

use counter::Counter;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn parse(line: &str) -> Self {
        let (x, y, z) = line.split(',').map(|word| word.parse::<i64>().unwrap()).collect_tuple().unwrap();
        Self { x, y, z }
    }

    fn len(&self) -> i64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).isqrt()
    }

    fn dist(&self, other: &Self) -> i64 {
        (*other - *self).len()
    }

    fn ordered<'a>(a: &'a Self, b: &'a Self) -> (&'a Self, &'a Self) {
        if *a < *b { (a, b) } else { (b, a) }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BoxPair {
    rev_dist: Reverse<i64>,
    a: Pos,
    b: Pos,
}

impl BoxPair {
    fn create(aa: Pos, bb: Pos) -> Self {
        let (a, b) = Pos::ordered(&aa, &bb);
        Self { rev_dist: Reverse(a.dist(b)), a: *a, b: *b }
    }
}

fn connect(circuits: &mut HashMap<Pos, usize>, pair: BoxPair) -> Option<usize> {
    let src = circuits[&pair.a];
    let dst = circuits[&pair.b];
    if src == dst { return None; } // already connected
    let mut counter: Counter<usize> = Counter::new();
    for circuit in circuits.values_mut() {
        if *circuit == src { *circuit = dst; }
        counter[circuit] += 1;
    }
    Some(counter.len())
}

fn main() {
    let boxes: Vec<Pos> = io::stdin().lines().map(Result::unwrap).map(|line| Pos::parse(&line)).collect();
    let mut circuits: HashMap<Pos, usize> = boxes.iter().enumerate().map(|(i, pos)| (*pos, i)).collect();
    let mut min_dist: BinaryHeap<BoxPair> = BinaryHeap::new();
    for (i, a) in boxes[..boxes.len() - 1].iter().enumerate() {
        for b in boxes[i + 1..].iter() {
            let pair = BoxPair::create(*a, *b);
            min_dist.push(pair);
        }
    }

    for _ in 0..1000 {
        connect(&mut circuits, min_dist.pop().unwrap());
    }
    let counts: Counter<usize> = circuits.values().cloned().collect();
    println!("Part 1: {}", counts.k_most_common_ordered(3).into_iter().map(|(_, count)| count).product::<usize>()); // 66640

    let part2_answer = loop {
        let pair = min_dist.pop().unwrap();
        if connect(&mut circuits, pair) == Some(1) {
            break pair.a.x * pair.b.x;
        }
    };
    println!("Part 2: {}", part2_answer); // 78894156
}
