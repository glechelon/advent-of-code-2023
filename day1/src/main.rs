use std::{collections::HashMap, fs};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DIGITS_STRING: HashMap<&'static str, i32> = {
        let mut digits_string = HashMap::new();
        digits_string.insert("one", 1);
        digits_string.insert("two", 2);
        digits_string.insert("three", 3);
        digits_string.insert("four", 4);
        digits_string.insert("five", 5);
        digits_string.insert("six", 6);
        digits_string.insert("seven", 7);
        digits_string.insert("eight", 8);
        digits_string.insert("nine", 9);
        digits_string
    };
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Impossible de lire le fichier");
    let calibration_values: Vec<String> = input
        .lines()
        .map(|ligne| extract_digits(ligne))
        .map(|caputered_elements| get_calibration_value(caputered_elements))
        .collect();

    let resultat: i32 = calibration_values
        .iter()
        .map(|cv| convert_to_integer(cv))
        .sum();

    println!("Résultat : {:?}", &resultat);
}

fn extract_digits(ligne: &str) -> Vec<String> {
    let mut extracted_digits_by_index: Vec<(String, usize)> = Vec::new();

    let digits_numeric_format = DIGITS_STRING
        .clone()
        .values()
        .map(|digit| digit.to_string())
        .collect::<Vec<String>>();

    let digits_alpha_format: Vec<String> = DIGITS_STRING
        .clone()
        .keys()
        .map(ToString::to_string)
        .collect();
    digits_numeric_format.iter().for_each(|digit| {
        if ligne.contains(digit) {
            ligne.match_indices(digit).into_iter().for_each(|index| {
                extracted_digits_by_index.push((digit.clone(), index.0));
            })
        }
    });

    digits_alpha_format.iter().for_each(|digit| {
        if ligne.contains(digit) {
            ligne.match_indices(digit).into_iter().for_each(|index| {
                extracted_digits_by_index.push((digit.clone(), index.0));
            })
        }
    });

    extracted_digits_by_index.sort_by(|a, b| a.1.cmp(&b.1));
    extracted_digits_by_index
        .iter()
        .map(|d| d.0.clone())
        .collect::<Vec<String>>()
}

fn convert_to_integer(cv: &String) -> i32 {
    dbg!(cv);
    cv.parse::<i32>()
        .expect("Impossible de convertir en entier")
}

fn is_digit_string(cv: &String) -> bool {
    DIGITS_STRING
        .clone()
        .into_keys()
        .any(|ds| ds.eq(cv.as_str()))
}

fn get_calibration_value(caputered_elements: Vec<String>) -> String {
    convert_to_number_format_if_needed(caputered_elements.get(0).unwrap().to_string())
        + &convert_to_number_format_if_needed(caputered_elements.last().unwrap().to_string())
}

fn convert_to_number_format_if_needed(input: String) -> String {
    if is_digit_string(&input) {
        DIGITS_STRING
            .get(&input.as_str())
            .expect("Impossible de convertir en entier")
            .to_string()
    } else {
        input
    }
}
