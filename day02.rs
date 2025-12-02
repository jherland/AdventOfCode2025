use itertools::Itertools;
use std::cmp::max;
use std::io;
use std::io::Read;

fn in_range(num: usize, range: (usize, usize)) -> bool {
    num >= range.0 && num <= range.1
}

fn repeat_kernels_in_range(range: (usize, usize), min_len: usize) -> impl Iterator<Item = usize> {
    let start = range.0.to_string();
    let end = range.1.to_string();
    let max_len = max(min_len, start.len() / 2);
    let extend = end.len() - start.len();
    (min_len..=max_len)
        .map(move |len| {
            let start_kernel = &start[0..len].parse::<usize>().unwrap();
            let end_kernel = &end[0..len + extend].parse::<usize>().unwrap();
            (*start_kernel..=*end_kernel)
                .map(move |kernel| {
                    let k = kernel.to_string();
                    (len..=k.len()).map(move |l| k[0..l].parse::<usize>().unwrap())
                })
                .flatten()
        })
        .flatten()
}

fn repeat_kernel(kernel: usize, repeats: usize) -> usize {
    std::iter::repeat_n(kernel.to_string(), repeats).collect::<String>().parse::<usize>().unwrap()
}

fn repeated_twice_in_range(range: (usize, usize)) -> impl Iterator<Item = usize> {
    let start = range.0.to_string();
    repeat_kernels_in_range(range, max(1, start.len() / 2))
        .map(move |kernel| repeat_kernel(kernel, 2))
        .filter(move |n| in_range(*n, range))
        .unique()
}

fn all_repeats_in_range(range: (usize, usize)) -> impl Iterator<Item = usize> {
    let min_len = range.0.to_string().len();
    let max_len = range.1.to_string().len();
    repeat_kernels_in_range(range, 1)
        .map(move |kernel| {
            let k_len = kernel.to_string().len();
            (max(2, min_len / k_len)..=(max_len / k_len))
                .map(move |repeats| repeat_kernel(kernel, repeats))
        })
        .flatten()
        .filter(move |n| in_range(*n, range))
        .unique()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ranges: Vec<(usize, usize)> = input.trim().split(',').map(|s| {
        let (start, end) = s.split_at(s.find('-').unwrap());
        (start.parse::<usize>().unwrap(), end[1..].parse::<usize>().unwrap())
    }).collect();

    println!("Part 1: {}", ranges.iter().map(|range| repeated_twice_in_range(*range)).flatten().sum::<usize>());
    println!("Part 2: {}", ranges.iter().map(|range| all_repeats_in_range(*range)).flatten().sum::<usize>());
}
