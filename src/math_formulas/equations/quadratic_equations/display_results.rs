pub fn display_result(result: &f64, diskriminante: i8, solution: i8) {
    if diskriminante == 1 {
        if solution == 1 {
            println!("\nDie erste lösung mit (+):\n");
            println!("Lösung mit (+): {}", result);
        }
        else {
            println!("\nDie zweite lösung mit (-):\n");
            println!("Lösung mit (-): {}", result);
        }
    }
}