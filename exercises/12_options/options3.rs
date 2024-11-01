#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        // using ref here is a reference to the value inside the option
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
