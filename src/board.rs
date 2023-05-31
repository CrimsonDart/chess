
use crate::{Board, Space::*};

pub const STANDARD_BOARD: Board =
    [[Rook(false), Knight(false), Bishop(false), King(false), Queen(false), Bishop(false), Knight(false), Rook(false)],
    [Pawn(false, false); 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Pawn(true, false); 8],
    [Rook(true), Knight(true), Bishop(true), King(true), Queen(true), Bishop(true), Knight(true), Rook(true)]
    ];

pub const DEBUG_BOARD: Board =
    [[Rook(false), Knight(false), Bishop(false), King(false), Queen(false), Bishop(false), Knight(false), Rook(false)],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Rook(true), Knight(true), Bishop(true), King(true), Queen(true), Bishop(true), Knight(true), Rook(true)]
    ];
