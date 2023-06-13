use crate::board::{Board, Loc, read_board, do_move};
use crate::piece::KNIGHT_MOVES;
use crate::types::{MoveData, Movement::*, Space, Space::*, Direction};

macro_rules! push_or_return {
    ($is_get_list: ident, $vector: ident, $testc: ident) => {
        if $is_get_list {
            $vector.push($testc);
        } else {
            return Pair::Val2(true);
        }
    };
}


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
        if is_check(&test_board, fromc // TODO make into kings location.
                    , from.is_white()) {
            vector[index] = MoveData {relation: Check, to: toc};
        }
    }
}

pub fn is_check(board: &Board, kingc: Loc, is_white: bool) -> bool {
    match dyn_check(board, kingc, is_white, false) {
        Pair::Val1(_) => panic!("whoops"),
        Pair::Val2(v) => {
            return v;
        }
    }
}

fn dyn_check(board: &Board, kingc: Loc, is_white: bool, is_get_list: bool) -> Pair<Vec<Loc>, bool> {
    let mut vector = Vec::new();

    let from = read_board(board, kingc).unwrap();

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
                push_or_return!(is_get_list, vector, testc);
            }
        }

        let testc = Direction::East.translate(pawn_dir, 1);
        let test = read_board(board, testc);
        if let Some(Pawn(w, _)) = test {
            if w != is_white {
                push_or_return!(is_get_list, vector, testc);
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
                            push_or_return!(is_get_list, vector, testc);
                        }
                        break;
                    },
                    Some(King(w, _)) => if index == 1 {
                        if w != is_white {
                            push_or_return!(is_get_list, vector, testc);
                        }
                        break;
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
                    push_or_return!(is_get_list, vector, testc);
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
                            push_or_return!(is_get_list, vector, testc);
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
    Pair::Val1(vector)
}

enum Pair<T, E> {
    Val1(T),
    Val2(E)
}


