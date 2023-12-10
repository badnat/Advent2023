#![allow(clippy::needless_return)]

// ========================================================================

use std::collections::HashSet;

fn main() {
    let games = include_str!("../input/part1")
        .trim()
        .split('\n')
        .collect::<Vec<&str>>();

    let numbers: Vec<Vec<&str>> = games
        .into_iter()
        .map(|s| s.split(':').collect::<Vec<&str>>())
        .map(|s| *(s.get(1).unwrap_or(&"")))
        .map(|s| s.split('|').collect::<Vec<&str>>())
        .collect();

    let mut sum: u32 = 0;

    for n in numbers {
        let winning_numbers: HashSet<u32> = n
            .first()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let our_numbers: HashSet<u32> = n
            .last()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let exp: u32 = winning_numbers
            .intersection(&our_numbers)
            .count()
            .try_into()
            .unwrap();

        let mut score: u32 = 0;
        let two: u32 = 2;

        if exp > 0 {
            score = two.pow(exp - 1);
        }
        sum += score;
    }

    println!("{sum}");
}

// ========================================================================
