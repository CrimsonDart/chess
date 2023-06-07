



fn get_opposing_pieces(board: &Board, is_white: bool) -> Vec<Loc> {
    let mut vector = Vec::new();

    for iy in 0..8 {
        for ix in 0..8 {
            let space = board[iy][ix];
            if space == Open {continue;}
            if space.is_white() == is_white {continue;}
            vector.push([(ix + 1) as isize, (iy + 1) as isize]);
        }
    }
    vector
}

fn is_check(board: &Board, is_white: bool) -> bool {

    for pc in get_opposing_pieces(board, is_white) {

        let moveset = board.move_list(pc);
        for movement in moveset {
            let test = board.read_board(movement.to);

            if let Some(King(_)) = test {
                return true;
            }
        }
    }
    return false;
}

pub fn deep_checks(board: &Board, fromc: Loc, is_white: bool, vector: &mut Vec<MoveData>) {

    for index in 0..vector.len() {
        //println!("iteration {} of {}", index, vector.len());

        let movement = vector[index];

        let mut test_board = board.clone();

        test_board.do_move(fromc, movement);

        if is_check(&test_board, is_white) {
            vector[index] = MoveData::new(Movement::Check, movement.to);
        }
    }
}
