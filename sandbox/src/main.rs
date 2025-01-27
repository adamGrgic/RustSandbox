use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor,
};
use std::io::{stdout, Write};

fn main() -> crossterm::Result<()> {
    let options = vec!["Option 1", "Option 2", "Option 3", "Option 4"];
    let mut selected = vec![false; options.len()];
    let mut current = 0;

    // Enable raw mode to capture key events
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::Hide)?;

    loop {
        // Display the options
        for (i, option) in options.iter().enumerate() {
            if i == current {
                write!(stdout, "> ")?; // Highlight the current selection
            } else {
                write!(stdout, "  ")?;
            }

            if selected[i] {
                writeln!(stdout, "[X] {}", option)?;
            } else {
                writeln!(stdout, "[ ] {}", option)?;
            }
        }

        stdout.flush()?;

        // Wait for a key event
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if current > 0 {
                        current -= 1;
                    }
                }
                KeyCode::Down => {
                    if current < options.len() - 1 {
                        current += 1;
                    }
                }
                KeyCode::Char(' ') => {
                    // Toggle selection
                    selected[current] = !selected[current];
                }
                KeyCode::Enter => {
                    // Exit on Enter
                    break;
                }
                KeyCode::Esc => {
                    // Exit on Escape
                    disable_raw_mode()?;
                    execute!(stdout, cursor::Show)?;
                    return Ok(());
                }
                _ => {}
            }
        }

        // Clear the screen for the next render
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    }

    // Disable raw mode and clean up
    disable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), cursor::Show)?;

    // Display final selections
    println!("You selected:");
    for (i, option) in options.iter().enumerate() {
        if selected[i] {
            println!("- {}", option);
        }
    }

    Ok(())
}

