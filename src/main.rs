#![allow(dead_code)]

use helping_tools::display_terminal::display_terminals;

mod paths;

mod helping_tools;
mod algebra;
mod arithmetic;

fn main() {
    let equation_string: String = "3 +2+1*60-5/658".to_string();
    println!();
    display_terminals("Original Formel".to_string(), &equation_string);

    let splitted_terms: Vec<String> = paths::str_manipulation::strings_refactor(equation_string);

    let result_mult_div: Vec<String> = paths::calc_nums::calculate_numbers_mult_diff(splitted_terms);

    //println!("Ergebnis: {:?}", result_mult_div);
    println!();
    display_terminals("Addition mit Subtraktion".to_string(), &result_mult_div.join(" "));

    let result_addition_all: String = paths::calc_nums::calculate_numbers_addition(result_mult_div);

    let end_result: f64 = match result_addition_all.parse::<f64>() {
        Ok(result) => result,
        Err(_) => 0.0
    };

    //println!("Endergebnis: {:.2}", end_result);
    display_terminals("Endergebnis".to_string(), &end_result.to_string());
    println!();
}