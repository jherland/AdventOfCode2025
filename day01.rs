use std::io;

fn parse(line: &String) -> i32 {
    let dir: i32 = match line.chars().nth(0) {
        Some('L') => -1,
        Some('R') => 1,
        _ => panic!()
    };
    dir * line[1..].parse::<i32>().unwrap()
}

fn rotate_and_count_zeroes(rotations: impl Iterator<Item = i32>, init: i32) -> usize
{
    rotations
        .scan(init, |acc, rot| {
            *acc = (*acc + rot) % 100;
            Some(*acc)
        })
        .filter(|acc| *acc == 0)
        .count()
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let rotations: Vec<i32> = lines
        .into_iter()
        .map(|line| parse(&line))
        .collect::<Vec<_>>();
    println!("Part 1: {}", rotate_and_count_zeroes(rotations.clone().into_iter(), 50));

    let clicks = rotations
        .iter()
        .map(|rot| std::iter::repeat_n(
            if *rot >= 0 { 1 } else { -1 },
            rot.abs().try_into().unwrap()
        ))
        .flatten();
    println!("Part 2: {}", rotate_and_count_zeroes(clicks, 50));
}
