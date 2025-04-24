use crate::paths;

pub fn one_unknown(equation: String) {
    
    println!("\nOriginal Gleichung: {}", &equation);

    println!("\n1. Leerzeichen entfernen: \n");

    // Alle Leerzeichen entfernen
    // let without_whitespaces: String = paths::str_manipulation::remove_whitespaces(equation);

    // println!("Ergebnis: {}", without_whitespaces);


    // Der Split von der Gleichung zu einer rechten und linken Seite.
    //let equation_split_equal: Vec<String> = without_whitespaces.split('=').map(str::to_string).collect(); 

    println!("\n2. Aufteilung der Gleichung: \n");

    // for left_and_right in equation_split_equal.iter() {
    //     println!("{}", left_and_right);
    // }

    // Hier ersetze ich die Operatoren mit einem zusätzlichen Leerzeichen
    // um es später weiter splitten zu können. Mit der ausgelagerten Funktion
    // terms_replace_operators, damit spare ich mir redudanten Code.
    //let terms_replaced_operators_both: String = paths::str_manipulation::terms_replace_operators(equation_split_equal);

    println!("\n3. Leerzeichen vor den Operatoren setzten: \n");

    // for terms_with_whitespace_ops in terms_replaced_operators_both.iter() {
    //     println!("{:+}", terms_with_whitespace_ops);
    // }
}



fn split_x_and_number_terms(split_terms: Vec<&str>) -> (Vec<&str>, Vec<i32>) {
    let mut tupel_terms: (Vec<&str>, Vec<i32>) = (Vec::new(), Vec::new());

    for term in split_terms {
        if term.contains("x") {
            tupel_terms.0.push(term);
        }
        else {
            tupel_terms.1.push(term.parse::<i32>().unwrap());
        }
    }

    tupel_terms
}