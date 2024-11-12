// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
        // this calls the constructor of the Wrapper struct
        // the wrapper wraps whatever value is passed to it
        // and stores it in the value field
    }
}

fn main() {
    // You can optionally experiment here.
    println!(
        "when we wrap a number the value is: {}",
        &Wrapper::new(42).value // using & because we want to borrow the value
    );
    println!(
        "when we wrap a string the value is: {}",
        &Wrapper::new("42").value
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
