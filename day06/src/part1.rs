#![allow(clippy::needless_return)]

// ======================================================

fn main() {
    let input = include_str!("../inputs/input")
        .split('\n')
        .map(|s| {
            s.split_whitespace()
                .skip(1)
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut answer = 1;
    for i in 0..input[0].len() {
        answer *= get_num_winning_times(input[1][i] as f32, input[0][i] as f32);
    }
    println!("{:?}", answer);
}

// ======================================================

fn get_num_winning_times(d: f32, t: f32) -> u32 {
    let roots = roots(d, t);
    return (roots.1.ceil() as u32 - roots.0.floor() as u32 - 1).max(0);
}

fn roots(d: f32, t: f32) -> (f32, f32) {
    let low = (t - ((t * t - 4. * d).sqrt())) / 2.;
    let high = (t + ((t * t - 4. * d).sqrt())) / 2.;
    return (low, high);
}

// ======================================================
