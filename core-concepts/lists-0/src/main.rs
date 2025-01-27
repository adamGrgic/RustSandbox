// Notes on lists
// I think I'm more concerned with being able to iterate through lists of objects and do things
// with their properties, like modify them or make new lists . How is that handled in rust?
//
//
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

fn get_oldest_person(people: &[Person]) -> Option<&Person> {
    people.iter().max_by_key(|x| x.age) // Propagates None if the list is empty
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./src/data.json";

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

     // Parse the JSON data into the `Person` struct
    let people: Vec<Person> = serde_json::from_reader(reader)?;

    if let Some(person) = get_oldest_person(&people) {
        println!("Oldest person is: {:?}", person);
    } else {
        println!("No people in the list.");
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)?;

    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    // Deserialize the JSON data
    let mut people: Vec<Person> = serde_json::from_str(&json_content)?;
    println!("Current data: {:?}", people);

    // Modify the data
    people.push(Person {
        name: "Viola".to_string(),
        age: 28,
        is_student: false,
    });

    // Serialize the updated data
    let updated_json = serde_json::to_string_pretty(&people)?;

    // Write the updated JSON back to the file
    file.set_len(0)?; // Clear the file before writing
    file.write_all(updated_json.as_bytes())?;

    println!("File updated successfully!");
    Ok(())
}

