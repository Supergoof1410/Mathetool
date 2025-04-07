#![allow(dead_code)]

mod paths;

mod helping_tools;
mod algebra;
mod arithmetic;

fn main() {
    let calculate_string: String = "3+6  + 8 - 3+5+7-9".to_string();

    let removed_spaces: String = paths::str_manipulation::remove_whitespaces(calculate_string);

    println!("Leerzeichen entfernt: {}\n", removed_spaces);

    let replaced_operators: String = paths::str_manipulation::terms_replace_operators(removed_spaces);

    println!("Leerzeichen vor den Operatoren: {}\n", replaced_operators);

    let splitted_terms: Vec<String> = paths::str_manipulation::split_terms(replaced_operators);

    println!("Terme einzeln aufgeteilt {:?}", splitted_terms);

    println!("Ergebnis: {:?}", paths::calc_nums::calculate_numbers(splitted_terms));
}