

use crate::{piece::move_list, check::deep_checks, types::Direction};

use super::types::{Space::*, Space, PawnState, Movement, Movement::*};


pub type Loc = [isize; 2];
pub type Board = [[Space; 8];8];





pub const STANDARD_BOARD: Board =
    [[Rook(false, false), Knight(false), Bishop(false), Queen(false), King(false, false), Bishop(false), Knight(false), Rook(false, false)],
    [Pawn(false, PawnState::NotMoved); 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Pawn(true, PawnState::NotMoved); 8],
    [Rook(true, false), Knight(true), Bishop(true), Queen(true), King(true, false), Bishop(true), Knight(true), Rook(true, false)]
    ];

pub const NO_PAWNS: Board =
    [[Rook(false, false), Knight(false), Bishop(false), Queen(false), King(false, false), Bishop(false), Knight(false), Rook(false, false)],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Rook(true, false), Knight(true), Bishop(true), Queen(true), King(true, false), Bishop(true), Knight(true), Rook(true, false)]
    ];

pub fn read_board(board: &Board, r: Loc) -> Option<Space> {
    if r[0] < 1 || r[0] > 8 || r[1] < 1 || r[1] > 8 {
        return None;
    }

    Some(board[r[1] as usize - 1][r[0] as usize - 1])
}

pub fn write_board(board: &mut Board, loc: Loc, space: Space) -> bool {
    if loc[0] < 1 || loc[0] > 8 || loc[1] < 1 || loc[1] > 8 {
        return false;
    }

    board[loc[1] as usize - 1][loc[0] as usize - 1] = space;
    true
}

pub fn move_piece(board: &mut Board, fromc: Loc, from: Space, toc: Loc, to: Space) -> bool {

    let mut moveset = move_list(board, fromc, from);
    print!("len: {}", moveset.len());
    deep_checks(board, fromc, from.is_white(), &mut moveset);

    for valid_move in moveset {
        if valid_move.to == toc {
            return do_move(board, fromc, from, toc, valid_move.relation);
        }
    };
    return false;
}

pub fn do_move(board: &mut Board, fromc: Loc, from: Space, toc: Loc, relation: Movement) -> bool {

    match relation {
        PawnSkip => {
            write_board(board, fromc, Open);
            write_board(board, toc, Pawn(from.is_white(), PawnState::PrevSkipped))
        },
        QueenSide | KingSide => {

            let dir = if relation == KingSide {
                Direction::East
            } else {
                Direction::West
            };

            write_board(board, fromc, Open);
            write_board(board, toc, King(from.is_white(), true));

            write_board(board, dir.translate(fromc, 1), Rook(from.is_white(), true));
            write_board(board, dir.translate(fromc,
                if dir == Direction::East {
                    3
                } else {
                    4
                }
            ), Open)
        },

        Enemy | Empty => {

            match from {
                King(w, _) => {
                    write_board(board, toc, King(w, true));
                },
                Rook(w, _) => {
                    write_board(board, toc, Rook(w, true));
                },
                Pawn(w, state) => {

                    write_board(board, toc, Pawn(w,
                        match state {
                            PawnState::NotMoved => PawnState::Moved,
                            PawnState::PrevSkipped => PawnState::Moved,
                            PawnState::Moved => PawnState::Moved
                        }
                    ));
                },
                _ => {
                    write_board(board, toc, from);
                }
            }

            write_board(board, fromc, Open)
        },
        Blocked | Check => {
            false
        }
    }
}
