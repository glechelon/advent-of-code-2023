use std::{collections::HashMap, fs};

use lazy_static::lazy_static;

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Vec<(String, i32)>>,
}

lazy_static! {
    static ref QUANTITY_OF_CUBES_BY_COLOR: HashMap<String, i32> = {
        let mut quantity_of_cubes_by_color = HashMap::new();
        //12 red cubes, 13 green cubes, and 14 blue cubes
      quantity_of_cubes_by_color.insert("red".to_string(), 12);
      quantity_of_cubes_by_color.insert("green".to_string(), 13);
      quantity_of_cubes_by_color.insert("blue".to_string(), 14);
      quantity_of_cubes_by_color
    };
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Impossible de lire le fichier");
    let games = extract_games(input);
    println!("{:#?}", &games);
    let result: i32 = games
        .iter()
        .filter(|game| !is_game_impossible(game))
        .map(|game| game.id)
        .sum();
    println!("Somme des ids des parties possibles : {:?}", &result);
}

fn is_game_impossible(game: &&Game) -> bool {
    game.sets.iter().any(|set| {
        set.iter().any(|colors| {
            colors.1.gt(QUANTITY_OF_CUBES_BY_COLOR
                .get(&colors.0)
                .expect("Impossible de trouver la couleur correspondante."))
        })
    })
}

fn extract_games(input: String) -> Vec<Game> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let game = line.split(":").collect::<Vec<&str>>();
            let game_header = extract_game_header(&game);
            let game_id = extract_game_id(game_header);
            let game_body = extract_game_body(game);
            let sets = extract_sets(game_body);
            Game {
                id: game_id,
                sets: sets,
            }
        })
        .collect()
}

fn extract_sets(game_body: String) -> Vec<Vec<(String, i32)>> {
    let sets: Vec<Vec<(String, i32)>> = game_body
        .split(";")
        .into_iter()
        .map(|raw_set| extract_set(raw_set))
        .collect();
    sets
}

fn extract_set(raw_set: &str) -> Vec<(String, i32)> {
    raw_set
        .split(",")
        .into_iter()
        .map(|raw_color| extract_color(raw_color))
        .collect()
}

fn extract_color(raw_color: &str) -> (String, i32) {
    let color: Vec<&str> = raw_color.split(" ").collect();
    (
        color.get(2).unwrap().to_string(),
        color.get(1).unwrap().parse::<i32>().unwrap(),
    )
}

fn extract_game_body(game: Vec<&str>) -> String {
    let game_body: &&str = game
        .get(1)
        .expect("impossible de récupérer le corps de la partie");
    game_body.to_string()
}

fn extract_game_id(game_header: String) -> i32 {
    let game_id: i32 = game_header
        .split(" ")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse()
        .unwrap();
    game_id
}

fn extract_game_header(game: &Vec<&str>) -> String {
    let game_header = game
        .get(0)
        .expect("impossible de récupérer l'entête de la partie");
    game_header.to_string()
}
