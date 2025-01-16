fn main() {
    // in rust we must signal when a variable is subject to change by using
    // the mut keyword. Without mut, this code won't compile.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // const is a keyword we can use to declare that something is immutable (and wont work with
    // mut

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // use a variable that has already been declared
    let x = 5;

    let x = x + 1;

    {
        let x = x*2;
        println!("The value of x ini the inner scope is: {x}");
        // expected output : 12
    }

    println!("The value of x is: {x}");
    // expected output : 6



}
