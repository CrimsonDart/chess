
use std::time::Instant;
use crossterm::event::{KeyEvent, KeyCode};
use crate::state::{Space, Board, AccessBoard};

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
    let cursor = &mut user.key_cursor;

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



            let cursor = if let Some(c) = user.mouse_cursor {
                c
            } else {
                user.key_cursor
            };

            // gets the space at the cursor location
            let cursor_space = match user.board.read_board(cursor) {
                Some(s) => s,
                None => return
            };

            match (user.selected, cursor_space) {
                (Some(arr), _) => {
                    if arr != cursor {
                        if user.board.move_piece(arr, cursor) {
                            user.turn_white = !user.turn_white;
                        }
                    }
                    user.selected = Option::None;
                    return;
                },
                (_, Space::Open) => {
                    user.selected = Some(cursor);
                },
                (Option::None, piece) => {
                    if piece.is_white() == user.turn_white {
                        user.selected = Some(cursor);
                    }
                }
            }

            if let Some(arr) = user.selected {

                // deselect the space if the user selects it again.
                if arr == cursor {
                    user.selected = Option::None;
                    return;
                }
                // if the user DOESNT select the same space twice...
                else {
                    // if the move is successful (no error)
                    if user.board.move_piece(arr, cursor) {
                        user.turn_white = !user.turn_white;
                        user.selected = Option::None;
                        return;

                    } else {
                        user.selected = Option::None;
                        return;
                    }
                }
            }
            user.selected = Some(cursor);
        },
    }
}
