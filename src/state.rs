

use Space::*;
use Direction::*;

pub fn get_board() -> &'static [[Space; 8]; 8] {
    unsafe {
        &BOARD
    }

}

pub type Loc = [isize; 2];
pub type Board = [[Space; 8];8];

pub trait AccessBoard {
    fn read_board(&self, x: isize, y: isize) -> Option<Space>;
    fn write_board(&self, x:isize, y: isize, space: Space) -> bool;
}

static mut BOARD: Board =
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
pub fn read_board(r: Loc) -> Option<Space> {
    if r[0] < 1 || r[0] > 8 || r[1] < 1 || r[1] > 8 {
        return None;
    }

    unsafe {
        Some(BOARD[r[1] as usize - 1][r[0] as usize - 1])
    }
}

// directly writes to the board.
//
// if successful, returns true.
pub fn write_board(r: Loc, space: Space) -> bool {
    if r[0] < 1 || r[0] > 8 || r[1] < 1 || r[1] > 8 {
        return false;
    }

    unsafe {
        BOARD[r[1] as usize - 1][r[0] as usize - 1] = space;
        return true;
    }
}

// Tries to move a piece from [fx, fy] to [tx, ty]
// returns an error if unsuccsessful.
pub fn move_piece(from_coords: Loc, to_coords: Loc) -> bool {

    // reads board.
    let from = read_board(from_coords);
    let to = read_board(to_coords);

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

    for valid_move in from.move_list(from_coords) {
        if valid_move.to[0] == to_coords[0] && valid_move.to[1] == to_coords[1] {
            movement = valid_move.interaction;
            break;
        }
    }


    // if the movement was a Pawn Skip, (which can be performed only once)
    // disables the pawn skip.
    //
    // performs RookKing swap as well.

    return match movement {
        PawnSkip => {
            write_board(from_coords, Space::Open);
            write_board(to_coords, Space::Pawn(from.is_white().unwrap(), true))
        },
        KingRookSwap => {
            write_board(to_coords, from);
            write_board(from_coords, to)
        },
        Enemy | Empty => {
            write_board(from_coords, Space::Open);
            write_board(to_coords, from)
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
    fn test_at(&self, testc: Loc) -> (Movement, Option<Space>) {

        if let Open = self {
            return (Movement::Blocked, None);
        }

        use Movement::*;

        return match read_board(testc) {
            Some(Open) => {
                (Empty, Some(Open))
            },
            Some(piece) => {
                if piece.is_white() != self.is_white() {
                    return (Enemy, Some(piece));
                }
                (Blocked, Some(piece))
            },
            None => (Blocked, None)
        };
    }

    fn test_line(&self, startc: Loc, direction: Direction, vector: &mut Vec<MoveData>) {

        let mut index: isize = 1;

        while index < 9 {

            let testc = direction.translate(startc, index);
            use Movement::*;
            let (interaction, piece) = self.test_at(testc);


            match interaction {

                Enemy | Empty | Check => {
                    vector.push(MoveData::new(interaction, startc, testc, self.clone(), piece.unwrap()));
                },
                Enemy => {
                    break;
                },
                PawnSkip | Blocked | Castle => {
                    break;
                },
            }

            index = index + 1;
        }
    }

    pub fn move_list(&self, piece: Loc) -> Vec<MoveData> {
        let mut vector: Vec<MoveData> = Vec::new();

        use Movement::*;

        match self {

            Pawn(w, is_moved) => {

                let dir = match w {
                    true => -1,
                    false => 1
                };
                let infront = [piece[0], piece[1] + dir];

                // pushes the space in front of the pawn, if empty
                if let Some(Open) = read_board(infront) {
                    if !is_moved {
                        let pawnskip = [piece[0], piece[1] + (dir * 2)];
                        if let Some(Open) = read_board(pawnskip) {
                            vector.push(MoveData::new(PawnSkip, piece, pawnskip, self.clone(), Open));
                        }
                    }
                    vector.push(MoveData::new(Empty, piece, infront, self.clone(), Open));
                }

                let attack_left = [piece[0] - 1, piece[1] + dir];
                let attack_right = [piece[0] + 1, piece[1] + dir];

                //diagonal attacks
                if let Some(p) = read_board(attack_left) {
                    if p != Open && p.is_white().unwrap() != *w {
                        vector.push(MoveData::new(Enemy, piece, attack_left, self.clone(), p));
                    }
                }

                if let Some(p) = read_board(attack_right) {
                    if p != Open && p.is_white().unwrap() != *w {
                        vector.push(MoveData::new(Enemy, piece, attack_right, self.clone(), p));
                    }
                }
            },

            Rook(_) => {

                for dir in Direction::CARDINALS {
                    self.test_line(piece, dir, &mut vector);
                }
            }
            Knight(_) => {


                let knight_test: [Loc; 8] = [[1,2],[2,1],[-1,2],[-2,1],[1,-2],[2,-1],[-1,-2],[-2,-1]];

                for testc in knight_test {

                    let (interaction, space) = self.test_at(testc);

                    if interaction == Blocked || space == None {
                        continue;
                    }
                    vector.push(MoveData::new(interaction, piece, testc, self.clone(), space.unwrap()));
                }
            },
            Bishop(_) => {

                for dir in Direction::ORDINALS {
                    self.test_line(piece, dir, &mut vector);
                }

            },
            Queen(_) => {
                for dir in Direction::CARDINALS {
                    self.test_line(piece, dir, &mut vector);
                }
                for dir in Direction::ORDINALS {
                    self.test_line(piece, dir, &mut vector);
                }
            },
            King(_) => {

                use Movement::*;
                for dir in Direction::CARDINALS {

                    let testc = dir.translate(piece, 1);

                    match read_board(testc) {
                        Some(Open) => {
                            vector.push(MoveData::new(Empty, piece, testc, self.clone(), Open))
                        },
                        Some(space) => {
                            if space.is_white() != self.is_white() {
                                vector.push(MoveData::new(Enemy, piece, testc, self.clone(), space))
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

    fn test_for_checks(kingc: Loc) -> Vec<(isize, isize)> {
        let vector = Vec::new();

        // copies board, so we aren't constantly making unsafe calls
        // better readability, but worse performance.
        // TODO
        let board = unsafe {
            BOARD.clone()
        };
        let king = read_board(kingc);

        let king = match king {
            Some(King(k)) => {
                King(k)
            },
            _ => {return vector;}
        };

        let king_vector = king.move_list(kingc);

        for ty in 0..8 {
            for tx in 0..8 {
                let space: Space = unsafe {BOARD[ty][tx]};
                if space.is_white() == king.is_white() {
                    continue;
                }
                let testc: Loc = [tx as isize, ty as isize];










            }
        }



        vector
    }
}


pub struct MoveData {
    pub interaction: Movement,
    pub from: Loc,
    pub to: Loc,
    pub attacker: Space,
    pub target: Space
}

impl MoveData {
    fn new(interaction: Movement, from: Loc, to: Loc, attacker: Space, target: Space) -> Self {
        Self {
            interaction,
            from,
            to,
            attacker,
            target
        }
    }
}


trait Add<T> {
    fn add(self, b: T) -> T ;
}

impl Add<Loc> for Loc {
    fn add(self, b: Loc) -> Loc {

        [self[0] + b[0], self[1] + b[1]]
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

impl Direction {

    const CARDINALS: [Direction; 4] = [North, South, East, West];
    const ORDINALS: [Direction; 4] = [NW, NE, SW, SE];

    pub fn translate(&self, loc: Loc, d: isize) -> Loc {

        use Direction::*;
        let out = match self {
            North => loc.add([0, d]),
            South => loc.add([0, -d]),
            East => loc.add([d, 0]),
            West => loc.add([-d, 0]),
            NW => loc.add([-d, d]),
            NE => loc.add([d,d]),
            SW => loc.add([-d, -d]),
            SE => loc.add([d, -d])
        };
        out
    }
}
