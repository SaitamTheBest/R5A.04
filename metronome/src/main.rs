use std::io;

fn main() {
    // Lire l'entrée depuis stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée");
    
    // Convertir l'entrée en entier
    let ticks: f64 = input.trim().parse().expect("Veuillez entrer un entier valide");
    
    // Calculer le nombre de révolutions
    let revolutions = ticks / 4.0;
    
    // Afficher le résultat avec deux décimales
    println!("{:.2}", revolutions);
}