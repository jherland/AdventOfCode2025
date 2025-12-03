use std::io;

fn parse_line(line: &String) -> Vec<u32> {
    line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn max_selection(bank: &Vec<u32>, len: usize) -> Vec<u32> {
    let mut j: usize = 0; // current start index in bank
    (0..len)
        .rev()
        .map(|i| {
            let max = bank[j..bank.len() - i].iter().max().unwrap();
            j = j + 1 + bank[j..].iter().position(|x| x == max).unwrap();
            *max
        })
        .collect()
}

fn joltage(selection: &Vec<u32>) -> u64 {
    selection.iter().fold(0u64, |acc, x| acc * 10 + (*x as u64))
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let banks: Vec<Vec<u32>> = lines.iter().map(|line| parse_line(line)).collect();
    println!("Part 1: {}", banks.iter().map(|bank| joltage(&max_selection(bank, 2))).sum::<u64>());
    println!("Part 2: {}", banks.iter().map(|bank| joltage(&max_selection(bank, 12))).sum::<u64>());
}
