use Space::*;
use Direction::*;

pub type Loc = [isize; 2];
pub type Board = [[Space; 8];8];

pub trait AccessBoard {
    fn read_board(&self, loc: Loc) -> Option<Space>;
    fn move_piece(&mut self, fromc: Loc, toc: Loc) -> bool;
    fn move_list(&self, from: Loc) -> Vec<MoveData>;
}

trait BoardOps {
    fn write_board(&mut self, loc: Loc, space: Space) -> bool;
    fn test_line(&self, fromc: Loc, dir: Direction, vector: &mut Vec<MoveData>);
    fn get_relation(&self, fromc: Loc, toc: Loc) -> Movement;
}

impl AccessBoard for Board {
    // gets the Piece at the given input space.
    // rusty safe wrapper for direct static memory access.
    // if the inputs are out of bounds, retuns None.
    //
    // coordinates are real board space, not zeroed.
    fn read_board(&self, r: Loc) -> Option<Space> {
        if r[0] < 1 || r[0] > 8 || r[1] < 1 || r[1] > 8 {
            return None;
        }

        Some(self[r[1] as usize - 1][r[0] as usize - 1])
    }

    // Move list returned is unfiltered.
    fn move_list(&self, fromc: Loc) -> Vec<MoveData> {
        let mut vector: Vec<MoveData> = Vec::new();
        use Movement::*;

        let from = self.read_board(fromc);
        let from = match from {
            Some(p) => p,
            None => {return vector;}
        };



        match from {

            Pawn(w, is_moved) => {let dir = match w {
                    true => Direction::South,
                    false => Direction::North
                };

                let infront = dir.translate(fromc, 1);

                // pushes the space in front of the pawn, if empty
                if let Some(Open) = self.read_board(infront) {
                    if !is_moved {
                        let pawnskip = dir.translate(fromc, 2);
                        if let Some(Open) = self.read_board(pawnskip) {
                            vector.push(MoveData::new(PawnSkip, pawnskip));
                        }
                    }
                    vector.push(MoveData::new(Empty, infront));
                }


                let attack_east = Direction::East.translate(infront, 1);
                let attack_west = Direction::West.translate(infront, 1);

                //diagonal attacks
                if let Some(p) = self.read_board(attack_east) {
                    if p != Open && p.is_white() != w {
                        vector.push(MoveData::new(Enemy, attack_east));
                    }
                }

                if let Some(p) = self.read_board(attack_west) {
                    if p != Open && p.is_white() != w {
                        vector.push(MoveData::new(Enemy, attack_west));
                    }
                }
            },

            Rook(_) => {

                for dir in Direction::CARDINALS {
                    self.test_line(fromc, dir, &mut vector);
                }
            }
            Knight(_) => {

                let knight_test: [Loc; 8] = [[1,2],[2,1],[-1,2],[-2,1],[1,-2],[2,-1],[-1,-2],[-2,-1]];

                for dloc in knight_test {

                    let testc = [fromc[0] + dloc[0], fromc[1] + dloc[1]];
                    let interaction = self.get_relation(fromc, testc);
                    if interaction == Blocked {
                        continue;
                    }
                    vector.push(MoveData::new(interaction, testc));
                }
            },
            Bishop(_) => {
                for dir in Direction::ORDINALS {
                    self.test_line(fromc, dir, &mut vector);
                }
            },
            Queen(_) => {
                for dir in Direction::CARDINALS {
                    self.test_line(fromc, dir, &mut vector);
                }
                for dir in Direction::ORDINALS {
                    self.test_line(fromc, dir, &mut vector);
                }
            },
            King(w) => {

                use Movement::*;
                for dir in Direction::CARDINALS {

                    let testc = dir.translate(fromc, 1);

                    match self.read_board(testc) {
                        Some(Open) => {
                            vector.push(MoveData::new(Empty, testc))
                        },
                        Some(space) => {
                            if space.is_white() != w {
                                vector.push(MoveData::new(Enemy, testc))
                            }
                        },
                        None => {}
                    }
                }

                // Tests for castle move
                // <Insert code here>

            },
            Open => {return vector;}
        }

        deep_checks(self, fromc, self.read_board(fromc).unwrap().is_white(), &mut vector);
        vector
    }

    fn move_piece(&mut self, fromc: Loc, toc: Loc) -> bool {
        use Movement::*;

        let movement = {
            let mut out = Blocked;

            for valid_move in self.move_list(fromc) {
                if valid_move.to == toc {
                    out = valid_move.interaction;
                    break;
                }
            }
            out
        };

        let from = self.read_board(fromc);
        let from = match from {
            Some(f) => f,
            None => return false
        };

        let to = self.read_board(toc);
        // destination out of bounds is invalid
        let to = match to {
            Some(t) => t,
            None => return false
        };

        // if the movement was a Pawn Skip, (which can be performed only once)
        // disables the pawn skip.
        //
        // performs RookKing swap as well.

        return match movement {
            PawnSkip => {
                self.write_board(fromc, Space::Open);
                self.write_board(toc, Space::Pawn(from.is_white(), true))
            },
            Castle => {
                self.write_board(toc, from);
                self.write_board(fromc, to)
            },
            Enemy | Empty => {
                self.write_board(fromc, Space::Open);
                self.write_board(toc, from)
            },
            Blocked | Check => {
                false
            }
        };
    }

}

impl BoardOps for Board {
    // directly writes to the board.
    //
    // if successful, returns true.
    fn write_board(&mut self, r: Loc, space: Space) -> bool {
        if r[0] < 1 || r[0] > 8 || r[1] < 1 || r[1] > 8 {
            return false;
        }

        self[r[1] as usize - 1][r[0] as usize - 1] = space;
        return true;
    }

    fn get_relation(&self, fromc: Loc, toc: Loc) -> Movement {
        use Movement::*;

        let from = self.read_board(fromc);
        let to = self.read_board(toc);

        let from = match from {
            Some(Open) | None => {return Blocked;},
            Some(space) => space,
        };

        return match to {
            Some(Open) => Empty,
            Some(piece) => {
                if from.is_white() != piece.is_white() {
                    Enemy
                } else {
                    Blocked
                }
            },
            None => Blocked
        }
    }

    fn test_line(&self, fromc: Loc, dir: Direction, vector: &mut Vec<MoveData>) {
        use Movement::*;

        for index in 1..9 {

            let toc = dir.translate(fromc, index);
            let relation = self.get_relation(fromc, toc);

            match relation {
                Empty | Check => {
                    vector.push(MoveData::new(relation, toc));
                },
                Enemy => {
                    vector.push(MoveData::new(relation, toc));
                    break;
                },
                PawnSkip | Blocked | Castle => {
                    break;
                }
            }
        }
    }
}

fn get_opposing_pieces(board: &Board, is_white: bool) -> Vec<Loc> {
    let mut vector = Vec::new();

    for iy in 1..9 {
        for ix in 1..9 {
            let space = board[iy][ix];
            if space == Open || space.is_white() == is_white {continue;}
            vector.push([ix as isize, iy as isize]);
        }
    }
    vector
}

fn get_check(board: &Board, is_white: bool) -> Vec<Loc> {
    let mut vector = Vec::new();

    for pc in get_opposing_pieces(board, is_white) {

        let moveset = board.move_list(pc);
        for movement in moveset {
            let test = board.read_board(movement.to);

            if let Some(King(_)) = test {
                vector.push(pc);
            }
        }
    }
    vector
}

fn is_check(board: &Board, is_white: bool) -> bool {
    for pc in get_opposing_pieces(board, is_white) {

        let moveset = board.move_list(pc);
        for movement in moveset {
            let test = board.read_board(movement.to);

            if let Some(King(_)) = test {
                return true;
            }
        }
    }
    return false;
}

fn is_move_check(board: &Board, fromc: Loc, toc: Loc) -> bool {
    let mut test_board = board.clone();
    if test_board.move_piece(fromc, toc) {

        return is_check(board, test_board.read_board(toc).unwrap().is_white());

    }
    return false;
}

fn deep_checks(board: &Board, fromc: Loc, is_white: bool, vector: &mut Vec<MoveData>) {

    for index in 0..vector.len() {

        let movement = vector[index];

        let mut test_board = board.clone();
        test_board.move_piece(fromc, movement.to);

        if is_check(&test_board, is_white) {
            vector[index] = MoveData::new(Movement::Check, movement.to);
        }
    }
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

    pub fn is_white(&self) -> bool {
        match self {
            Pawn(w, _) => *w,
            Rook(w) => *w,
            Knight(w) => *w,
            Bishop(w) => *w,
            Queen(w) => *w,
            King(w) => *w,
            Open => {panic!("tried to get the team of an empty space!")}
        }
    }
}

#[derive(Copy, Clone)]
pub struct MoveData {
    pub interaction: Movement,
    pub to: Loc,
}

impl MoveData {
    fn new(interaction: Movement, to: Loc) -> Self {
        Self {
            interaction,
            to,
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
pub enum Direction {
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
