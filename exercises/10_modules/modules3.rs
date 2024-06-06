// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

// On utilise l'instruction 'use' pour importer SystemTime et UNIX_EPOCH du module std::time
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // On obtient le temps actuel et on calcule la durée depuis l'époque UNIX
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        // Si l'opération réussit, on affiche le nombre de secondes depuis l'époque UNIX
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        // Si l'opération échoue (ce qui signifie que le temps actuel est avant l'époque UNIX), on génère une panique
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
