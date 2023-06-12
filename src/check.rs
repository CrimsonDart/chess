use crate::board::{Board, Loc, read_board, do_move};
use crate::piece::move_list;
use crate::types::{MoveData, Movement::*, Space, Space::*};

fn get_opposing_pieces(board: &Board, is_white: bool) -> Vec<(Loc, Space)> {
    let mut vector = Vec::new();

    for iy in 0..8 {
        for ix in 0..8 {
            let space = board[iy][ix];
            if space == Open {continue;}
            if space.is_white() == is_white {continue;}
            vector.push(([(ix + 1) as isize, (iy + 1) as isize], space));
        }
    }
    vector
}

pub fn is_check(board: &Board, is_white: bool) -> bool {

    for pc in get_opposing_pieces(board, is_white) {

        let moveset = move_list(board, pc.0, pc.1);
        for movement in moveset {
            let test = read_board(board, movement.to);

            if let Some(King(_, _)) = test {
                return true;
            }
        }
    }
    false
}

pub fn deep_checks(board: &Board, fromc: Loc, is_white: bool, vector: &mut Vec<MoveData>) {

    print!("len: {}", vector.len());
    for index in 0..vector.len() {
        //println!("iteration {} of {}", index, vector.len());

        let (toc, relation) = {
            let movement = vector[index];
            (movement.to, movement.relation)
        };

        let mut test_board = board.clone();
        let from = read_board(&test_board, fromc).unwrap();

        do_move(&mut test_board, fromc, from, toc, relation);

        if is_check(&test_board, is_white) {
            vector[index] = MoveData {relation: Check, to: toc};
        }
    }
}
