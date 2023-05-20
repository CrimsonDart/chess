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
pub fn read_board(x: usize, y: usize) -> Result<Space, &'static str> {

    if x < 1 || x > 8 || y < 1 || y > 8 {
        return Err("Index out of Bounds!");
    }

    unsafe {
        Ok(BOARD[y - 1][x - 1])
    }
}

pub fn write_board(x: usize, y: usize, space: Space) -> Result<(), &'static str> {

    if x < 1 || x > 8 || y < 1 || y > 8 {
        return Err("Index out of Bounds!");
    }

    unsafe {
        BOARD[y - 1][x -1] = space;
        Ok(())
    }
}

// Tries to move a piece from [fx, fy] to [tx, ty]
// returns an error if unsuccsessful.
pub fn move_piece(fx: usize, fy: usize, tx: usize, ty: usize) -> Result<(), &'static str> {

    let from = read_board(fx, fy)?;
    let to = read_board(tx, ty)?;

    if let None = from {
        return Err("\"From\" piece is Empty!");
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
        return Err("No valid moves available");
    }

    if let Some(to) = to {
        if from.1 == to.1 {
            return Err("Colors of both pieces are the same!");
        }
    }

    write_board(fx, fy, None)?;

    // if the movement was a Pawn Skip, (which can be performed only once)
    // disables the pawn skip.
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
    KingRookSwap
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece (pub PieceType, pub bool);

impl Piece {
    pub const fn new(piece: PieceType, is_white: bool) -> Self {
        Piece(piece, is_white)
    }

    // tests at a location (x, y) if the attacking piece is able to move there.
    // tests for opposite teams, and if the space is empty.
    fn test_at(x:usize, y:usize, is_white: bool) -> Option<PieceInteraction> {

        let space = read_board(x, y);

        if let Ok(Some(p)) = space {
            if p.1 != is_white {
                return Some(PieceInteraction::Enemy);
            }
        } else if let Ok(None) = space {
            return Some(PieceInteraction::Empty);
        }
        return None;
    }

    fn test_line(x:usize, y:usize, dx: isize, dy: isize, is_white: bool, vector: &mut Vec<(usize, usize, PieceInteraction)>) {

        let mut index: isize = 1;

        loop {

            let (cx, cy): (usize, usize) =
                ((isize::try_from(x).unwrap() + (index * dx)).try_into().unwrap(),
                                            (isize::try_from(y).unwrap() + (index * dy)).try_into().unwrap());

            if let Some(interaction) = Self::test_at(cx, cy, is_white) {
                vector.push((cx, cy, interaction));
                if let PieceInteraction::Enemy = interaction {
                    break;
                }
            } else {
                break;
            }
            index = index + 1;
        }
    }

    pub fn move_list(&self, x: usize, y: usize) -> Vec<(usize, usize, PieceInteraction)> {
        let mut vector: Vec<(usize, usize, PieceInteraction)> = Vec::new();

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
                Piece::test_line(x, y, 0, 1, self.1, &mut vector);
                Piece::test_line(x, y, 0, -1, self.1, &mut vector);
                Piece::test_line(x, y, -1, 0, self.1, &mut vector);
                Piece::test_line(x, y, 1, 0, self.1, &mut vector);
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

                for pair in buf {
                    let (dx, dy) = (pair.0 as usize, pair.1 as usize);
                    if let Some(interaction) = Self::test_at(dx, dy, self.1) {
                        vector.push((dx, dy, interaction));
                    }
                }
            },
            Bishop => {
                Piece::test_line(x, y, 1, 1, self.1, &mut vector);
                Piece::test_line(x, y, -1, 1, self.1, &mut vector);
                Piece::test_line(x, y, 1, -1, self.1, &mut vector);
                Piece::test_line(x, y, -1, -1, self.1, &mut vector);
            },
            Queen => {
                Piece::test_line(x, y, 1, 1, self.1, &mut vector);
                Piece::test_line(x, y, -1, 1, self.1, &mut vector);
                Piece::test_line(x, y, 1, -1, self.1, &mut vector);
                Piece::test_line(x, y, -1, -1, self.1, &mut vector);
                Piece::test_line(x, y, 0, 1, self.1, &mut vector);
                Piece::test_line(x, y, 0, -1, self.1, &mut vector);
                Piece::test_line(x, y, -1, 0, self.1, &mut vector);
                Piece::test_line(x, y, 1, 0, self.1, &mut vector);
            },
            King => {
                let (x, y): (isize, isize) = (x as isize, y as isize);
                const arr: [isize; 2] = [-1, 1];
                for dx in arr{
                    for dy in arr {
                        if let Some(_) = Self::test_at((x + dx) as usize, (y + dy) as usize, self.1) {
                            vector.push(((x + dx) as usize, (y + dy) as usize, PieceInteraction::Empty));
                        }
                    }
                }

            }
        }
        vector
    }
}
