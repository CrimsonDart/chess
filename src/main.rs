use Space::*;
use std::fmt::{Display, Formatter, Error};

fn main() {

    print_board();




}

fn print_board() {
    println!("~ A  B  C  D  E  F  G  H ");

    let mut i: u8 = 0;

    for row in BOARD.iter().rev() {
        i = i + 1;
        println!("{} {} {} {} {} {} {} {} {}", i, row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]);
    }
}

static BOARD: [[Space; 8]; 8] =
    [[WhiteRook, WhiteKnight, WhiteBishop, WhiteKing, WhiteQueen, WhiteBishop, WhiteKnight, WhiteRook],
    [WhitePawn; 8],
    [Space::None; 8],
    [Space::None; 8],
    [Space::None; 8],
    [Space::None; 8],
    [BlackPawn; 8],
    [BlackRook, BlackKnight, BlackBishop, BlackKing, BlackQueen, BlackBishop, BlackKnight, BlackRook]
    ];

#[derive(Clone, Copy, Debug)]
enum Space {
    None,

    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,

    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}",
               match self {
                   None => "__",

                   WhitePawn => "WP",
                   WhiteRook => "WR",
                   WhiteKnight => "WN",
                   WhiteBishop => "WB",
                   WhiteQueen => "WQ",
                   WhiteKing => "WK",

                   BlackPawn => "BP",
                   BlackRook => "BR",
                   BlackKnight => "BN",
                   BlackBishop => "BB",
                   BlackQueen => "BQ",
                   BlackKing => "BK",
               }
        )
    }
}
