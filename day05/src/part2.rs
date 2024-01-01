#![allow(clippy::needless_return)]

use rayon::prelude::*;
use std::time::Instant;

// ========================================================================

fn main() {
    let now = Instant::now();

    let text: Vec<&str> = include_str!("../inputs/input").split("\n\n").collect();

    let mut seeds_info: Vec<u64> = text[0].split("seeds:").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut seeds = Vec::new();

    while seeds_info.len() > 0 {
        let range = seeds_info.pop().unwrap();
        let start = seeds_info.pop().unwrap();

        for i in start..start + range {
            seeds.push(i);
        }
    }

    println!("{} Seeds Created!", &seeds.len());

    let blocks: Vec<Block> = text.into_iter().skip(1).map(make_block).collect();
    println!("Blocks Created!");

    let m = seeds
        .par_iter()
        .map(|&s| chain_translate(s, &blocks))
        .min()
        .unwrap();

    println!("{:?}", m);
    println!("Time: {:?}", now.elapsed());
}

// ========================================================================

#[derive(Debug)]
struct Block {
    sources: Vec<u64>,
    destinations: Vec<u64>,
    spans: Vec<u64>,
}
impl Block {
    fn translate(&self, input: u64) -> u64 {
        for i in 0..self.sources.len() {
            if (input >= self.sources[i]) && (input < self.sources[i] + self.spans[i]) {
                return input - self.sources[i] + self.destinations[i];
            }
        }
        return input;
    }
}

// ========================================================================

fn chain_translate(input: u64, blocks: &Vec<Block>) -> u64 {
    let mut output = input;
    for block in blocks {
        output = block.translate(output);
    }
    return output;
}

fn make_block(input_str: &str) -> Block {
    let mut sources = Vec::new();
    let mut destinations = Vec::new();
    let mut spans = Vec::new();

    let lines: Vec<&str> = input_str.split('\n').skip(1).collect();

    for line in lines {
        let l: Vec<&str> = line.split_whitespace().collect();
        if l.len() <= 2 {
            continue;
        }
        destinations.push(l[0].parse::<u64>().unwrap());
        sources.push(l[1].parse::<u64>().unwrap());
        spans.push(l[2].parse::<u64>().unwrap());
    }

    return Block {
        sources,
        destinations,
        spans,
    };
}
