use std::fs;

use regex::Regex;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Impossible de lire le fichier");
    let number_regex = Regex::new("[0-9]").expect("Erreur lors de la création de la regex");
    let calibration_values: Vec<String> = input
        .lines()
        .map(|ligne| {
            number_regex
                .find_iter(ligne)
                .map(|f| f.as_str())
                .collect::<Vec<&str>>()
        })
        .map(|caputered_elements| get_calibration_value(caputered_elements))
        .collect();

    let resultat: i32 = calibration_values
        .iter()
        .map(|cv| {
            cv.parse::<i32>()
                .expect("Impossible de convertir en entier")
        })
        .sum();

    println!("Résultat : {:?}", &resultat);
}

fn get_calibration_value(caputered_elements: Vec<&str>) -> String {
    caputered_elements.get(0).unwrap().to_string()
        + caputered_elements
            .get(caputered_elements.len() - 1)
            .unwrap()
}
