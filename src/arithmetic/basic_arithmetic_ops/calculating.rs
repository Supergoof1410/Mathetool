use core::f64;

// Hier wird die vorletzte Rechenregel angewendet die Punkt vor Strich
// Berechnung, zur besseren Wartbarkeit werden die Multiplikation und Division
// in zwei extra Funktionen ausgelagert.
pub fn calculate_numbers_mult_diff(numbers: Vec<String>) -> Vec<String> {
    
    let mut result_mul_div_vector: Vec<String> = numbers.clone();
    let mut index: usize = 0;
    
    while index < result_mul_div_vector.len(){

        // Multiplikation
        if result_mul_div_vector[index].contains("*") {

            let result_mult = calculate_numbers_multiplikation(&result_mul_div_vector);
            
            // Das aktuelle Element wird entfernt
            result_mul_div_vector.remove(index);

            // Das vorherige Element wird entfernt
            result_mul_div_vector.remove(index - 1);

            // Und durch ein neues ersetzt
            result_mul_div_vector.insert(index - 1, result_mult);
            index = 0;
        }

        // Division
        else if result_mul_div_vector[index].contains("/") {
            let result_mult = calculate_numbers_division(&result_mul_div_vector);
            // Das aktuelle Element wird entfernt
            result_mul_div_vector.remove(index);
            
            // Das vorherige Element wird entfernt
            result_mul_div_vector.remove(index - 1);

            // Und durch ein neues ersetzt
            result_mul_div_vector.insert(index - 1, result_mult);
            index = 0;
        }
        else {
            //result_mul_div_vector.push(result_mul_div_vector[index].to_string());
            index += 1;
        }
    }

    return result_mul_div_vector
}

// Funktion zum berechnen der einzelnen Multiplikationen in einem
// String "3+6+8-3*5+7*9"
pub fn calculate_numbers_multiplikation(numbers: &Vec<String>) -> String {
    let mut result_mult: String = String::new();
    println!("Vor Multiplikation: {:?}", numbers);

    for (i, num) in numbers.iter().enumerate() {
        if num.contains("*") {
            let left = match numbers[i - 1].parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    return "Invalid Number".to_string()
                }     
            };  
            let right = match num.replace("*", "").parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    return "Invalid Number".to_string()
                }
            }; 

            let result: f64 = left * right;
            result_mult = result.to_string();  
            break;
        }
        
    }
    return result_mult 
    
}

// Funktion zum berechnen der einzelnen Multiplikationen in einem
// String "3+6+8-3*5+7/9"
pub fn calculate_numbers_division(numbers: &Vec<String>) -> String {
    let mut result_mult: String = String::new();
    println!("Vor Division: {:?}", numbers);

    for (i,num) in numbers.iter().enumerate() {
        if num.contains("/") {
            let left = match numbers[i - 1].parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    return "Invalid Number".to_string()
                }     
            };  
            let right = match num.replace("/", "").parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    return "Invalid Number".to_string()
                }
            }; 

            let result: f64 = left / right;
            result_mult = result.to_string(); 
            break;   
        }
        
    }
    return result_mult
}

// Mit dieser Funktion werden die Zahlen sowohl positiv
// und negativ miteinander addiert. Das ist der letzte Schritt
// bei den Rechenregeln.
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

fn removing_from_vector() {

}