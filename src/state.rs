

use Space::*;


pub fn get_board() -> &'static [[Space; 8]; 8] {
    unsafe {
        &BOARD
    }
}

static mut BOARD: [[Space; 8]; 8] =
    [[Rook(false), Knight(false), Bishop(false), King(false), Queen(false), Bishop(false), Knight(false), Rook(false)],
    [Pawn(false, false); 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Open; 8],
    [Rook(true), Knight(true), Bishop(true), King(true), Queen(true), Bishop(true), Knight(true), Rook(true)]
    ];

// gets the Piece at the given input space.
// rusty safe wrapper for direct static memory access.
// if the inputs are out of bounds, retuns None.
//
// coordinates are real board space, not zeroed.
pub fn read_board(x: isize, y: isize) -> Option<Space> {
    if x < 1 || x > 8 || y < 1 || y > 8 {
        return None;
    }

    unsafe {
        Some(BOARD[y as usize - 1][x as usize - 1])
    }
}

// directly writes to the board.
//
// if successful, returns true.
pub fn write_board(x: isize, y: isize, space: Space) -> bool {
    if x < 1 || x > 8 || y < 1 || y > 8 {
        return false;
    }

    unsafe {
        BOARD[y as usize - 1][x as usize - 1] = space;
        return true;
    }
}

// Tries to move a piece from [fx, fy] to [tx, ty]
// returns an error if unsuccsessful.
pub fn move_piece(fx: isize, fy: isize, tx: isize, ty: isize) -> bool {

    // reads board.
    let from = read_board(fx, fy);
    let to = read_board(tx, ty);

    // moving piece from out of bounds is invalid
    let from = match from {
        Some(f) => f,
        None => return false
    };

    // destination out of bounds is invalid
    let to = match to {
        Some(t) => t,
        None => return false
    };

    // moving piece from an empty space is invalid
    if let Open = from {
        return false;
    }

    use Movement::*;

    let mut movement = Blocked;

    for valid_move in from.move_list(fx, fy) {
        if valid_move.0 == tx && valid_move.1 == ty {
            movement = valid_move.2;
            break;
        }
    }


    // if the movement was a Pawn Skip, (which can be performed only once)
    // disables the pawn skip.
    //
    // performs RookKing swap as well.

    return match movement {
        PawnSkip => {
            write_board(fx, fy, Space::Open);
            write_board(tx, ty, Space::Pawn(from.is_white().unwrap(), true))
        },
        KingRookSwap => {
            write_board(tx, ty, from);
            write_board(fx, fy, to)
        },
        Enemy | Empty => {
            write_board(fx, fy, Space::Open);
            write_board(tx, ty, from)
        },
        Blocked => {
            false
        }

    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Empty,
    Enemy,
    PawnSkip,
    Castle,
    Check,
    Blocked
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Space {
    Pawn(bool, bool),
    Rook(bool),
    Knight(bool),
    Bishop(bool),
    Queen(bool),
    King(bool),
    Open
}

impl Into<char> for Space {
    fn into(self) -> char {

        match self {
            Pawn(_, _) => 'P',
            Rook(_) => 'R',
            Knight(_) => 'N',
            Bishop(_) => 'B',
            Queen(_) => 'Q',
            King(_) => 'K',
            Open => ' '
        }
    }
}

impl Space {

    pub fn is_white(&self) -> Option<bool> {
        match self {
            Pawn(w, _) => Some(*w),
            Rook(w) => Some(*w),
            Knight(w) => Some(*w),
            Bishop(w) => Some(*w),
            Queen(w) => Some(*w),
            King(w) => Some(*w),
            Open => None
        }
    }

    // tests at a location (tx, ty) if the attacking piece is able to move there.
    // tests for opposite teams, and if the space is empty.
    fn test_at(&self, tx: isize, ty: isize) -> Movement {

        if let Open = self {
            return Movement::Blocked;
        }

        use Movement::*;

        return match read_board(tx, ty) {
            Some(Open) => {
                Empty
            },
            Some(piece) => {
                if piece.is_white() != self.is_white() {
                    return Enemy;
                }
                Blocked
            },
            None => Blocked
        };
    }

    fn test_line(&self, x:isize, y:isize, direction: Direction, vector: &mut Vec<(isize, isize, Movement)>) {

        let mut index: isize = 1;

        while index < 9 {

            let (tx, ty) = direction.translate(x, y, index);
            use Movement::*;
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

    pub fn move_list(&self, x: isize, y: isize) -> Vec<(isize, isize, Movement)> {
        let mut vector: Vec<(isize, isize, Movement)> = Vec::new();

        use Movement::*;

        match self {

            Pawn(w, is_moved) => {

                let dir = match w {
                    true => -1,
                    false => 1
                };

                // pushes the space in front of the pawn, if empty
                if let Some(Open) = read_board(x, y + dir) {
                    if !is_moved {
                        if let Some(Open) = read_board(x, y + (dir * 2)) {
                         vector.push((x, y + (dir * 2), PawnSkip));
                        }
                    }
                    vector.push((x, y + dir, Empty));
                }

                //diagonal attacks
                if let Some(p) = read_board(x-1, y + dir) {
                    if p != Open && p.is_white().unwrap() != *w {
                        vector.push((x - 1, y + dir, Enemy));
                    }
                }

                if let Some(p) = read_board(x + 1, y + dir) {
                    if p != Open && p.is_white().unwrap() != *w {
                        vector.push((x + 1, y + dir, Enemy));
                    }
                }
            },

            Rook(_) => {

                for dir in Direction::CARDINALS {
                    self.test_line(x, y, dir, &mut vector);
                }
            }
            Knight(_) => {
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

                    if interaction == Blocked {
                        continue;
                    }
                    vector.push((tx, ty, interaction));
                }
            },
            Bishop(_) => {

                for dir in Direction::ORDINALS {
                    self.test_line(x, y, dir, &mut vector);
                }

            },
            Queen(_) => {
                for dir in Direction::CARDINALS {
                    self.test_line(x, y, dir, &mut vector);
                }
                for dir in Direction::ORDINALS {
                    self.test_line(x, y, dir, &mut vector);
                }
            },
            King(_) => {

                use Movement::*;
                for dir in Direction::CARDINALS {

                    let (dx, dy) = dir.translate(x, y, 1);

                    match read_board(dx, dy) {
                        Some(Open) => {
                            vector.push((dx, dy, Empty))
                        },
                        Some(piece) => {
                            if piece.is_white() != self.is_white() {
                                vector.push((dx, dy, Enemy))
                            }
                        },
                        None => {}
                    }
                }

                // Tests for castle move
                // <Insert code here>



            },
            Open => {}
        }
        vector
    }

    fn test_for_checks(king: (isize, isize)) -> Vec<(isize, isize)> {
        let vector = Vec::new();

        // copies board, so we aren't constantly making unsafe calls
        // better readability, but worse performance.
        // TODO
        let board = unsafe {
            BOARD.clone()
        };
        let (kx, ky) = king;
        let king = read_board(kx, ky);
        let king = if let Some(King(_)) = king {
            let Some(k) = king;
            k
        } else {
            return vector;
        };


        for ty in 0..8 {
            for tx in 0..8 {
                let piece = unsafe {BOARD[ty][tx]};



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
