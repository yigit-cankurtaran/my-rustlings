// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());
    // to_owned() is used to make an owned copy of a borrowed value

    string_slice("nice weather".into());
    // into() is used to convert to an inferred type

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);
    // we're getting a string slice of the first two characters
    // this is a string slice of the first two characters of "abc"

    string_slice("  hello there ".trim());
    // trim returns a slice that doesn't include the whitespace at the beginning or end

    string("Happy Monday!".replace("Mon", "Tues"));
    // replace returns a new string with all occurrences of a string replaced with another

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
    // to_lowercase returns a new string with all characters in lowercase
}
