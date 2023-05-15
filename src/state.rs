use Piece::*;
use std::fmt::{Display, Formatter, Error};
use std::fmt::Write as _;

pub fn print_board() {

    let mut s = String::new();
    write!(&mut s, "~ A  B  C  D  E  F  G  H \n");
    let mut i: u8 = 9;

    for row in BOARD.iter().rev() {
        i = i - 1;

        write!(&mut s, "{} ", i);

        for space in row {
            match space {
                Some(data) => {
                    write!(&mut s, "{}{} ",
                           if data.1 {
                               "W"
                           } else {
                               "B"
                           },

                           match data.0 {
                               Pawn => "P",
                               Rook => "R",
                               Knight => "N",
                               Bishop => "B",
                               Queen => "Q",
                               King => "K"
                           }
                       );
                },
                None => {write!(&mut s, "__ ");}
            }
        }
        write!(&mut s, "\n");
    }
    print!("{}", s);
}




static BOARD: [[Option<Space>; 8]; 8] =
    [[Some(Space::new(Rook, true)), Some(Space::new(Knight, true)), Some(Space::new(Bishop, true)), Some(Space::new(King, true)),
      Some(Space::new(Queen, true)), Some(Space::new(Bishop, true)), Some(Space::new(Knight, true)), Some(Space::new(Rook, true))],
    [Some(Space::new(Pawn, true)); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [Some(Space::new(Pawn, false)); 8],
    [Some(Space::new(Rook, false)), Some(Space::new(Knight, false)), Some(Space::new(Bishop, false)), Some(Space::new(King, false)),
     Some(Space::new(Queen, false)), Some(Space::new(Bishop, false)), Some(Space::new(Knight, false)), Some(Space::new(Rook, false))]
    ];


pub fn read_board(x: usize, y: usize) -> Result<Option<Space>, &'static str> {

    if x < 0 || x > 7 || y < 0 || y > 7 {
        return Err("Index out of Bounds!");
    }

    Ok(BOARD[y][x])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Space (Piece, bool);

impl Space {
    const fn new(piece: Piece, is_white: bool) -> Self {
        Space(piece, is_white)
    }

    /*
    fn move_list(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut vector: Vec<(usize, usize)> = Vec::new();

        match self.piece {

            Pawn => {

                let m = if self.is_white {
                    y-1
                } else {
                    y+1
                };

                // pushes the space in front of the pawn if it's empty.
                let forward = read_board(x, m)
                if read_board(x, m).is_ok() == None {
                    vector.push((x, m));
                }

                // pushes diagonal to
                let diagonal_p = read_board(x + 1, m)?;
                let diagonal_n = read_board(x - 1, m)?;









                if self.is_white != BOARD[x - 1][m] {
                    vector.push((x-1, m));
                }
                if self.get_color() == BOARD[x + 1][m].get_color().opposite() {
                    vector.push((x+1, m));
                }
            },

            WhiteRook | BlackRook => {


            },



            _ => ()
        }










        vector
    }
    */
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}{}",
               if self.1 {
                   "W"
               } else {
                   "B"
               },

               match self.0 {
                   Pawn => "P",
                   Rook => "R",
                   Knight => "N",
                   Bishop => "B",
                   Queen => "Q",
                   King => "K"
               }





                       )
    }
}

