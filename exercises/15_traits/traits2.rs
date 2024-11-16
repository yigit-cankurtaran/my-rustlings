trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        // using mut here is important
        // we're changing the value of self
        self.push("Bar".to_string());
        self
    }
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        self + "Bar"
        // we don't need to use mut here because we're not changing the value of self
        // we're just returning a new value as a result of the method
    }
}

fn main() {
    // You can optionally experiment here.
    let lol = vec![String::from("Foo")].append_bar();
    println!("lol: {lol:?}");
    // prints ["Foo", "Bar"]
    let lmao = String::from("Bar").append_bar();
    println!("lmao: {lmao:?}");
    // prints "BarBar"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
