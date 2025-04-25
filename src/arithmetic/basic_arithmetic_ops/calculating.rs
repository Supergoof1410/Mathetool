use core::f64;

use crate::helping_tools::display_terminal::display_terminals;

// Hier wird die vorletzte Rechenregel angewendet die Punkt vor Strich
// Berechnung, zur besseren Wartbarkeit werden die Multiplikation und Division
// in zwei extra Funktionen ausgelagert.
pub fn calculate_numbers_mult_diff(numbers: Vec<String>) -> Vec<String> {
    
    let mut result_mul_div_vector: Vec<String> = numbers;
    let mut index: usize = 0;
    let mut counter_mult: usize = 0; 
    let mut counter_div: usize = 0;

    println!("\nBerechnung (links nach rechts)\n");
    
    while index < result_mul_div_vector.len(){

        // Multiplikation
        if result_mul_div_vector[index].contains("*") {
            
            counter_mult += 1;

            let result_mult: String = calculate_mult_diff(&result_mul_div_vector, counter_mult, '*');
            
            removing_from_vector(&mut result_mul_div_vector, index, result_mult);
            
            println!("Vektor davor: {:?}", result_mul_div_vector);
            println!("Vektor danach: {:?}\n", result_mul_div_vector);
            
            index = 0;
        }

        // Division
        else if result_mul_div_vector[index].contains("/") {

            counter_div += 1;

            let result_mult: String = calculate_mult_diff(&result_mul_div_vector, counter_div, '/');

            removing_from_vector(&mut result_mul_div_vector, index, result_mult);
            
            println!("Vektor davor: {:?}", result_mul_div_vector);
            println!("Vektor danach: {:?}\n", result_mul_div_vector);

            index = 0;
        }
        else {
            index += 1;
        }
    }

    return result_mul_div_vector
}


// Mit dieser Funktion werden die Zahlen sowohl positiv
// und negativ miteinander addiert. Das ist der letzte Schritt
// bei den Rechenregeln. Da hier das Vorzeichen mitgenommen wird
// muss man hier nicht auf die Vertauschungsregel achten.
pub fn calculate_numbers_addition(numbers: Vec<String>) -> String {
    
    let mut result: f64 = 0.0;
    
    for num in numbers.iter() {
        match num.parse::<f64>() {
            Ok(number) => result += number,
            Err(_) => return "Error: invalid number".to_string(),
        }
    }
    return result.to_string()
}

// Funktion für die Multiplikation und die Division
fn calculate_mult_diff(numbers: &Vec<String>, counter: usize, which_operator: char) -> String {
    let mut result_mult: String = String::new();
    let mut result: f64;
    let mut operation: String = String::new();

    for (i, num) in numbers.iter().enumerate() {
        if num.contains(which_operator) {
            let left = match numbers[i - 1].parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    return "Invalid Number".to_string()
                }     
            };  
            let right = match num.replace(which_operator, "").parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    return "Invalid Number".to_string()
                }
            }; 

            match which_operator {
                '*' => {
                    result = left * right;
                    operation = "Multiplikation".to_string();
                }
                '/' => {
                    result = left / right;
                    operation = "Division".to_string();
                }
                _ => return "Invalid Number".to_string()
            }

            result_mult = result.to_string();  
            break;
        }
    }

    print!("{}. ", counter);
    display_terminals(operation, &result_mult);

    return result_mult 
}

// Kleines Refactoring weil das entfernen und hinzufügen mehrmals
// vorkommt, habe ich sie ausgelagert.
fn removing_from_vector(numbers_vector: &mut Vec<String>, index: usize, result_mult: String) {
    
    // Das aktuelle Element wird entfernt
    numbers_vector.remove(index);
    // Das vorherige Element wird entfernt
    numbers_vector.remove(index - 1);
    // Und durch ein neues ersetzt
    numbers_vector.insert(index - 1, result_mult);
}