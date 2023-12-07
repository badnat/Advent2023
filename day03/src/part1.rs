#![allow(clippy::needless_return)]

// Make a 2d Vector out of lines
// map the x, y positions of the all the symbols to a Set<Tuple<u32,u32>>
//
// Iterate through the Set of symbol positions
//
// Check all adjacent psoitions, if one is a character then search left and right to find digits
//
// Store these digits and loop around those to see if any other symbols touch and remove those from
// the set of symbol positions
use itertools::iproduct;
use std::collections::HashSet;

fn main() {
    let schematic = make_schematic(include_str!("../input/part1"));
    println!("{:?}", schematic.symbol_positions);
    schematic.adjacent_search((1, 1));
}

struct Part {
    number: u32,
    row: usize,
    span: (usize, usize),
}
struct Schematic {
    char_matrix: Vec<Vec<char>>,
    symbol_positions: HashSet<(usize, usize)>,
}
impl Schematic {
    fn adjacent_search(&self, pos: (usize, usize)) {

        for (i, j) in iproduct!(-1..=1, -1..=1) {
            if (i == 0 && j == 0) || pos.0+i < 0 || pos.0+i > self.char_matrix.len() || pos.1+j < 0 || pos.1+j > self.char_matrix[0]. {
                continue;
            }
            println!("{:?}", &self.char_matrix.get(pos.0 + i));
        }
    }
}

fn make_schematic(input: &str) -> Schematic {
    let char_matrix: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect(); // that was so crab

    let mut symbol_positions: HashSet<(usize, usize)> = HashSet::new();

    for (x, y) in iproduct!((0..char_matrix.len()), (0..char_matrix[0].len())) {
        if is_symbol(char_matrix[x][y]) {
            symbol_positions.insert((x, y));
        }
    }

    let schematic = Schematic {
        char_matrix,
        symbol_positions,
    };
    return schematic;
}

fn is_symbol(c: char) -> bool {
    return !(c.is_ascii_digit() || (c == '.'));
}
