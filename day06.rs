use itertools::Itertools;
use std::iter::zip;
use std::io;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s.chars().next().unwrap() {
            '+' => Self::Add,
            '*' => Self::Mul,
            _ => panic!("Unrecognized operator '{}'!", s),
        }
    }

    fn apply(&self, nums: Vec<usize>) -> usize {
        let func = match self {
            Self::Add => |a: usize, b: usize| a + b,
            Self::Mul => |a: usize, b: usize| a * b,
        };
        nums.into_iter().reduce(func).unwrap()
    }
}

fn main() {
    let mut lines: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();
    let ops: Vec<Op> = lines.pop().unwrap().split_whitespace().map(|word| Op::parse(word)).collect();

    let cols: Vec<Vec<usize>> = transpose(
        lines
            .iter()
            .map(|line| line.split_whitespace().map(|word| word.parse::<usize>().unwrap()).collect())
            .collect()
    );
    println!("Part 1: {}", zip(cols, ops.clone()).map(|(col, op)| op.apply(col)).sum::<usize>()); // 6757749566978

    let cols2: Vec<Vec<usize>> = transpose(lines.iter().map(|line| line.chars().collect()).collect())
        .into_iter()
        .map(|chars| chars.into_iter().collect::<String>())
        .chunk_by(|s| s.trim() != "")
        .into_iter()
        .filter(|(non_empty, _)| *non_empty)
        .map(|(_, group)| group.map(|s| s.trim().parse().unwrap()).collect())
        .collect();
    println!("Part 2: {}", zip(cols2, ops).map(|(col, op)| op.apply(col)).sum::<usize>()); // 6757749566978
}
