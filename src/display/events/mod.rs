mod key_press;
mod mouse_move;
mod resize;

use crossterm::event::{read, Event};

use super::{dynamic::TerminalC, widget::DisplayState};

pub static mut BREAK_LOOP: bool = false;

// routes all events from the terminal to each module.
pub fn start_event_loop(terminal: &mut TerminalC) -> crossterm::Result<()> {














    'event: loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => key_press::event(event),
            Event::Mouse(event) => mouse_move::event(event),
            //#[cfg(feature = "bracketed-paste")]
            Event::Resize(width, height) => resize::event(width, height),
            _ => ()
            // removed:
            // Paste Event,
            // Focus Gained,
            // Focus Lost.
        }

        let state = DisplayState {
            board: &crate::get_board()
        };

        terminal.draw(|f| {
            f.render_widget(state, DisplayState::get_rect());
        })?;

        if unsafe {BREAK_LOOP} {
            break 'event;
        }
    }
    Ok(())
}
