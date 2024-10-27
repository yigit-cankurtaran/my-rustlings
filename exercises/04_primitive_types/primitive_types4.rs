fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???

        let nice_slice = &a[1..4];
        // the value of `nice_slice` is of type `&[i32]`
        // we're using the value to create a slice of the same type
        // we can't use "a" directly because the type of `a` is `[i32; 5]`
        // we need a &[i32] instead for it to be a valid slice

        assert_eq!([2, 3, 4], nice_slice);
    }
}
