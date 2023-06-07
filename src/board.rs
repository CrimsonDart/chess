
use super::types::{Space::*, Space, PawnState};


pub type Loc = [isize; 2];
pub type Board = [[Space; 8];8];





pub const STANDARD_BOARD: Board =
    [[Rook(false), Knight(false), Bishop(false), King(false), Queen(false), Bishop(false), Knight(false), Rook(false)],
    [Pawn(false, PawnState::NotMoved); 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Pawn(true, PawnState::NotMoved); 8],
    [Rook(true), Knight(true), Bishop(true), King(true), Queen(true), Bishop(true), Knight(true), Rook(true)]
    ];

pub const NO_PAWNS: Board =
    [[Rook(false), Knight(false), Bishop(false), King(false), Queen(false), Bishop(false), Knight(false), Rook(false)],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Rook(true), Knight(true), Bishop(true), King(true), Queen(true), Bishop(true), Knight(true), Rook(true)]
    ];





pub fn read_board(board: &Board, r: Loc) -> Option<Space> {
    if r[0] < 1 || r[0] > 8 || r[1] < 1 || r[1] > 8 {
        return None;
    }

    Some(board[r[1] as usize - 1][r[0] as usize - 1])
}
