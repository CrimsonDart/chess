use Space::*;
use Direction::*;
use super::board::Loc;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Space {
    Pawn(bool, PawnState),
    Rook(bool, bool),
    Knight(bool),
    Bishop(bool),
    Queen(bool),
    King(bool, bool),
    Open
}

impl Space {

    pub fn is_white(&self) -> bool {
        match self {
            Pawn(w, _) => *w,
            Rook(w, _) => *w,
            Knight(w) => *w,
            Bishop(w) => *w,
            Queen(w) => *w,
            King(w, _) => *w,
            Open => {panic!("tried to get the team of an empty space!")}
        }
    }
}

impl From<Space> for char {
    fn from(value: Space) -> Self {
        match value {
            Pawn(_, _) => 'P',
            Rook(_, _) => 'R',
            Knight(_) => 'N',
            Bishop(_) => 'B',
            Queen(_) => 'Q',
            King(_, _) => 'K',
            Open => ' '
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PawnState {
    NotMoved,
    PrevSkipped,
    Moved
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Empty,
    Enemy,
    PawnSkip,
    KingSide,
    QueenSide,
    Check,
    Blocked,
    EnPessant
}

#[derive(Copy, Clone)]
pub struct MoveData {
    pub relation: Movement,
    pub to: Loc,
}

trait Add {
    fn add(self, b: Self) -> Self ;
}

impl Add for Loc {
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

    pub const CARDINALS: [Direction; 4] = [North, South, East, West];
    pub const ORDINALS: [Direction; 4] = [NW, NE, SW, SE];

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
