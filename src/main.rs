mod math_formulas;
fn main() {
    let variables: [f64; 3] = [2.0,-3.0,-2.0];

    println!("\nQuadratische Gleichung mittels Mitternachtsformel lösen!");

    println!("\nWerte: A = {}, B = {}, C = {}", variables[0], variables[1], variables[2]);

    // Das Array wird mit call-by-reference übergeben sie sollen statisch bleiben
    println!("\nWir übergeben die quadratische Formel an die Mitternachtsformel\nmit den oben angegebenen Werten.");
    math_formulas::midnight_formula::midnight_formula(&variables);
}