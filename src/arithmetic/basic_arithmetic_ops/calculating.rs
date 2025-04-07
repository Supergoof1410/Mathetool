pub fn calculate_numbers(numbers: Vec<String>) -> String {
    
    let mut result: i32 = 0;
    
    for num in numbers.iter() {
        match num.parse::<i32>() {
            Ok(number) => result += number,
            Err(_) => return "Error: invalid number".to_string(),
        }
    }
    return result.to_string()
}