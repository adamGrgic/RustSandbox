// use this module to understand basic list concepts in Rust

// Defining and instantiating structs

// instead of classes like in dotnet, we use Structs
// my current understanding of this here is that these are more property based rather than
// in dotnet where you have functions and other behavior to describe the class

// we use derive to inject certain dependencies we want into the struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, lists-0!");

    let mut first_names: Vec<&str> = Vec::new();

    // first names
    first_names.push("Sally");
    first_names.push("Luke");
    first_names.push("Alex");
    first_names.push("Samantha");
    first_names.push("Thomas");
    first_names.push("Eric");
    first_names.push("Tyler");
    first_names.push("Kelsey");
    first_names.push("Kate");
    first_names.push("Kyle");
    first_names.push("Sean");

    let mut last_names: Vec<&str> = Vec::new();

    last_names.push("Smith");
    last_names.push("O'Grady");
    last_names.push("Kent");
    last_names.push("Piker");
    last_names.push("Williams");
    last_names.push("Jackson");
    last_names.push("Garcia");
    last_names.push("Gomez");

    let names_count = 50;
    let mut names_list: Vec<&str, &str> = Vec::new();

    loop {
        names_list.push(("firstname","lastname"));


    }




    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };


    // println!("{:?}", user1);

    // user1.username = String::from("foo dog");

    // print json
    // println!("{:#?}", user1);
}
