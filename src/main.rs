#![allow(dead_code)]

mod math_formulas;
fn main() {
    // Zum testen ist das Array statisch
    let variables: [f64; 3] = [2.0,3.0,5.0];
     
    println!("\nWerte: A = {}, B = {}, C = {}", variables[0], variables[1], variables[2]);

    // Test potenzieren

    //println!("Potenz aus: {} = {}", variables[1], variables[1].powf(3.0));

    // Das Array wird mit call-by-reference übergeben.
    math_formulas::equations::quadratic_equations::midnight_formula::midnight_formula(&variables);
    println!("--------------------------------------------");
    math_formulas::equations::quadratic_equations::p_and_q_formula::p_and_q_formula(&variables);

}