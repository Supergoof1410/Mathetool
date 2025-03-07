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
/// - Komplexe Zahlen (imaginäre Lösungen) werden aktuell nicht unterstützt.
///
/// # Hinweis
/// Die Funktion nutzt eine mutable lokale Variable `result`, um die Berechnung
/// nacheinander durchzuführen, ohne für jede Lösung eine eigene Variable zu erzeugen.
/// Dies hält den Speicherbedarf minimal und den Code klar.

pub fn midnight_formula(variables_array: &[f64; 3]) {

    println!("\nQuadratische Gleichung mittels Mitternachtsformel lösen!");

    println!("\nWerte: A = {}, B = {}, C = {}", variables_array[0], variables_array[1], variables_array[2]);

    // Die Result Variable wird noch an anderer Stelle gebraucht, deswegen wird sie nur für die
    // Funktion mutable gemacht.
    let mut result: f64;

    // Berechnen der Diskriminante
    println!("\nJetzt wird die Diskriminante als erstes berechnet\n");
    let discriminante: f64 = (variables_array[1] * variables_array[1]) - 4.0 * variables_array[0]*variables_array[2];
    println!("Diskriminante: {}\n", discriminante);

    // Berechnung mit komplexen Zahlen "i"
    if discriminante < 0.0 {
        println!("Es gibt keine Lösung!");
    }

    // Berechnung nur wenn die Diskriminante = 0 ist
    else if discriminante == 0.0 {
        println!("Es gibt genau eine Lösung!");
        result = (-(variables_array[1])-discriminante.sqrt())/(2.0 * variables_array[0]);
        println!("Lösung: {}", result); 
    }

    // Berechnung mit zwei Lösungen, wenn die Diskriminante > 0 ist
    else {
        println!("Es gibt 2 Lösungen!");
        println!("\nDie erste lösung mit (+):\n");

        result = (-(variables_array[1])+discriminante.sqrt())/(2.0 * variables_array[0]);
        println!("Lösung mit (+): {}", result);

        println!("\nDie erste lösung mit (-):\n");

        result = (-(variables_array[1])-discriminante.sqrt())/(2.0 * variables_array[0]);
        println!("Lösung mit (+): {}", result);
    }
}