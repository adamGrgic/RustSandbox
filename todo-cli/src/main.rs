use std::io::{self, Write};
use std::fmt;
use tokio::fs;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}
};
//use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    complete: bool
}

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
        println!("=== OUTPUT === \r\n");
        println!("debug: {} \r\n", output_message);

        for (_index, todo) in todos.iter().enumerate() {
            println!("{} \r\n", todo);
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
                        0 => "You selected: Create Todo".to_string(),
                        1 => "You selected: Finish Todo".to_string(),
                        2 => "You selected: List Todos".to_string(),
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
    let file_name = "./todos.json";
    let file_content = fs::read_to_string(&file_name);
    let todos: Vec<Todo> = serde_json::from_str(file_content)?;
    Ok(todos)
}

