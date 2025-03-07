mod math_formulas;
fn main() {
    // Zum testen ist das Array statisch
    let variables: [f64; 3] = [2.0,-3.0,-2.0];
     
    println!("\nWir übergeben die quadratische Formel an die Mitternachtsformel\n");
  
    // Das Array wird mit call-by-reference übergeben.
    math_formulas::midnight_formula::midnight_formula(&variables);
}