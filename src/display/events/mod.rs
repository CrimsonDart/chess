mod key_press;
mod mouse_move;
mod resize;

use std::time::Duration;
use crossterm::event::{poll, read, Event};
use super::{dynamic::TerminalC, widget::DisplayState};

pub static mut BREAK_LOOP: bool = false;


// positions are stored in IN GAME location,
// in the x,y format.
// the first space is 1,A, not 0,0.
pub struct UserState {

    pub key_cursor: [u8; 2],
    pub mouse_cursor: Option<[u8; 2]>,
    pub selected: Option<[u8; 2]>,
    pub cursor_blink: CursorBlink

}

// each value is a cooldown until
// the enum transitions to its next state.
pub enum CursorBlink {
    Cooldown(u16),
    On(u16),
    Off(u16)
}




// routes all events from the terminal to each module.
pub fn start_event_loop(terminal: &mut TerminalC) -> crossterm::Result<()> {


    let mut user_state = UserState {
        key_cursor: [1, 1],
        mouse_cursor: None,
        selected: None,
        cursor_blink: CursorBlink::Cooldown(0)

    };












    'event: loop {
        // `read()` blocks until an `Event` is available

        if poll(Duration::from_millis(1))? {
            match read()? {
                Event::Key(event) => key_press::event(event, &mut user_state),
                Event::Mouse(event) => mouse_move::event(event),
                //#[cfg(feature = "bracketed-paste")]
                Event::Resize(width, height) => resize::event(width, height),
                _ => ()
                // removed:
                // Paste Event,
                // Focus Gained,
                // Focus Lost.
            }
        }

        match user_state.cursor_blink {
            CursorBlink::Cooldown(time) => {
                if time == 0 {
                    user_state.cursor_blink = CursorBlink::Off(200);
                } else {
                    user_state.cursor_blink = CursorBlink::Cooldown(time - 1);
                }
            },
            CursorBlink::On(time) => {
                if time == 0 {
                    user_state.cursor_blink = CursorBlink::Off(200);
                } else {
                    user_state.cursor_blink = CursorBlink::On(time - 1);
                }
            },
            CursorBlink::Off(time) => {
                if time == 0 {
                    user_state.cursor_blink = CursorBlink::On(200);
                } else {
                    user_state.cursor_blink = CursorBlink::Off(time - 1);
                }
            }
        }





        let pos = DisplayState {
            board: &crate::get_board(),
            user: &user_state
        };

        terminal.draw(|f| {
            f.render_widget(pos, DisplayState::get_rect());

        })?;

        if unsafe {BREAK_LOOP} {
            break 'event;
        }
    }
    Ok(())
}
