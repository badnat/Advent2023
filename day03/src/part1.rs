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

// ========================================================================================

fn main() {
    let mut schematic = make_schematic(include_str!("../input/part1"));

    schematic.search_for_parts();

    println!("{:?}", schematic.sum_parts());
}

// ========================================================================================
#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    number: u32,
    row: usize,
    span: (usize, usize),
}

// ========================================================================================

struct Schematic {
    char_matrix: Vec<Vec<char>>,
    symbol_positions: HashSet<(usize, usize)>,
    parts: HashSet<Part>,
}
impl Schematic {
    fn search_for_parts(&mut self) {
        for pos in self.symbol_positions.clone() {
            self.adjacent_search(pos);
        }
    }

    fn adjacent_search(&mut self, pos: (usize, usize)) {
        let mut lower_x = 0;
        let mut lower_y = 0;
        if pos.0 != lower_x {
            lower_x = pos.0 - 1;
        }
        if pos.1 != lower_y {
            lower_y = pos.1 - 1;
        }

        let mut upper_x = self.char_matrix.len() - 1;
        let mut upper_y = self.char_matrix[0].len() - 1;
        if pos.0 != upper_x {
            upper_x = pos.0 + 1;
        }
        if pos.1 != upper_y {
            upper_y = pos.1 + 1;
        }

        for (i, j) in iproduct!(lower_x..=upper_x, lower_y..=upper_y) {
            if i == pos.0 && j == pos.1 {
                continue;
            }
            if self.char_matrix[i][j].is_ascii_digit() {
                self.parts.insert(self.find_part((i, j)));
            }
        }
    }

    fn find_part(&self, pos: (usize, usize)) -> Part {
        let mut digit_vec = vec![];
        let mut span = (pos.1, pos.1);
        for y_back in (0..pos.1).rev() {
            if self.char_matrix[pos.0][y_back].is_ascii_digit() {
                digit_vec.insert(0, self.char_matrix[pos.0][y_back]);
                span.0 = y_back;
            } else {
                break;
            }
        }
        for y_forward in pos.1..self.char_matrix[0].len() {
            if self.char_matrix[pos.0][y_forward].is_ascii_digit() {
                digit_vec.push(self.char_matrix[pos.0][y_forward]);
                span.1 = y_forward;
            } else {
                break;
            }
        }
        return make_part(digit_vec, pos.0, span);
    }

    fn sum_parts(&self) -> u32 {
        let sum = self.parts.iter().map(|p| p.number).sum();
        return sum;
    }
}

// ========================================================================================

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
        parts: HashSet::new(),
    };
    return schematic;
}

fn is_symbol(c: char) -> bool {
    return !(c.is_ascii_digit() || (c == '.'));
}

fn make_part(digit_array: Vec<char>, row: usize, span: (usize, usize)) -> Part {
    let part = Part {
        number: digit_array.into_iter().collect::<String>().parse().unwrap(),
        row,
        span,
    };
    return part;
}
