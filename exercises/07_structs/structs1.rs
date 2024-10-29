struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u8,
    green: u8,
    blue: u8,
    // using u8 because it's small enough to fit in a single byte
    // max value is 255
    // to use this with println we need to add a # in front of it
}
// more complicated, named fields

// struct ColorTupleStruct(/* TODO: Add the fields that the test `tuple_structs` expects */);
struct ColorTupleStruct(u8, u8, u8);
// called a tuple struct, because it's like a tuple_structs
// used for simple types because no named fields

#[derive(Debug)]
struct UnitStruct; // unit structs are like unit types
                   // used as markers
                   // they don't have any fields
                   // they are useful for generics and enums

fn main() {
    // You can optionally experiment here.
    let unit_struct = UnitStruct;
    println!("{unit_struct:?}s are fun!");

    let color_regular = ColorRegularStruct {
        red: 0,
        green: 255,
        blue: 0,
    };

    println!("green in regular is {}", color_regular.green);
    // field access is not allowed in strings

    let color_tuple = ColorTupleStruct(0, 255, 0);
    println!("green in tuple is {}", color_tuple.1);
    // in tuple you access values with indexes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
