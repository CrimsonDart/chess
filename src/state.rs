use Piece::*;
use std::fmt::{Display, Formatter, Error};







pub fn get_board() -> &'static [[Option<Space>; 8]; 8] {
    &BOARD
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

// gets the Piece at the given input space.
// returns Err if out of bounds, returns Ok(None) if empty.
//
// coordinates are real board space, not zeroed.
pub fn read_board(x: usize, y: usize) -> Result<Option<Space>, &'static str> {

    if x < 1 || x > 8 || y < 1 || y > 8 {
        return Err("Index out of Bounds!");
    }

    Ok(BOARD[y - 1][x - 1])
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

impl Into<char> for Piece {
    fn into(self) -> char {

        match self {
            Pawn => 'P',
            Rook => 'R',
            Knight => 'N',
            Bishop => 'B',
            Queen => 'Q',
            King => 'K'
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Space (pub Piece, pub bool);

impl Space {
    pub const fn new(piece: Piece, is_white: bool) -> Self {
        Space(piece, is_white)
    }

    fn test_line(x:usize, y:usize, dx: isize, dy: isize, is_white: bool, vector: &mut Vec<(usize, usize)>) {

        let mut index: isize = 1;
        let mut not_ended = true;

        while not_ended {

            let (cx, cy): (usize, usize) =
                ((isize::try_from(x).unwrap() + (index * dx)).try_into().unwrap(),
                                            (isize::try_from(y).unwrap() + (index * dy)).try_into().unwrap());


            let space = read_board(cx, cy);

            if let Ok(Some(p)) = space {
                if p.1 != is_white {
                    vector.push((cx, cy));
                    not_ended = false;
                    println!("enemy space");
                } else {
                    not_ended = false;
                    println!("ally space");
                }
            } else if let Ok(None) = space {
                vector.push((cx, cy));
                println!("Empty Space");
            } else {
                println!("err");
                not_ended = false;
            }
            index = index + 1;
        }
    }


    pub fn move_list(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut vector: Vec<(usize, usize)> = Vec::new();

        match self.0 {

            Pawn => {

                // pushes the space in front of the pawn, if empty

                let m = match self.1 {
                    true => y-1,
                    false => y+1
                };

                if let Ok(space) = read_board(x, m) {
                    if space == None {
                        vector.push((x, m));
                    }
                }

                //diagonal attacks

                if let Ok(Some(p)) = read_board(x-1, m) {
                    if p.1 != self.1 {
                        vector.push((x-1, m));
                    }
                }

                if let Ok(Some(p)) = read_board(x+1, m) {
                    if p.1 != self.1 {
                        vector.push((x+1, m));
                    }
                }
            },

            Rook => {
                Space::test_line(x, y, 0, 1, self.1, &mut vector);
                Space::test_line(x, y, 0, -1, self.1, &mut vector);
                Space::test_line(x, y, -1, 0, self.1, &mut vector);
                Space::test_line(x, y, 1, 0, self.1, &mut vector);
            }
            Knight => {/*TODO*/},
            Bishop => {
                Space::test_line(x, y, 1, 1, self.1, &mut vector);
                Space::test_line(x, y, -1, 1, self.1, &mut vector);
                Space::test_line(x, y, 1, -1, self.1, &mut vector);
                Space::test_line(x, y, -1, -1, self.1, &mut vector);

            }



            _ => ()
        }
        vector
    }

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

