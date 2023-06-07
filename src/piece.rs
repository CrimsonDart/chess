
use crate::board::read_board;

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
            PawnSkip | Blocked | Castle => {
                break;
            }
        }
    }
}


fn pawn_list(board: &Board, fromc: Loc, from: Space, vector: &mut Vec<MoveData>) {

    let Pawn(is_white, is_moved) = from;

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

    let attack = West.translate(slide, 1);
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

    const knight_moves: [Loc; 8] = [[1,2],[2,1],[-1,2],[-2,1],[1,-2],[2,-1],[-1,-2],[-2,-1]];
    for delta in knight_moves {

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
    for dir in Direction::CARDINALS {

        let toc = dir.translate(fromc, 1);
        let to = read_board(board, toc);

        match to {
            Some(Open) => {
                vector.push(MoveData { relation: Empty, to: toc });
            },
            Some(piece) => {
                if piece.is_white != from.is_white() {
                    vector.push(MoveData { relation: Enemy, to: toc });
                }
            },
            None => {}
        }
    }
}

fn move_list(board: &Board, fromc: Loc) -> Vec<MoveData> {

    let mut vector = Vec::new();
    let from = read_board(board, fromc);
    let from = match from {
        Some(p) => p,
        None => {return vector;}
    };

    match from {
        Pawn(_,_) => {
            pawn_list(board, fromc, from, &mut vector);
        },
        Rook(_) => {
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
        King(_) => {
            king_list(board, fromc, from, &mut vector);
        },
        Open => {}
    }
    vector
}
