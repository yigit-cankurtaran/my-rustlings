fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            // if the word is target
            // we're using optionals because the value might be None
            assert_eq!(word, target);
            // extracts the value from the option
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];
        // mutable vector with a single None

        for i in 1..=range {
            optional_integers.push(Some(i));
            // pushes Some(i) to the vector until the range is reached
            // last element would be Some(10)
        }

        let mut cursor = range;
        // creating a mutable variable to iterate over the vector

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        while let Some(Some(integer)) = optional_integers.pop() {
            // needs 2 layers of pattern matching (nested)
            // first Some matches the pop and that returns an Option
            // then the Option is matched and the value is extracted

            // if it encounters a None it will stop

            // checks if the value popped is equal to the cursor
            assert_eq!(integer, cursor);
            cursor -= 1;
            // decrements the cursor
            // the loop ends when the cursor is 0
            // because the vector only has a None
        }

        assert_eq!(cursor, 0);
        // asserts that the cursor is 0
        // meaning all values were popped

        // this example is about handling values that might be absent
    }
}
