
use std::env;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut text = String::new();
    let mut filename = String::new();
    let mut is_editing_filename = false;
    let mut terminal = Terminal::new(backend)?;
   
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        filename = args[1].clone();
        if let Ok(contents) = load_file(&filename) {
            text = contents;
        } else {
            println!("File not found, creating a new file.");
        }
    } else {
        is_editing_filename = true;
    }

    
    let mut text = String::new();
    loop {

        if !is_editing_filename {
            if let Ok(contents) = load_file(&filename) {
                text = contents;
            } 
            // Set is_editing_filename to true so that it doesn't load the content every frame.
            is_editing_filename = true; 
        } 


        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1), // Filename bar
                        Constraint::Min(1),    // Text area
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let filename_text = if is_editing_filename {
                filename.clone()
            } else {
                filename.clone() + " (press Enter to edit)"
            };
            let filename_block = Paragraph::new(filename_text)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Filename"));
            f.render_widget(filename_block, chunks[0]);
            let text_block = Paragraph::new(text.clone())
                .block(Block::default().borders(Borders::ALL).title("Editor"));
            f.render_widget(text_block, chunks[1]);
        })?;

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char(c) => {
                    if is_editing_filename {
                        filename.push(c);
                    } else {
                        text.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if is_editing_filename {
                        filename.pop();
                    } else {
                        text.pop();
                    }
                }
                KeyCode::Enter => {
                    if is_editing_filename {
                        is_editing_filename = false;

                        // If file does not exist or filename is empty, start with an empty file
                        if let Ok(contents) = load_file(&filename) {
                            text = contents;
                        } else {
                            text = String::new(); // Start with empty content
                        }
                    } else {
                        text.push('\n'); // Insert newline
                    }
                }
                KeyCode::F(2) => {
                    if !filename.is_empty() {
                        match save_file(&filename, &text) {
                            Ok(_) => println!("File saved successfully!"),
                            Err(e) => println!("Error saving file: {}", e),
                        }
                    } else {
                        println!("Please enter a filename before saving.");
                    }
                }
                KeyCode::Esc => break, // Exit
                _ => {}
            }
        }
    }    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    // ... (same cleanup as before)

    Ok(())
}

// Function to save the file
fn save_file(filename: &str, text: &str) -> io::Result<()> {
    fs::write(filename, text)
}

// Function to load the file
fn load_file(filename: &str) -> io::Result<String> {
    let mut file = fs::File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

