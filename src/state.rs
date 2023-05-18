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
    [space_new(Pawn, false); 8],
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

pub fn move_piece(fx: usize, fy: usize, tx: usize, ty: usize) -> Result<(), &'static str> {


    let from = read_board(fx, fy)?;
    let to = read_board(tx, ty)?;

    if let None = from {
        return Err("\"From\" piece is Empty!");
    }

    let from = from.unwrap();

    let move_list = from.move_list(fx, fy);
    let mut is_valid = false;

    for valid_move in move_list {
        if valid_move.0 == fx && valid_move.1 == fy {
            is_valid = true;
            break;
        }
    };

    if !is_valid {
        return Err("No valid moves available");
    }

    if let Some(to) = to {
        if from.1 == to.1 {
            return Err("Colors of both pieces are the same!");
        }
    }
    write_board(fx, fy, None)?;
    write_board(tx, ty, Some(from))?;

    Ok(())
}




#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl Into<char> for PieceType {
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

pub enum PieceInteraction {
    Empty,
    Enemy
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece (pub PieceType, pub bool);

impl Piece {
    pub const fn new(piece: PieceType, is_white: bool) -> Self {
        Piece(piece, is_white)
    }

    fn test_line(x:usize, y:usize, dx: isize, dy: isize, is_white: bool, vector: &mut Vec<(usize, usize, PieceInteraction)>) {

        let mut index: isize = 1;

        loop {

            let (cx, cy): (usize, usize) =
                ((isize::try_from(x).unwrap() + (index * dx)).try_into().unwrap(),
                                            (isize::try_from(y).unwrap() + (index * dy)).try_into().unwrap());


            let space = read_board(cx, cy);

            if let Ok(Some(p)) = space {
                if p.1 != is_white {
                    vector.push((cx, cy, PieceInteraction::Enemy));
                    return;
                } else {
                    return;
                }
            } else if let Ok(None) = space {
                vector.push((cx, cy, PieceInteraction::Empty));
            } else {
                return;
            }
            index = index + 1;
        }
    }

    pub fn move_list(&self, x: usize, y: usize) -> Vec<(usize, usize, PieceInteraction)> {
        let mut vector: Vec<(usize, usize, PieceInteraction)> = Vec::new();

        match self.0 {

            Pawn => {

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
            Knight => {/*TODO*/},
            Bishop => {
                Piece::test_line(x, y, 1, 1, self.1, &mut vector);
                Piece::test_line(x, y, -1, 1, self.1, &mut vector);
                Piece::test_line(x, y, 1, -1, self.1, &mut vector);
                Piece::test_line(x, y, -1, -1, self.1, &mut vector);

            }

            _ => ()
        }
        vector
    }
}
