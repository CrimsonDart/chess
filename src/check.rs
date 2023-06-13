
use crate::board::{Board, Loc, read_board, do_move};
use crate::piece::{KNIGHT_MOVES, move_list};
use crate::types::{MoveData, Movement::*, Space, Space::*, Direction};

pub fn deep_checks(board: &Board, fromc: Loc, vector: &mut Vec<MoveData>) {

    for index in 0..vector.len() {
        //println!("iteration {} of {}", index, vector.len());

        let (toc, relation) = {
            let movement = vector[index];
            (movement.to, movement.relation)
        };

        let mut test_board = board.clone();
        let from = read_board(&test_board, fromc).unwrap();

        do_move(&mut test_board, fromc, from, toc, relation);
        if is_check(&test_board, get_king(board, from.is_white()).unwrap(), from.is_white()) {
            vector[index] = MoveData {relation: Check, to: toc};
        }
    }
}

fn get_king(board: &Board, is_white: bool) -> Option<Loc> {

    for x in 1..9 {
        for y in 1..9 {
            let test = read_board(board, [x, y]);
            if let Some(King(w, _)) = test {
                if w == is_white {
                    return Some([x, y]);
                }
            }
        }
    }
    None
}

pub fn is_check(board: &Board, kingc: Loc, is_white: bool) -> bool {

    // checks for pawns first :)
    'pawn: {
        let pawn_dir = match is_white {
            true => {
                Direction::North
            },
            false => {
                Direction::South
            }
        }.translate(kingc, 1);

        let testc = Direction::West.translate(pawn_dir, 1);
        let test = read_board(board, testc);
        if let Some(Pawn(w, _)) = test {
            if w != is_white {
                return true;
            }
        }

        let testc = Direction::East.translate(pawn_dir, 1);
        let test = read_board(board, testc);
        if let Some(Pawn(w, _)) = test {
            if w != is_white {
                return true;
            }
        }
    }

    'rook: {
        for dir in Direction::CARDINALS {
            for index in 1..9 {
                let testc = dir.translate(kingc, index);
                let test = read_board(board, testc);

                match test {
                    Some(Rook(w, _)) | Some(Queen(w)) => {

                        if w != is_white {
                            return true;
                        }
                        break;
                    },
                    Some(King(w, _)) => if index == 1 {
                        if w != is_white {
                            return true;
                        }
                    }
                    Some(Open) => {},
                    Some(_) | None => {
                        break;
                    },
                }
            }
        }
    }

    'knight: {
        for delta in KNIGHT_MOVES {

            let testc = [kingc[0] + delta[0], kingc[1] + delta[1]];
            let test = read_board(board, testc);
            match test {
                Some(Knight(w)) => if w != is_white {
                    return true;
                },
                _ => {}
            }
        }
    }

    'bishop: {
        for dir in Direction::ORDINALS {
            for index in 1..9 {
                let testc = dir.translate(kingc, index);
                let test = read_board(board, testc);

                match test {
                    Some(Bishop(w)) | Some(Queen(w)) => {
                        if w != is_white {
                            return true;
                        }
                        break;
                    },
                    Some(Open) => {},
                    Some(_) | None => {
                        break;
                    }
                }
            }
        }
    }
    false
}

// the 'is_white' variable is the team that is defending;
// the one getting "checkmated" if you will.
pub fn is_checkmate(board: &Board, is_white: bool) -> bool {


    let kingc = get_king(board, is_white).unwrap();

    // compiles all opposing pieces.

    if !is_check(board, kingc, is_white) {
        return false;
    }

    let mut vector = Vec::new();

    for y in 1..9 {
        for x in 1..9 {
            let testc = [x, y];
            let test = read_board(board, testc);

            match test {
                Some(Open) | None => {
                    continue;
                },
                Some(piece) => if piece.is_white() == is_white {
                    vector.push((testc, piece));
                }
            }
        }
    }

    for (fromc, from) in vector {
        for movement in move_list(board, fromc, from) {

            let mut test_board = board.clone();
            do_move(&mut test_board, fromc, from, movement.to, movement.relation);
            let kingc = if let King(_, _) = from {
                movement.to
            } else {
                kingc
            };

            if !is_check(&test_board, kingc, is_white) {
                return false;
            }
        }
    }
    true
}
