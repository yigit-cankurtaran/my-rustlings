fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        // after y is used it's dropped, so x can be used again
        let z = &mut x;
        // using x again
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
