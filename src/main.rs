#![allow(dead_code)]

mod paths;

mod helping_tools;
mod algebra;
mod arithmetic;

fn main() {
    let equation_string: String = "-3 + 6 * 2 - 4 / 2".to_string();

    let splitted_terms: Vec<String> = paths::str_manipulation::strings_refactor(equation_string);

    let result_mult_div: Vec<String> = paths::calc_nums::calculate_numbers_mult_diff(splitted_terms);

    println!("Ergebnis: {:?}", result_mult_div);

    let result_mult_div_all: String = paths::calc_nums::calculate_numbers_addition(result_mult_div);

    let end_result: f64 = match result_mult_div_all.parse::<f64>() {
        Ok(result) => result,
        Err(_) => 0.0
    };

    println!("Endergebnis: {:.2}", end_result);
}