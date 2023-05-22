use PieceType::*;

pub fn get_board() -> &'static [[Space; 8]; 8] {
    unsafe {
        &BOARD
    }
}

pub type Space = Option<Piece>;

const fn space_new(piece_type: PieceType, is_white: bool) -> Space {
    Some(Piece::new(piece_type, is_white))
}

static mut BOARD: [[Space; 8]; 8] =
    [[space_new(Rook, false), space_new(Knight, false), space_new(Bishop, false), space_new(King, false),
      space_new(Queen, false), space_new(Bishop, false), space_new(Knight, false), space_new(Rook, false)],
    [space_new(Pawn(false), false); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [space_new(Rook, true), space_new(Knight, true), space_new(Bishop, true), space_new(King, true),
     space_new(Queen, true), space_new(Bishop, true), space_new(Knight, true), space_new(Rook, true)]
    ];

// gets the Piece at the given input space.
// returns Err if out of bounds, returns Ok(None) if empty.
//
// coordinates are real board space, not zeroed.
pub fn read_board(x: isize, y: isize) -> Result<Space, &'static str> {

    if x < 1 || x > 8 || y < 1 || y > 8 {
        return Err("Index out of Bounds!");
    }

    unsafe {
        Ok(BOARD[y as usize - 1][x as usize - 1])
    }
}

pub fn write_board(x: isize, y: isize, space: Space) -> Result<(), &'static str> {

    if x < 1 || x > 8 || y < 1 || y > 8 {
        return Err("Index out of Bounds!");
    }

    unsafe {
        BOARD[y as usize - 1][x as usize - 1] = space;
        Ok(())
    }
}

// Tries to move a piece from [fx, fy] to [tx, ty]
// returns an error if unsuccsessful.
pub fn move_piece(fx: isize, fy: isize, tx: isize, ty: isize) -> bool {

    let from = read_board(fx, fy)?;
    let to = read_board(tx, ty)?;

    if let None = from {
        return false;
    }

    let from = from.unwrap();
    let mut is_valid = false;
    let mut interaction = PieceInteraction::Empty;

    for valid_move in from.move_list(fx, fy) {
        if valid_move.0 == tx && valid_move.1 == ty {
            is_valid = true;
            interaction = valid_move.2;
            break;
        }
    }

    if !is_valid {
        return false;
    }

    write_board(fx, fy, None)?;

    // if the movement was a Pawn Skip, (which can be performed only once)
    // disables the pawn skip.

    use PieceInteraction::*;
    return match match interaction {
        PawnSkip => write_board(tx, ty, Some(Piece(PieceType::Pawn(true), from.1))),
        KingRookSwap => {
            write_board(tx, ty, Some(from)).ok();
            write_board(fx, fy, to)
        },
        _ => {
            write_board(tx, ty, Some(from))
        }
    } {
        Ok(_) => true,
        Err(_) => false
    };



    return if let PieceInteraction::PawnSkip = interaction {
        write_board(tx, ty, Some(Piece(PieceType::Pawn(true), from.1)))
    } else {
        write_board(tx, ty, Some(from))
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn(bool),
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl Into<char> for PieceType {
    fn into(self) -> char {

        match self {
            Pawn(_) => 'P',
            Rook => 'R',
            Knight => 'N',
            Bishop => 'B',
            Queen => 'Q',
            King => 'K'
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceInteraction {
    Empty,
    Enemy,
    PawnSkip,
    KingRookSwap,
    Ally,
    OutOfBounds
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece (pub PieceType, pub bool);

impl Piece {
    pub const fn new(piece: PieceType, is_white: bool) -> Self {
        Piece(piece, is_white)
    }

    // tests at a location (tx, ty) if the attacking piece is able to move there.
    // tests for opposite teams, and if the space is empty.
    fn test_at(&self, tx: isize, ty: isize) -> PieceInteraction {

        let space = read_board(tx, ty);


        use PieceInteraction::*;
        return match space {
            Ok(Some(p)) => {
                if p.1 == self.1 {
                    if self.0 == PieceType::Rook && p.0 == PieceType::King {
                        return KingRookSwap;
                    }
                    return Ally;
                }
                Enemy
            },
            Ok(None) => {
                Empty
            },
            Err(_) => {
                OutOfBounds
            }
        };
    }

    fn test_line(&self, x:isize, y:isize, direction: Direction, vector: &mut Vec<(isize, isize, PieceInteraction)>) {

        let mut index: isize = 1;

        while index < 9 {

            let (tx, ty) = direction.translate(x, y, index);
            use PieceInteraction::*;
            let interaction = self.test_at(tx, ty);
            match interaction {

                Enemy => {
                    vector.push((tx, ty, interaction));
                    break;
                },
                KingRookSwap => if index == 1 {
                    vector.push((tx, ty, interaction));
                    break;
                } else {
                    break;
                }
                Empty => {
                    vector.push((tx, ty, Empty));
                },
                PawnSkip => {
                    break;
                },
                Ally => {
                    break;
                },
                OutOfBounds => {
                    break;
                }
            }

            index = index + 1;
        }
    }

    pub fn move_list(&self, x: isize, y: isize) -> Vec<(isize, isize, PieceInteraction)> {
        let mut vector: Vec<(isize, isize, PieceInteraction)> = Vec::new();

        match self.0 {

            Pawn(is_skipped) => {

                // pushes the space in front of the pawn, if empty

                let m = match self.1 {
                    true => y-1,
                    false => y+1
                };

                if let Ok(space) = read_board(x, m) {
                    if space == None {
                        vector.push((x, m, PieceInteraction::Empty));
                    }
                }

                if !is_skipped {
                    if let Ok(space) = read_board(x, y + 2) {
                        if space == None {
                            vector.push((x, y + 2, PieceInteraction::PawnSkip));
                        }
                    }
                }

                //diagonal attacks

                if let Ok(Some(p)) = read_board(x-1, m) {
                    if p.1 != self.1 {
                        vector.push((x-1, m, PieceInteraction::Enemy));
                    }
                }

                if let Ok(Some(p)) = read_board(x+1, m) {
                    if p.1 != self.1 {
                        vector.push((x+1, m, PieceInteraction::Enemy));
                    }
                }
            },

            Rook => {

                for dir in Direction::CARDINALS {
                    self.test_line(x, y, dir, &mut vector);
                }
            }
            Knight => {
                let mut buf: Vec<(isize, isize)> = Vec::new();

                let (x, y): (isize, isize) = (x as isize, y as isize);

                for x_is_1 in [true, false] {
                    let (dx, dy) = if x_is_1 {
                        (1, 2)
                    } else {
                        (2, 1)
                    };

                    for is_x_neg in [true, false] {
                        let (dx, dy) = if is_x_neg {
                            (dx * -1, dy)
                        } else {
                            (dx, dy)
                        };
                        for is_y_neg in [true, false] {

                            if is_y_neg {
                                buf.push((x + dx, y + (dy * -1)));
                            } else {
                                buf.push((x + dx, y + dy));
                            }
                        }
                    }
                }

                for (tx, ty) in buf {

                    let interaction = self.test_at(tx, ty);

                    if interaction == PieceInteraction::Ally || interaction == PieceInteraction::OutOfBounds {
                        continue;
                    }
                    vector.push((tx, ty, interaction));
                }
            },
            Bishop => {

                for dir in Direction::ORDINALS {
                    self.test_line(x, y, dir, &mut vector);
                }

            },
            Queen => {
                for dir in Direction::CARDINALS {
                    self.test_line(x, y, dir, &mut vector);
                }
                for dir in Direction::ORDINALS {
                    self.test_line(x, y, dir, &mut vector);
                }
            },
            King => {

                for dir in Direction::CARDINALS {

                    let (dx, dy) = dir.translate(x, y, 1);

                    if let Some(interaction) = 'testat: {

                        let space = read_board(dx, dy);

                        if let Ok(Some(p)) = space {
                            if p.1 != self.1 {
                                break 'testat Some(PieceInteraction::Enemy);
                            } else if p.1 == self.1 && p.0 == PieceType::Rook {
                                break 'testat Some(PieceInteraction::KingRookSwap);
                            }
                        } else if let Ok(None) = space {
                            break 'testat Some(PieceInteraction::Empty);
                        }
                        None
                    } {
                        vector.push((dx, dy, interaction));
                    }
                }
            }
        }
        vector
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
 enum Direction {
     North,
     South,
     East,
     West,
     NW,
     NE,
     SW,
     SE
}

use Direction::*;

impl Direction {

    const CARDINALS: [Direction; 4] = [North, South, East, West];
    const ORDINALS: [Direction; 4] = [NW, NE, SW, SE];

    pub fn translate(&self, x: isize, y: isize, d: isize) -> (isize, isize) {

        use Direction::*;
        let (x, y) = match self {
            North => (x, y + d),
            South => (x, y - d),
            East => (x + d, y),
            West => (x - d, y),
            NW => (x - d, y + d),
            NE => (x + d, y + d),
            SW => (x - d, y - d),
            SE => (x + d, y - d)
        };
        return (x, y);

    }
}
