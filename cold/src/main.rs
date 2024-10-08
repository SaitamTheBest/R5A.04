use std::io;

fn main() {
    // Lecture du nombre de températures
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Lecture des températures
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let temperatures: Vec<i32> = input
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();

    // Comptage des températures strictement inférieures à 0
    let count = temperatures.iter().filter(|&&t| t < 0).count();

    // Affichage du résultat
    println!("{}", count);
}
