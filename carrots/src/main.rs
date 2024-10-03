use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Lire la première ligne avec N et P
    if let Some(Ok(first_line)) = lines.next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        // N est le nombre de participants (que l'on ignore)
        let _n: usize = parts[0].parse().expect("N doit être un entier");
        // P est le nombre de carottes à distribuer (ce que l'on veut)
        let p: usize = parts[1].parse().expect("P doit être un entier");
        
        // Afficher le nombre de carottes
        println!("{}", p);
    }
}