use std::io::{self, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

fn main()-> crossterm::Result<()> {
    println!("Welcome to your todo application.");


    println!("What would you like to do?");

    let menu_options = vec!["Create Todo", "Finish Todo", "List Todos", "Exit"];
    let mut selected_index = 0;

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    loop {

        execute!(stdout, cursor::MoveTo(0,0), Clear(ClearType::FromCursorDown))?;

        for (index, option) in menu_options.iter().enumerate() {
            if index == selected_index {
                write!(stdout,"> \x1b[32m{}\x1b[0m \r\n", option.trim())?;
            } else {
                write!(stdout,"  {} \r\n", option.trim())?;
            }
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
                match selected_index {
                    0 => println!("You selected: Create Todo"),
                    1 => println!("You selected: Finish Todo"),
                    2 => println!("You selected: List Todos"),
                    3 => {
                         println!("Exiting...");
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
