// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// On définit une structure Book qui prend une référence avec une durée de vie 'a pour l'auteur et le titre.
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // On crée une instance de Book avec des références aux chaînes de caractères.
    let book = Book { author: &name, title: &title };

    // On affiche le titre et l'auteur du livre.
    println!("{} by {}", book.title, book.author);
}
