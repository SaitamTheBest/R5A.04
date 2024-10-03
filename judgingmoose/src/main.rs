use std::io::{self, BufRead};

fn make_status(n: i32, p: i32) -> String {
    if n == 0 && p == 0 {
        format!("Not a moose")
    } else if n == p {
        format!("Even {}", 2 * n)
    } else {
        format!("Odd {}", 2 * std::cmp::max(n, p))
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Lire la première ligne avec N et P
    if let Some(Ok(first_line)) = lines.next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        let n: i32 = parts[0].parse().expect("N doit être un entier");
        let p: i32 = parts[1].parse().expect("P doit être un entier");

        println!("{}", make_status(n, p));
    }
}
