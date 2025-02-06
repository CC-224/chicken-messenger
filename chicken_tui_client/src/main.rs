use std::io;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::{Block, Borders}};

fn main() -> io::Result<()> {
    // Terminal vorbereiten
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Chicken Messenger TUI").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc {
                break; // Beende das Programm mit ESC
            }
        }
    }

    // Terminal zurücksetzen
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
