use std::io;

fn main() {
    // Lecture du nombre d'entrées N
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Lecture des valeurs des entrées
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<i32> = input
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    // Calcul de la somme des dépenses (valeurs négatives)
    let total_expenses: i32 = values
        .iter()
        .filter(|&&x| x < 0)  // Filtrer les dépenses (valeurs négatives)
        .sum::<i32>().abs();   // Prendre la somme absolue des dépenses

    // Affichage du résultat
    println!("{}", total_expenses);
}
