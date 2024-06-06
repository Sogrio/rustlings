// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip] // Indique à Rustfmt de ne pas formater ce bloc de code

// Définition d'une macro `my_macro` avec deux patterns de correspondance
macro_rules! my_macro {
    // Aucun argument passé à la macro
    () => {
        println!("Check out my macro!"); // Affiche un message
    };
    // Correspondance pour un argument passé à la macro
    ($val:expr) => {
        println!("Look at this other macro: {}", $val); // Affiche un message avec la valeur de l'argument
    }
}

fn main() {
    my_macro!(); // Appel de la macro sans argument
    my_macro!(7777); // Appel de la macro avec un argument
}
