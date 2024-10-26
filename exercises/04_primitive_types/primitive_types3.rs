fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a: [i32; 100] = [0; 100];
    // the semicolon is a way to create an array
    // the number inside the brackets is the length of the array
    // the number inside the brackets is the value of each element
    // this is a hundred zeroes

    println!("{:?}", a);
    // the question mark is a way to print the value of a variable
    // implements the Debug trait
    // might learn more about that later

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
