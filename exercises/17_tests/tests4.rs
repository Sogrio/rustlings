// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

// On définit une structure Rectangle avec des champs width et height.
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // On change uniquement les fonctions de test elles-mêmes.
    pub fn new(width: i32, height: i32) -> Self {
        // On vérifie si la largeur et la hauteur sont positives avant de créer le rectangle.
        if width <= 0 || height <= 0 {
            panic!("La largeur et la hauteur du rectangle ne peuvent pas être négatives !")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // Ce test vérifie si le rectangle a la taille que nous passons dans son constructeur.
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // vérifier la largeur
        assert_eq!(rect.height, 20); // vérifier la hauteur
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // Ce test vérifie si le programme panique lorsque nous essayons de créer un rectangle avec une largeur négative.
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // Ce test vérifie si le programme panique lorsque nous essayons de créer un rectangle avec une hauteur négative.
        let _rect = Rectangle::new(10, -10);
    }
}
