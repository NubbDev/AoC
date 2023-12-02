use std::collections::HashMap;

fn main() {
    let valid_games: HashMap<&str, i32> = HashMap::from({
        let mut map = HashMap::new();
        map.insert("red", 12);
        map.insert("green", 13);
        map.insert("blue", 14);
        map
    });

    // let _test = include_str!("../test.txt");
    let _input = include_str!("../input.txt");

    let part1 = part1(_input, valid_games);
    let part2 = part2(_input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &str, valid: HashMap<&str, i32>) -> i32 {
    let games = input.lines();

    let mut possible_games: Vec<i32> = vec![];

    'game: for game in games {
        let info_split: Vec<&str> = game.split(":").collect();
        let game_info = info_split[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let game_value: String = info_split[1].to_string();

        let game_value_split: Vec<&str> = game_value.split(";").collect();

        for values in game_value_split {
            let values_split: Vec<&str> = values.split(",").collect();

            for value in values_split {
                let value_split: Vec<&str> = value.split(" ").collect();

                let value_name = value_split[2];
                let value_amount  = value_split[1].parse::<i32>().unwrap();

                let compare_value = valid.get(value_name).unwrap().clone();

                if possible_games.contains(&game_info) {
                    if value_amount <= compare_value {
                        continue
                    } else {
                        let index = possible_games.iter().position(|x| *x == game_info).unwrap();
                        possible_games.remove(index);
                        continue 'game;
                    }
                } else {
                    if compare_value > value_amount {
                        possible_games.push(game_info);
                    }
                }
            }
        }
    }

    possible_games.iter().sum()
}

fn part2(input: &str) -> i32 {
    let games = input.lines();
    let mut total = 0;
    for game in games {
        let info_split: Vec<&str> = game.split(":").collect();
        // let game_info = info_split[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let game_value: String = info_split[1].to_string();

        let game_value_split: Vec<&str> = game_value.split(";").collect();

        let mut smallest_values: HashMap<String, i32> = HashMap::new();
        
        for values in game_value_split {
            let values_split = values.split(",").collect::<Vec<&str>>();
            for value in values_split {
                let value_split: Vec<&str> = value.split(" ").collect();
                let value_name = value_split[2];
                let value_amount  = value_split[1].parse::<i32>().unwrap();

                let value = smallest_values.get(value_name);

                if value == None {
                    smallest_values.insert(value_name.to_string(), value_amount);
                    continue
                }

                if &value_amount > value.unwrap() {
                    smallest_values.insert(value_name.to_string(), value_amount);
                }
            }
        }

        let mut product_sum = 1;
        for (_, value) in smallest_values {
            product_sum *= value;
        }

        total += product_sum;
        
    }
    total
}