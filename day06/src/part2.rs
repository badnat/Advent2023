#![allow(clippy::needless_return)]

// ======================================================

fn main() {
    let input = include_str!("../inputs/input")
        .lines()
        .map(|s| {
            s.split(':')
                .nth(1)
                .unwrap()
                .replace(' ', "")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();
    println!(
        "{}",
        get_num_winning_times(input[1] as f64, input[0] as f64)
    );
}

// ======================================================

fn get_num_winning_times(d: f64, t: f64) -> u64 {
    let roots = roots(d, t);
    return (roots.1.ceil() as u64 - roots.0.floor() as u64 - 1).max(0);
}

fn roots(d: f64, t: f64) -> (f64, f64) {
    let low = (t - ((t * t - 4. * d).sqrt())) / 2.;
    let high = (t + ((t * t - 4. * d).sqrt())) / 2.;
    return (low, high);
}

// ======================================================
