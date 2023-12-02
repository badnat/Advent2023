fn main() {
    let lines = include_str!("../input/part1").split('\n');
    let mut sum = 0;
    for line in lines {
        sum += get_calibration_value(line);
    }
    println!("{sum}");

}
fn get_calibration_value(mumbo:&str) -> u32 {
    let digits: Vec<u32> = mumbo.chars().filter_map(|b| b.to_digit(10)).collect();

    let first = digits.first();
    let last = digits.last();

    let first = match first {
        Some(first) => first,
        None => &0,
    };

    let last = match last {
        Some(last) => last,
        None => &0,
    };

    return (first * 10) + (last)
}
