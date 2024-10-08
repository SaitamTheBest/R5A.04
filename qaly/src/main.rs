use std::io;

fn main() {
    let mut input = String::new();

    // Lecture du nombre de périodes
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut total_qaly = 0.0;

    // Lecture de chaque période et calcul du QALY
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let quality_of_life: f64 = parts[0].parse().unwrap();
        let years: f64 = parts[1].parse().unwrap();
        total_qaly += quality_of_life * years;
    }

    // Affichage du résultat avec une précision de 3 décimales
    println!("{:.3}", total_qaly);
}
