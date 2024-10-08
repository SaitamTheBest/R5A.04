use std::io;

fn main() {
    let mut blimps = Vec::new();

    // Lecture des 5 lignes d'entrées représentant les codes des dirigeables
    for _ in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        blimps.push(input.trim().to_string());
    }

    // Trouver les indices des dirigeables contenant "FBI"
    let mut indices = Vec::new();
    for (i, blimp) in blimps.iter().enumerate() {
        if blimp.contains("FBI") {
            indices.push(i + 1);  // On ajoute 1 car l'énumération commence à 0
        }
    }

    // Affichage des résultats
    if indices.is_empty() {
        println!("HE GOT AWAY!");
    } else {
        let result = indices.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", result);
    }
}
