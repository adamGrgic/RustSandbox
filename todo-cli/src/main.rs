use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::fmt;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}
};
//use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: String,
    title: String,
    complete: bool
}

use uuid::Uuid;

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} (Completed: {})",
            self.id,
            self.title,
            if self.complete { "Yes" } else { "No" }
        )
    }
}


fn rename_across_filesystems(old_file: &str, new_file: &str) -> io::Result<()> {
    fs::copy(old_file, new_file)?; // Copy the file
    fs::remove_file(old_file)?;   // Remove the old file
    Ok(())
}


#[tokio::main]
async fn main()-> crossterm::Result<()> {
    println!("Welcome to your todo application.");


    println!("What would you like to do?");

    let menu_options = vec!["Create Todo", "Finish Todo", "List Todos", "Exit"];
    let mut selected_index = 0;
    let mut output_message = String::new();

//    let path = "./test.txt";
//    let content = std::fs::read_to_string(path)
//    .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let todos = match fetch_todos().await {
        Ok(todos) => todos,
        Err(err) => {
            eprintln!("Failed to fetch todos: {}", err);
            disable_raw_mode()?;
            return Ok(());
        }
    };


    loop {

        execute!(stdout,cursor::MoveTo(0,0), Clear(ClearType::All))?;

        println!("=== MENU === \r\n");

        for (index, option) in menu_options.iter().enumerate() {
            if index == selected_index {
                write!(stdout,"> \x1b[32m{}\x1b[0m \r\n", option.trim())?;
            } else {
                write!(stdout,"  {} \r\n", option.trim())?;
            }
        }

        execute!(stdout, cursor::MoveTo(0, menu_options.len() as u16+2))?;
        println!("=== DEBUG OUTPUT === \r\n");
        println!("debug: {} \r\n", output_message);

        for (_index, todo) in todos.iter().enumerate() {
            println!("[] {} \r", todo.title);
        }

        stdout.flush()?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < menu_options.len() - 1 {
                        selected_index += 1
                    }
                }
                KeyCode::Enter => {
                    // write!(stdout, "{} index number was selected",selected_index)?;
                    output_message = match selected_index {
                        0 => {
                            let mut todos = match fetch_todos().await {
                            Ok(todos) => todos,
                            Err(err) => {
                                    eprintln!("Failed to fetch todos: {}", err);
                                    disable_raw_mode()?;
                                    return Ok(());
                                }
                            };

                            disable_raw_mode()?;

                            println!("Please enter your task name:");

                            let mut name_input = String::new();
                            io::stdin().read_line(&mut name_input)?;

                            println!("You entered: {}", name_input.trim());
                            println!("Please enter your description:");

                            let mut description_input = String::new();
                            io::stdin().read_line(&mut description_input)?;

                            println!("You entered: {}", description_input.trim());

                            // create guid
                            let id = Uuid::new_v4().to_string();

                            todos.push(Todo {id,title:name_input.trim().to_string(), complete: false});
                            enable_raw_mode()?;

                            // Serialize results to JSON
                            let json = serde_json::to_string_pretty(&todos).unwrap();

                            let todo_results_file = format!(".todos.json");
                            let todo_results_file_swp = format!(".todos.swp.json");

                            match rename_across_filesystems(&todo_results_file, &todo_results_file_swp) {
                                Ok(_) => println!("File moved successfully!"),
                                Err(e) => eprintln!("Error moving file: {}", e),
                            }

                            match File::create(todo_results_file) {
                                Ok(mut file) => {
                                    // File creation succeeded, now write to it
                                    match file.write_all(json.as_bytes()) {
                                        Ok(_) => {
                                            println!("File written successfully!");
                                            fs::remove_file(todo_results_file_swp)?;   // Remove the old file
                                        },
                                        Err(e) => eprintln!("Failed to write to file: {}", e),
                                    }
                                }
                                Err(e) => eprintln!("Failed to create file: {}", e),
                            }


                            //let mut file = File::create(todo_results_file)?;
                            //file.write_all(json.as_bytes())?;


                            "You selected: Create Todo".to_string()
                        },
                        1 => "You selected: Finish Todo".to_string(),
                        2 => {
                            let mut todos = match fetch_todos().await {
                            Ok(todos) => todos,
                            Err(err) => {
                                    eprintln!("Failed to fetch todos: {}", err);
                                    disable_raw_mode()?;
                                    return Ok(());
                                }
                            };



                            "You selected: List Todos".to_string()
                        },
                        3 => {
                             println!("Exiting...\r\n");
                             break;
                        }
                        _ => unreachable!(),

                    }
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    //    let mut guess = String::new();

    //    io::stdin()
    //        .read_line(&mut guess)
    //        .expect("Failed to read line");

    disable_raw_mode()?;
    Ok(())

}

async fn fetch_todos() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let file_name = "./.todos.json";

    // Open the file
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let todos: Vec<Todo> = serde_json::from_reader(reader)?;
    Ok(todos)
}

