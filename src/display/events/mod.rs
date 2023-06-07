mod key_press;
mod mouse_move;
mod resize;

use std::time::{Duration, Instant};
use crossterm::event::{poll, read, Event};
use super::dynamic::TerminalC;
use crate::{Loc, state::Board};

pub static mut BREAK_LOOP: bool = false;


// positions are stored in IN GAME location,
// in the x,y format.
// the first space is 1,A, not 0,0.
pub struct UserState {

    pub key_cursor: Loc,
    pub selected: Option<Loc>,
    pub cursor_blink: bool,
    pub blink_timer: Instant,
    pub turn_white: bool,
    pub board: Board

}


// routes all events from the terminal to each module.
pub fn start_event_loop(terminal: &mut TerminalC) -> crossterm::Result<()> {

    let mut user_state = UserState {
        key_cursor: [1, 1],
        selected: None,
        cursor_blink: true,
        blink_timer: Instant::now(),
        turn_white: true,
        board: crate::board::STANDARD_BOARD

    };

    'event: loop {
        // `read()` blocks until an `Event` is available

        if poll(Duration::from_millis(1))? {
            match read()? {
                Event::Key(event) => key_press::event(event, &mut user_state),
                //Event::Mouse(event) => mouse_move::event(event),
                //#[cfg(feature = "bracketed-paste")]
                Event::Resize(width, height) => resize::event(width, height),
                _ => ()
                // removed:
                // Paste Event,
                // Focus Gained,
                // Focus Lost.
            }
        }

        let cb = user_state.cursor_blink.clone();

        // cursor blink manager
        if user_state.blink_timer.elapsed() >= Duration::from_millis(500) {
            user_state.cursor_blink = !cb;
            user_state.blink_timer = Instant::now();
        }

        terminal.draw(|f| {
            f.render_widget(&user_state, UserState::get_rect());

        })?;

        if unsafe {BREAK_LOOP} {
            break 'event;
        }
    }
    Ok(())
}
