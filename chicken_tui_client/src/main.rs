use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
};

fn main() -> io::Result<()> {
    // Terminal initialisieren:
    // Raw mode aktiviert, damit Eingaben nicht automatisch echoen oder verarbeitet werden.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // Schaltet in den alternativen Bildschirm, um unsere GUI anzuzeigen,
    // ohne den regulären Terminalinhalt zu überschreiben.
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Variablen für die Benutzereingabe, das korrekte Passwort und Fehlernachricht.
    let mut input = String::new();
    let unlock_password = "1234";
    let mut error_message = String::new();

    loop {
        // Zeichne die Benutzeroberfläche neu.
        terminal.draw(|f| draw_ui(f, &input, &error_message))?;
        
        // Lies das nächste Ereignis (z. B. Tastendruck) – blockierend.
        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, ..}) = event::read()? {
                match code {
                    // Ein Zeichen hinzufügen und bei neuer Eingabe auch die Fehlermeldung löschen.
                    KeyCode::Char(c) => {
                        input.push(c);
                        error_message.clear();
                    },
                    // Entfernt das letzte Zeichen (Backspace).
                    KeyCode::Backspace => {
                        input.pop();
                    },
                    // Bei Enter prüfen wir das eingegebene Passwort.
                    KeyCode::Enter => {
                        // Mit trim() werden führende oder nachfolgende Leerzeichen entfernt.
                        if input.trim() == unlock_password {
                            // Erfolgreich freigeschaltet. Terminal zurücksetzen und das Programm beenden.
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            println!("Unlocked!");
                            std::process::exit(0);
                        } else {
                            // Falsches Passwort: Setze eine Fehlermeldung und lösche die Eingabe.
                            error_message = "Falsches Passwort!".to_string();
                            input.clear();
                        }
                    },
                    // Mit ESC das Programm sofort beenden.
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        std::process::exit(0);
                    },
                    _ => {},
                }
            }
        }
    }
}

fn draw_ui(frame: &mut Frame, input: &str, error_message: &str) {
    // Bestimme die Gesamtfläche des Terminals.
    let area = frame.size();

    // Unterteile den Bildschirm in zwei vertikale Bereiche:
    // Der obere Bereich (50 %) dient zur Anzeige des Titels,
    // der untere Bereich (50 %) wird weiter unterteilt in Eingabe- und Fehlermeldungsfelder.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Percentage(50), Constraint::Percentage(50)]
                .as_ref(),
        )
        .split(area);

    // Obere Region: Einfache Anzeige mit dem Titel "Unlock Screen"
    let header = Block::default()
        .title("Unlock Screen")
        .borders(Borders::ALL);
    frame.render_widget(header, chunks[0]);

    // Teile den unteren Bereich in zwei Zeilen: 
    // die erste Zeile für das Eingabefeld, die zweite Zeile für Fehlermeldungen.
    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Length(3), Constraint::Length(3)]
                .as_ref(),
        )
        .split(chunks[1]);

    // Eingabefeld: Hier wird der aktuelle Stand der Eingabe angezeigt.
    let input_paragraph = Paragraph::new(input)
        .block(Block::default().title("Enter Password").borders(Borders::ALL));
    frame.render_widget(input_paragraph, sub_chunks[0]);

    // Fehleranzeige: Falls vorhanden, wird hier die Fehlermeldung gezeigt.
    let error_paragraph = Paragraph::new(error_message)
        .block(Block::default().title("Error").borders(Borders::ALL));
    frame.render_widget(error_paragraph, sub_chunks[1]);
}
