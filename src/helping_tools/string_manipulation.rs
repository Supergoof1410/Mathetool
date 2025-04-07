// Um überflüssige Leerzeichen zu entfernen
pub fn remove_whitespaces(with_whitespaces: String) -> String {
    with_whitespaces.chars().filter(|c| !c.is_whitespace()).collect()
}

// Mit dieser Funktion werden nur Leerzeichen vor dem Operator gesetzt, um
// sie in einer anderen Funktion besser zu teilen.
pub fn terms_replace_operators(splitted_equation: String) -> String {
    let mut terms_replaced: String = String::new();

    for terms in splitted_equation.chars() {
            match terms {
                '+' => terms_replaced.push_str( " +"),
                '-' => terms_replaced.push_str(" -"),
                _ => terms_replaced.push(terms),
        }
    }

    terms_replaced
}

// Hier werden die einzelnen Terme nochmals gesplitted, damit man besser
// mit Ihnen rechnen kann.
pub fn split_terms(splitting_terms: String) -> Vec<String> {
    let splitted_terms: Vec<String> = splitting_terms.split(' ').map(str::to_string).collect();
    splitted_terms
}