// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// On définit une structure générique Wrapper qui peut envelopper n'importe quel type T.
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    // On implémente une méthode new générique qui crée une instance de Wrapper pour n'importe quel type T.
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        // On vérifie si Wrapper peut contenir un u32.
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // On vérifie si Wrapper peut contenir une chaîne de caractères (&str).
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
