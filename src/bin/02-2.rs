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

    pub fn power_set(&self) -> u32 {
        self.r * self.g * self.b
    }
}


fn main() {
    let input = include_str!("../../inputs/2.in");
    let output = part2(input);
    dbg!(output);
}

fn part2(input:&str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let (id, game_vec) = cleanup(line);
        let game:Game =  max_num(id, game_vec);
        sum += game.power_set();
    };
    sum
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        assert_eq!(result,2286);
    }

    #[test]
    fn game1() {
        let (id, game_vec) = cleanup(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        );
        let game1 =  max_num(id, game_vec);
        assert_eq!(game1.power_set(),48);
    }
}
