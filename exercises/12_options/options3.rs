#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    // point struct with x and y coordinates
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });
    // creating an option with a value of a point

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        // match checking optional_point
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        // if optional_point is Some(ref p) it binds p as a reference to the
        // value inside the option
        // ownership stays in optional_point so we can use it later
        // then it prints the co-ordinates by matching p using p.x and p.y
        _ => panic!("No match!"),
        // if optional_point is None it will panic and print No match!
    }

    println!("{optional_point:?}"); // Don't change this line.
                                    // then we print the optional_point itself
                                    // prints Some(Point { x: 100, y: 200 })
}
