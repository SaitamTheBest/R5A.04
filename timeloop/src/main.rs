use std::io;

fn main() {
    // Lire l'entrée utilisateur
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");
    
    // Convertir l'entrée en un entier
    let n: usize = input.trim().parse().expect("Veuillez entrer un nombre valide");

    // Générer la sortie de 1 à N
    for i in 1..=n {
        println!("{} Abracadabra", i);
    }
}