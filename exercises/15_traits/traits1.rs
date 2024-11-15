// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
    // this is a trait
    // we're showing this to say "if you can call this method"
    // "you're implementing this trait"
    fn append_bar(self) -> Self;
    // AppendBar implements append_bar, append_bar is a method of AppendBar
    // append_bar takes self and returns Self
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
    // this is an implementation of the trait
    // we'll fill out the method specified in the trait
    // String now implements AppendBar
    fn append_bar(self) -> Self {
        self + "Bar"
        // this is the body of the method
        // take self and return Self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
