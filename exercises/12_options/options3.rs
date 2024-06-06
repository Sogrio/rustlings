// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // On utilise une référence (&) dans le match pour éviter la propriété de déplacement (move)
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y), // Affiche les coordonnées si Some
        _ => panic!("no match!"), // Panique si None
    }
    y; // Corrige sans enlever la ligne
}
