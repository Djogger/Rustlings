// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.



struct Wrapper<T, U> {
    value: T,
    value2: U,
}

impl<T, U> Wrapper<T, U> {
    pub fn new(value: T, value2: U) -> Self {
        Wrapper { value: value,  value2: value2}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42, "Foo").value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new(42, "Foo").value2, "Foo");
    }
}
