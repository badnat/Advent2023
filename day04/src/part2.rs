#![allow(clippy::needless_return)]

use std::collections::HashSet;

// ========================================================================
// Parse Input File
fn main() {
    let games = include_str!("../input/part2")
        .trim()
        .split('\n')
        .collect::<Vec<&str>>();

    let numbers: Vec<Vec<&str>> = games
        .into_iter()
        .map(|s| s.split(':').collect::<Vec<&str>>())
        .map(|s| *(s.get(1).unwrap_or(&"")))
        .map(|s| s.split('|').collect::<Vec<&str>>())
        .collect();

    let mut counts = vec![1; numbers.len()];

    // loop through numbers and get score for card
    for n in 0..numbers.len() {
        let winning_numbers: HashSet<u32> = numbers[n]
            .first()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let our_numbers: HashSet<u32> = numbers[n]
            .last()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let score: usize = winning_numbers.intersection(&our_numbers).count();

        // update count of card copies
        let mut max = numbers.len();
        if n + score < numbers.len() {
            max = n + score;
        }
        for i in n + 1..=max {
            counts[i] += counts[n];
        }
    }

    let sum: usize = counts.iter().sum();

    println!("{}", sum);
}

// ========================================================================
