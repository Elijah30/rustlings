// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

enum Wrapper {
    Integer(u32),
    Str(String),
}

impl Wrapper {
    pub fn new_integer(value: u32) -> Self {
        Wrapper::Integer(value)
    }

    pub fn new_str(value: &str) -> Self {
        Wrapper::Str(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new_integer(42), Wrapper::Integer(42));
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new_str("Foo"), Wrapper::Str("Foo".to_string()));
    }
}
