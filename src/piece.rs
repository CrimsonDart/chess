
use crate::board::{read_board, do_move};
use crate::check::is_check;

use super::types::{Direction, Direction::*, MoveData, Space, Space::*, Movement, Movement::*, PawnState };
use super::board::{Board, Loc};


fn get_relation(board: &Board, fromc: Loc, toc: Loc) -> Movement {

    let from = read_board(board, fromc);
    let to = read_board(board, toc);

    let from = match from {
        Some(Open) | None => {return Blocked;},
        Some(piece) => piece
    };

    match to {
        Some(Open) => Empty,
        Some(piece) => {
            match from.is_white() != piece.is_white() {
                true => Enemy,
                false => Blocked
            }
        },
        None => Blocked
    }
}

fn test_line(board: &Board, fromc: Loc, dir: Direction, vector: &mut Vec<MoveData>) {

    for index in 1..9 {

        let toc = dir.translate(fromc, index);
        let relation = get_relation(board, fromc, toc);

        match relation {
            Empty | Check => {
                vector.push(MoveData {relation, to: toc})
            },
            Enemy => {
                vector.push(MoveData {relation, to: toc});
                break;
            },
            PawnSkip | Blocked | QueenSide | KingSide => {
                break;
            }
        }
    }
}

fn pawn_list(board: &Board, fromc: Loc, from: Space, vector: &mut Vec<MoveData>) {

    let is_white = from.is_white();

    let dir = match is_white {
        true => South,
        false => North
    };

    let slide = dir.translate(fromc, 1);
    let relation = get_relation(board, fromc, slide);

    if relation == Empty {
        vector.push(MoveData {relation, to: slide});
        let slide = dir.translate(fromc, 2);
        let relation = get_relation(board, fromc, slide);
        if relation == Empty {
            vector.push(MoveData {relation, to: slide});
        }
    }

    let attack = East.translate(slide, 1);
    let relation = get_relation(board, fromc, attack);
    if relation == Enemy {
        vector.push(MoveData {relation, to: attack});
    }

    let attack = West.translate(slide, 1);
    let relation = get_relation(board, fromc, attack);
    if relation == Enemy {
        vector.push( MoveData {relation, to: attack} );
    }
}

fn rook_list(board: &Board, fromc: Loc, from: Space, vector: &mut Vec<MoveData>) {
    for dir in Direction::CARDINALS {
        test_line(board, fromc, dir, vector);
    }
}

fn knight_list(board: &Board, fromc: Loc, from: Space, vector: &mut Vec<MoveData>) {

    const KNIGHT_MOVES: [Loc; 8] = [[1,2],[2,1],[-1,2],[-2,1],[1,-2],[2,-1],[-1,-2],[-2,-1]];
    for delta in KNIGHT_MOVES {

        let toc = [fromc[0] + delta[0], fromc[1] + delta[1]];
        let relation = get_relation(board, fromc, toc);
        if relation == Blocked {
            continue;
        }
        vector.push(MoveData { relation, to: toc });
    }
}

fn bishop_list(board: &Board, fromc: Loc, from: Space, vector: &mut Vec<MoveData>) {
    for dir in Direction::ORDINALS {
        test_line(board, fromc, dir, vector);
    }
}

fn queen_list(board: &Board, fromc: Loc, from: Space, vector: &mut Vec<MoveData>) {
    for dir in Direction::ORDINALS {
        test_line(board, fromc, dir, vector);
    }
    for dir in Direction::CARDINALS {
        test_line(board, fromc, dir, vector);
    }
}

fn king_list(board: &Board, fromc: Loc, from: Space, vector: &mut Vec<MoveData>) {

    let (is_white, has_moved) = match from {
        King(w, m) => (w, m),
        _ => {return;}
    };

    for dir in Direction::CARDINALS {

        let toc = dir.translate(fromc, 1);
        let to = read_board(board, toc);

        match to {
            Some(Open) => {
                vector.push(MoveData { relation: Empty, to: toc });
            },
            Some(piece) => {
                if piece.is_white() != is_white {
                    vector.push(MoveData { relation: Enemy, to: toc });
                }
            },
            None => {}
        }
    }

    // castle rules :)
    // cant castle if the king has moved.
    if has_moved {return;}

    //cant castle if king is in check
    if is_check(board, from.is_white()) {return;}

    castle_check(board, fromc, from, East, vector);
    castle_check(board, fromc, from, West, vector);
}

fn castle_check(board: &Board, fromc: Loc, from: Space, dir: Direction, vector: &mut Vec<MoveData>) {
    let rook = read_board(board, dir.translate(fromc,
        if dir == East {
            3
        } else {
            4
        }
    ));

    if let Some(Rook(_, false)) = rook {
    } else {
        return;
    }

    for n in [1, 2] {

        let toc = dir.translate(fromc, n);

        if !(read_board(board, toc) == Some(Open)) {
            return;
        }

        let mut test_board = board.clone();
        do_move(&mut test_board, fromc, from, toc, Empty);
        if is_check(&test_board, from.is_white()) {
            return;
        }
    }
    vector.push(MoveData { relation: match dir {
        East => KingSide,
        West => QueenSide,
        _ => {return;}
    }, to: dir.translate(fromc, 2) });
}

pub fn move_list(board: &Board, fromc: Loc, from: Space) -> Vec<MoveData> {

    let mut vector = Vec::new();

    match from {
        Pawn(_,_) => {
            pawn_list(board, fromc, from, &mut vector);
        },
        Rook(_, _) => {
            rook_list(board, fromc, from, &mut vector);
        },
        Knight(_) => {
            knight_list(board, fromc, from, &mut vector);
        },
        Bishop(_) => {
            bishop_list(board, fromc, from, &mut vector);
        },
        Queen(_) => {
            queen_list(board, fromc, from, &mut vector);
        },
        King(_, _) => {
            king_list(board, fromc, from, &mut vector);
        },
        Open => {}
    }
    vector
}
