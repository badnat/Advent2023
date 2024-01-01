#![allow(clippy::needless_return)]

// ========================================================================

fn main() {
    let text: Vec<&str> = include_str!("../inputs/input").split("\n\n").collect();

    let seeds: Vec<u64> = text[0].split("seeds:").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let blocks: Vec<Block> = text.into_iter().skip(1).map(make_block).collect();

    let m = seeds
        .iter()
        .map(|s| chain_translate(*s, &blocks))
        .min()
        .unwrap();

    println!("{:?}", m);
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
