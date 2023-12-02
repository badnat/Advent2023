use either::Either;

fn main() {
    let lines = include_str!("../input/part2").split('\n');
    let mut sum = 0;
    for line in lines {
        sum += 10*find_number(line, true);
        sum += find_number(line, false);
    }
    println!("{sum}");    
}

fn find_number(line:&str, forward:bool) -> u32 {
    let alpha_numerals:Vec<char> = line.chars().collect();

    let span = if forward {
        Either::Left(0..alpha_numerals.len())
    } else {
        Either::Right((0..alpha_numerals.len()).rev())
    };

    for i in span{
        if alpha_numerals[i].is_digit(10) { return alpha_numerals[i].to_digit(10).unwrap();}
        for j in i..alpha_numerals.len() {
            let opt_num: Option<u32> = word_to_num(alpha_numerals[i..=j].to_vec());
            match opt_num {
                Some(opt_num) => return opt_num,
                None => (),
            };
        }
    }
    return 0;
}

fn word_to_num(char_vec:Vec<char>) -> Option<u32> {
    let s = char_vec.iter().cloned().collect::<String>();
    match s.as_str() {
        "zero" => return Some(0),
        "one" => return Some(1),
        "two" => return Some(2),
        "three" => return Some(3),
        "four" => return Some(4),
        "five" => return Some(5),
        "six" => return Some(6),
        "seven" => return Some(7),
        "eight" => return Some(8),
        "nine" => return Some(9),
        _ => return None,
    };
}
