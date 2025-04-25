pub fn display_terminals(output_string: String, formula: &String) {
    let width: usize = 30;

    println!("{:<width$} : {}", output_string, formula, width = width);
}