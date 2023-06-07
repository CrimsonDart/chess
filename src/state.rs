


    // gets the Piece at the given input space.
    // rusty safe wrapper for direct static memory access.
    // if the inputs are out of bounds, retuns None.
    //
    // coordinates are real board space, not zeroed.


    fn move_piece(&mut self, fromc: Loc, toc: Loc) -> bool {

        let from = self.read_board(fromc);
        let is_white = match from {
            None | Some(Open) => {
                return false;
            },
            Some(p) =>
                p.is_white()
        };

        let mut out = None;
        let mut moveset = self.move_list(fromc);
        deep_checks(self, fromc, is_white, &mut moveset);


        for valid_move in moveset {

            if valid_move.to == toc {
                out = Some(valid_move);
                break;
            }
        }

        let movement = match out {

            None => {
                return false;
            },
            Some(m) => m
        };

        return self.do_move(fromc, movement);
    }

impl BoardOps for Board {

    fn do_move(&mut self, fromc: Loc, movement: MoveData) -> bool {
        use Movement::*;


        let toc = movement.to;

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

        return match movement.interaction {
            PawnSkip => {
                self.write_board(fromc, Space::Open);
                self.write_board(toc, Space::Pawn(from.is_white(), PawnState::PrevSkipped))
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
