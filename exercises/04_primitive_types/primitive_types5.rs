fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    let (name, age) = cat;
    // the value of `name` is of type `&str`
    // the value of `age` is of type `f64`
    // this is called a "destructuring" pattern
    // helps us get multiple values out of a single value

    println!("{name} is {age} years old");
}
