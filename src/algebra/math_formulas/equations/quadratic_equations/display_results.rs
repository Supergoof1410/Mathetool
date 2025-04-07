pub fn display_result(result: &f64, radikand: &f64, solution: Option<i8>, result_imag: Option<f64>) {
    if *radikand < 0.0 {
        let radikand_negotation: f64 = *radikand * (-1.0);

        println!("Es gibt keine Lösung! Für reelle Zahlen\n");
        println!("Für komplexe Zahlen\n");

        if solution == Some(1) {
            println!("Radikand umkehren: {}\n", radikand_negotation * 4.0);
        }
        else {
            println!("Radikand umkehren: {}\n", radikand_negotation);
        }
        
        if let Some(value) = result_imag {
            println!("Lösungen: {} (+/-): {:.5?}i", result, value);
        }
    }
    else if *radikand == 0.0 {
        println!("Lösung: {}", result);
    }
    else {
        if solution == Some(1) {
            println!("\nDie erste lösung mit (+):\n");
            println!("Lösung mit (+): {}", result);
        }
        else {
            println!("\nDie zweite lösung mit (-):\n");
            println!("Lösung mit (-): {}", result);
        }
    }
}