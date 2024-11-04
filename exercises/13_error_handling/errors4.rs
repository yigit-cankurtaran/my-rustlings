#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        match value {
            v if v < 0 => Err(CreationError::Negative),
            // v is a match binding expression
            // referring to the value in each match arm
            // if the value is less than 0 it will return an error
            0 => Err(CreationError::Zero),
            // v if v == 0 => Err(CreationError::Zero),
            // redundant guard warning, we can just simply use the 0 match
            // if the value is 0 it will return an error
            _ => Ok(PositiveNonzeroInteger(value as u64)),
            // _ is the wild card, any value that doesn't match the previous
            // here is how we handle the self returning result
            // we call the impl block and return the result with the value inside
        }
    }
}

fn main() {
    // You can optionally experiment here.
    println!("{:?}", PositiveNonzeroInteger::new(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
