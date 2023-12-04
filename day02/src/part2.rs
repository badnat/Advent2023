static RED:u32 = 12;
static GREEN:u32 = 13;
static BLUE:u32 = 14;

use std::cmp::max;

fn main() {
    let lines = include_str!("../input/part2").split('\n');
    let mut sum = 0;
    for line in lines {
        if let Some(g) = line_to_game(line) {
            sum += g.min_power();
        }
    }
    println!("{sum}");
}
#[derive(Debug)]
struct Draw(u32, u32, u32);

#[derive(Debug)]
struct Game {
    draws:Vec<Draw>,
    id:u32,
}
impl Game {
    fn min_power(&self) -> u32{
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for draw in &self.draws {
           max_red = max(max_red, draw.0);
           max_green= max(max_green, draw.1);
           max_blue = max(max_blue, draw.2);
        }
        return max_red * max_green * max_blue;
    }
}

fn line_to_game(line:&str) -> Option<Game> {
    let g_d_split:Vec<&str> = line.split(':').collect();
    let game_id:Option<String> = g_d_split.get(0).map(|s| s.chars().filter(|n| n.is_digit(10)).collect());
    let game_id: Option<u32> = game_id.and_then(|s| s.parse().ok());

    if let None = game_id { return None; }

    let mut game = Game {
        draws: vec![],
        id: game_id.unwrap(),
    };

    let draw_strs = g_d_split.get(1).unwrap().split(';');
    for draw_str in draw_strs {
        if let Some(d) = str_to_draws(draw_str) { game.draws.push(d); } else { return None; }
    }
    return Some(game)
}

fn str_to_draws(draw_str:&str) -> Option<Draw>{
    let mut draw = Draw(0,0,0);
    for cube_str in draw_str.split(',') {
        let cube_info:Vec<&str> = cube_str.trim().split_whitespace().collect();
        let cube_num:u32 = cube_info.get(0).unwrap().parse().ok().unwrap();
        let cube_color:&str = cube_info.get(1).unwrap();

        match cube_color {
            "red" => draw.0 = cube_num,
            "green" => draw.1 = cube_num,
            "blue" => draw.2 = cube_num,
            _ => return None,
        };
    }
    return Some(draw);
}
