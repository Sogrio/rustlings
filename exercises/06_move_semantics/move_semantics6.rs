// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string(); // Crée une String à partir d'un littéral

    get_char(&data); // Passe une référence à `get_char`, donc `data` n'est pas déplacé

    string_uppercase(data); // Passe la propriété à `string_uppercase`, donc `data` est déplacé
}

// Ne doit pas prendre la propriété
fn get_char(data: &String) -> char {
    data.chars().last().unwrap() // Retourne le dernier caractère de la chaîne
}

// Doit prendre la propriété
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // Convertit la chaîne en majuscules

    println!("{}", data); // Affiche la chaîne en majuscules
}
