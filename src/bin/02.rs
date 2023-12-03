use ac::read_lines;
use regex::Regex;

struct Game {
    id: u32,
    r: u32,
    g: u32,
    b: u32
}

impl Game {
    pub fn is_possible(&self) -> bool {
        let max_r = 12;
        let max_g = 13;
        let max_b = 14;
        if self.r > max_r || self.g > max_g || self.b > max_b {
            false
        }else{
            true
        }
    }
}


fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("inputs/2.in") {
        for line in lines {
            let current_line = &line.unwrap();
            let (id, game_vec) = cleanup(&current_line);
            let game:Game =  max_num(id, game_vec);
            if game.is_possible(){
                sum += game.id;
            }
        }
    }
    println!("{sum}");
    // let word = "Game 1: 7 blue, 5 red; 10 red, 7 blue; 5 blue, 4 green, 15 red; 4 green, 6 red, 7 blue; 5 green, 8 blue, 4 red; 5 red, 4 blue, 3 green".to_string();
    // let (id, game_vec) = cleanup(&word);
    // let game:Game =  max_num(id, game_vec);
    // let r = game.is_possible();
    // println!("{}",r);
}

fn cleanup(game:&str) -> (u32, Vec<&str>){
    let re = Regex::new(r": ").unwrap();
    let re_comma = Regex::new(r"[;,] ").unwrap();

    let game_split:Vec<&str> = re.split(game).collect();
    let id:u32 = game_split[0].split_whitespace().nth(1).unwrap().parse().unwrap();
    let game_content = game_split[1];
    let game_vec:Vec<&str> = re_comma.split(game_content).collect();
    (id, game_vec)
}

fn max_num(id:u32, game_vec: Vec<&str>) -> Game {
    let mut r:u32 = 0;
    let mut g:u32 = 0;
    let mut b:u32 = 0;
    for current_stat in game_vec {
        let ball_stat:Vec<_> = current_stat.split_whitespace().collect();
        let number:u32 = ball_stat[0].parse().unwrap();
        match ball_stat[1] {
            "red" => r = if number > r {number} else {r},
            "green" => g = if number > g {number} else {g},
            "blue" => b = if number > b {number} else {b},
             _ => todo!()
        }
    }
    Game{id,r,g,b}
}
