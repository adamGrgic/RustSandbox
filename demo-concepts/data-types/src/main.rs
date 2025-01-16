// link: https://doc.rust-lang.org/book/ch03-02-data-types.html

fn main() {
    println!("Data types sample");

    // rust is a statically typed language
    // => we must know the types at compile time

    // this will throw an error because the type of "42 is not known at compile time
    // let guess = "42".parse().expect("Not a number");

    // by adding :u32 we define our type
    // let guess: u32 = "42".parse().expect("Not a number!");


}
