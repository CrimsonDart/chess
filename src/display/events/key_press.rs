
use crossterm::{event::{KeyEvent, KeyCode}};



pub fn event(e: KeyEvent) {
    let key = e.code;

    if key == KeyCode::Esc {

        println!("quitting program");

        unsafe{
            crate::display::events::BREAK_LOOP = true;
        }
    }

}
