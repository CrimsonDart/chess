
use std::time::Instant;
use crossterm::event::{KeyEvent, KeyCode};
use crate::{board::{read_board, move_piece}, types::Space};

use super::UserState;


enum Action {
    Up,
    Left,
    Down,
    Right,
    Select,
}

pub fn event(e: KeyEvent, user: &mut UserState) {
    let key = e.code;

    // quit the loop
    if key == KeyCode::Esc {
        println!("quitting program");
        unsafe{
            crate::display::events::BREAK_LOOP = true;
        }
    }


    // other stuff
    use Action::*;

    let action = match key {

        KeyCode::Char(letter) => match letter {
            'w' | 'k' => Action::Up,
            'a' | 'h' => Action::Left,
            's' | 'j' => Action::Down,
            'd' | 'l' => Action::Right,
            ' ' => Select,
            _ => {return;}

        },
        KeyCode::Enter => Select,
        KeyCode::Up => Up,
        KeyCode::Left => Left,
        KeyCode::Down => Down,
        KeyCode::Right => Right,

        _ => {return;}

    };

    act(action, user);
}


fn act(action: Action, user: &mut UserState) {
    let cursor = &mut user.cursor;

    user.cursor_blink = true;
    user.blink_timer = Instant::now();

    use Action::*;
    match action {
        Up => {
            if cursor[1] != 1 {
                cursor[1] = cursor[1] - 1;
            }
        },
        Left => {
            if cursor[0] != 1 {
                cursor[0] = cursor[0] - 1;
            }
        },
        Down => {
            if cursor[1] != 8 {
                cursor[1] = cursor[1] + 1;
            }
        },
        Right => {
            if cursor[0] != 8 {
                cursor[0] = cursor[0] + 1;
            }
        },
        Select => {

            let cursor = user.cursor;
            let selection = user.selected;

            // gets the space at the cursor location
            let cursor_space = match read_board(&user.board, cursor) {
                Some(s) => s,
                None => return
            };

            match selection {

                // if a space is already selected, then try to move the piece at "selection" to "cursor"

                Some(select) => {

                    let select_piece = match read_board(&user.board, select) {
                        Some(p) => p,
                        None => return
                    };

                    if select_piece == Space::Open {
                        user.selected = Some(cursor);
                        return;
                    }

                    if select != cursor &&
                        move_piece(&mut user.board, select, select_piece, cursor, cursor_space) {
                        user.turn_white = !user.turn_white;
                    }

                    user.selected = None;
                    return;
                },
                None => {
                    if cursor_space == Space::Open || cursor_space.is_white() == user.turn_white {
                        user.selected = Some(cursor);
                        return;
                    }
                }
            }
        },
    }
}
