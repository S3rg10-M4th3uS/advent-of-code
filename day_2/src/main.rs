use std::{collections::HashMap, num::ParseIntError, result, cmp::Ordering};

#[derive(Debug)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: HashMap<u16, Bag>,
}
impl Game {
    fn new(bags: Bag, id: u16) -> Game {
        let mut hasher = HashMap::new();
        hasher.insert(id, bags);
        Game { id: hasher }
    }
    fn bag_value(&self, color: &str, key: u16) -> usize {
        match self.id.get(&key) {
            Some(bag) => match color {
                "red" => bag.red,
                "green" => bag.green,
                "blue" => bag.blue,
                _ => 0, // Handle other colors as needed
            },
            None => 0, // Handle the case when the key is not present
        }
    }
}
fn main() {
    let line= include_str!("sample.txt");
    let original_game = Game::new(Bag { red: 12, green: 13, blue: 14 }, 0);
    let mut all_games = Vec::new();
    for lines in line.lines() {
           let bag = find_counts_by_color::<ParseIntError>(lines).unwrap();
           all_games.push(bag);
    }
    let mut key = 0;
    let result: Vec<_> = all_games
    
    .iter()
    .filter(|game| {
        ["red", "green", "blue"]
            .iter()
            .any(|&color| {
                let original_value = original_game.bag_value(color, 0); 
                let game_value = game.bag_value(color, key); 
                key += 1;
                match game_value.cmp(&original_value) {
                    Ordering::Greater => false,
                    Ordering::Less | Ordering::Equal => true,
                }
            })
    })
    .collect();

println!("{:?}", result);
}

fn find_counts_by_color<E>(input: &str) -> Result<Game, E>
where
    E: std::convert::From<std::num::ParseIntError>,
{
    let colors = vec!["red", "green", "blue"];
    let mut counts_by_color: Vec<(String, Vec<usize>)> = colors
        .iter()
        .map(|color| (color.to_string(), Vec::new()))
        .collect();

        for game in input.split(';') {
            for (color, values) in counts_by_color.iter_mut() {
                values.extend(game.split(',').filter_map(|item| {
                    let parts: Vec<&str> = item.trim().split_whitespace().collect();
                    if let Some(index) = parts.iter().position(|&x| x == *color) {
                        parts
                            .get(index.checked_sub(1)?)
                            .and_then(|s| s.parse::<usize>().ok())
                    } else {
                        None
                    }
                }));
            }
        }
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };
    for (color, values) in counts_by_color {
        if let Some(max_value) = values.iter().max() {
            match color.as_str() {
                "red" => bag.red = *max_value,
                "green" => bag.green = *max_value,
                "blue" => bag.blue = *max_value,
                _ => {}
            }
        }
    }
    let id = input
        .chars()
        .skip_while(|x| !x.is_ascii_digit())
        .take_while(|&x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<u16>()?;
    Ok(Game::new(bag, id))
}
