use std::error::Error;

#[derive(Debug)]
pub struct Draw {
    r: u64,
    g: u64,
    b: u64,
}

#[derive(Debug)]
pub struct Game {
    id: u64,
    draws: Vec<Draw>,
}

impl Game {
    fn is_possible(&self, r: u64, g: u64, b: u64) -> bool {
        for draw in &self.draws {
            if draw.r > r || draw.g > g || draw.b > b {
                return false;
            }
        }
        return true;
    }
}

pub fn line_to_games(line: String) -> Game {
    let split: Vec<&str> = line.split(":").collect();
    let game_id = split[0].trim()[5..].to_owned().parse::<u64>().unwrap();
    let games: Vec<Draw> = split[1]
        .trim()
        .split(";")
        .map(|draw| {
            let mut parsed_draw = Draw { r: 0, g: 0, b: 0 };
            for cube in draw.trim().split(",") {
                let parts: Vec<&str> = cube.trim().split(" ").collect();
                let number = parts[0].parse::<u64>().unwrap();
                match parts[1] {
                    "red" => parsed_draw.r += number,
                    "green" => parsed_draw.g += number,
                    "blue" => parsed_draw.b += number,
                    _ => panic!("Should not happen"),
                }
            }
            parsed_draw
        })
        .collect();

    Game {
        id: game_id,
        draws: games,
    }
}

pub fn task1(games: &Vec<Game>) -> u64 {
    games
        .into_iter()
        .filter(|g| g.is_possible(12, 13, 14))
        .map(|x| x.id)
        .sum()
}

pub fn task2(games: &Vec<Game>) -> u64 {
    games
        .into_iter()
        .map(|game| {
            let mut min_r = 0;
            let mut min_g = 0;
            let mut min_b = 0;
            for draw in &game.draws {
                min_r = std::cmp::max(min_r, draw.r);
                min_g = std::cmp::max(min_g, draw.g);
                min_b = std::cmp::max(min_b, draw.b);
            }
            min_r * min_g * min_b
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::common::fold_on_each_line1;

    #[test]
    fn test_day2() {
        let example_input: Vec<Game> =
            fold_on_each_line1("input/day2_example.txt", line_to_games).unwrap();
        let task1_example = task1(&example_input);
        println!("Day2 task1 example = {}", task1_example);
        assert_eq!(8, task1_example);

        let input = fold_on_each_line1("input/day2_input.txt", line_to_games).unwrap();
        let task1 = task1(&input);
        println!("Day2 task1 input = {}", task1);
        assert_eq!(2541, task1);

        let task2_exmple = task2(&example_input);
        println!("Day2 task2 example = {}", task2_exmple);

        let task2 = task2(&input);
        println!("Day2 task2 input = {}", task2);
    }
}
