use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    RED,
    BLACK,
}
impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::BLACK => Player::RED,
            Player::RED => Player::BLACK,
        }
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::BLACK => write!(f, "BLACK (O)"),
            Player::RED => write!(f, "RED (X)"),
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Player(Player),
    EMPTY,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Player(Player::RED) => write!(f, "X"),
            State::Player(Player::BLACK) => write!(f, "O"),
            State::EMPTY => write!(f, " "),
        }
    }
}
pub const ROW_COUNT: usize = 6;
pub const COL_COUNT: usize = 7;
pub struct Board {
    // We count from bottom left.
    // [0][0] is bottom left. [0][7] is bottom right
    // [7][0] is top left, [7][7] is top right
    positions: [[State; ROW_COUNT]; COL_COUNT],
}
pub struct BoardError;
pub struct Position {
    row: u32,
    column: u32,
}
impl Board {
    pub fn new() -> Board {
        Board {
            positions: [[State::EMPTY; ROW_COUNT]; COL_COUNT],
        }
    }
    pub fn print(&self) {
        for row in self.positions.iter().rev() {
            println!();
            for column in row.iter() {
                print!("| {} ", column);
            }
            print!("|");
        }
        println!();
    }
    // This checks a board for a win, this could need optimization
    pub fn checkwin(&self, latestToken: Position, player: Player) -> bool {
        let mut winSeq = 0;
        // Horizontal check
        for c in 0..(COL_COUNT - 3) {
            for r in 0..ROW_COUNT {
                for i in 0..4 {
                    if self.positions[r][c + i] == State::Player(player) {
                        winSeq = winSeq + 1;
                    }
                    if winSeq == 4 {
                        return true;
                    }
                }
                winSeq = 0;
            }
        }
        // Vertical check
        for c in 0..COL_COUNT {
            for r in 0..(ROW_COUNT - 3) {
                for i in 0..4 {
                    if self.positions[r + i][c] == State::Player(player) {
                        winSeq = winSeq + 1;
                    }
                    if winSeq == 4 {
                        return true;
                    }
                }
                winSeq = 0;
            }
        }
        //Diagonal checks
        for c in 0..(COL_COUNT - 3) {
            for r in 3..ROW_COUNT {
                for i in 0..4 {
                    if self.positions[r - i][c + i] == State::Player(player) {
                        winSeq = winSeq + 1;
                    }
                    if winSeq == 4 {
                        return true;
                    }
                }
                winSeq = 0;
            }
        }
        for c in 0..(COL_COUNT - 3) {
            for r in 0..(ROW_COUNT - 3) {
                for i in 0..4 {
                    if self.positions[r + i][c + i] == State::Player(player) {
                        winSeq = winSeq + 1;
                    }
                    if winSeq == 4 {
                        return true;
                    }
                }
                winSeq = 0;
            }
        }
        return false;
    }
    pub fn insert(&mut self, player: Player, column: u32) -> Result<bool, BoardError> {
        assert!(column < 8);
        for row in 0..ROW_COUNT {
            if self.positions[row][column as usize] == State::EMPTY {
                self.positions[row][column as usize] = State::Player(player);
                return Ok(self.checkwin(
                    Position {
                        row: row as u32,
                        column: column,
                    },
                    player,
                ));
            }
        }
        return Err(BoardError);
    }
}
