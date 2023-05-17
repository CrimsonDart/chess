use std::{io, thread, time::Duration};
use ratatui::{
    backend::CrosstermBackend, Terminal,
    widgets::{Block, Borders},
};
use crossterm::{
    execute,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

use super::events;

pub type TerminalC = Terminal<CrosstermBackend<io::Stdout>>;

pub fn open_term() -> Result<TerminalC, io::Error> {


    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal: TerminalC = Terminal::new(backend)?;
    Ok(terminal)
}

fn close_term(mut terminal: TerminalC) -> Result<(), io::Error> {
    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

// starts the terminal, and runs the event loop.
pub fn start_terminal() -> Result<(), io::Error> {
    let mut terminal = open_term()?;

    let res = draw(&mut terminal);
    let res2 = events::start_event_loop(&mut terminal);

    close_term(terminal)?;
    res?;
    res2?;

    Ok(())
}


pub fn draw(terminal: &mut TerminalC) -> Result<(), io::Error> {

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    // restore terminal
    Ok(())
}
