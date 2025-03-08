use super::display_results;

/// Berechnet die Lösungen einer quadratischen Gleichung mit der Mitternachtsformel.
///
/// # Parameter
/// - `variables_array`: Ein Array mit genau 3 Werten (f64), die die Koeffizienten
///   der quadratischen Gleichung darstellen: [a, b, c].
///   Diese Werte werden per Referenz übergeben, da sie unverändert bleiben sollen.
///
/// # Ablauf
/// - Die Diskriminante wird berechnet, um zu prüfen, ob es 0, 1 oder 2 Lösungen gibt.
/// - Je nach Fall wird eine oder beide Lösungen berechnet und ausgegeben.
/// - Komplexe Zahlen (imaginäre Lösungen) werden aktuell unterstützt.
///
/// # Hinweis
/// Die Funktion nutzt eine mutable lokale Variable `result`, um die Berechnung
/// nacheinander durchzuführen, ohne für jede Lösung eine eigene Variable zu erzeugen.
/// Dies hält den Speicherbedarf minimal und den Code klar.

pub fn midnight_formula(variables_array: &[f64; 3]) {

    println!("\nQuadratische Gleichung mittels Mitternachtsformel lösen!");

    // Die Result Variable wird noch an anderer Stelle gebraucht, deswegen wird sie nur für die
    // Funktion mutable gemacht.
    let mut result: f64;

    // Berechnen der Diskriminante
    println!("\nJetzt wird die Diskriminante als erstes berechnet\n");
    
    let radikand: f64 = (variables_array[1] * variables_array[1]) - 4.0 * variables_array[0]*variables_array[2];

    println!("\nRadikand = {}\n", radikand);
    
    // Berechnung mit komplexen Zahlen "i"
    if radikand < 0.0 {
        let radikand_negotation: f64 = radikand * (-1.0);

        result = (-(variables_array[1]))/(2.0 * variables_array[0]);
        let result_imag: f64 = radikand_negotation.sqrt() / (2.0 * variables_array[0]);
        display_results::display_result(&result, &radikand, None, Some(result_imag));
    }

    // Berechnung nur wenn die Diskriminante = 0 ist
    else if radikand == 0.0 {
        println!("Es gibt genau eine Lösung!");

        result = (-(variables_array[1])-radikand.sqrt())/(2.0 * variables_array[0]);
        display_results::display_result(&result, &radikand, None, None);
    }

    // Berechnung mit zwei Lösungen, wenn die Diskriminante > 0 ist
    else {
        println!("Es gibt 2 Lösungen!");

        result = (-(variables_array[1])+radikand.sqrt())/(2.0 * variables_array[0]);
        display_results::display_result(&result, &radikand, Some(1), None);
        
        result = (-(variables_array[1])-radikand.sqrt())/(2.0 * variables_array[0]);
        display_results::display_result(&result, &radikand, Some(0), None);
    }
}
