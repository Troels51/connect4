use std::{convert::TryInto, fmt, str::FromStr};

use serde::{Deserialize, Deserializer, de};
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Player(Player),
    EMPTY,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.positions.iter().rev() {
            write!(f, "\n");
            for column in row.iter() {
                write!(f, "| {} ", column);
            }
            write!(f, "|");
        }
        write!(f, "\n")
    }
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
#[derive(Debug, Copy, Clone)]
pub struct Board {
    // We count from bottom left.
    // [0][0] is bottom left. [0][7] is bottom right
    // [7][0] is top left, [7][7] is top right
    positions: [[State; COL_COUNT]; ROW_COUNT],
    height: [u32; COL_COUNT], //Height is the current height for a given column
    pub current_player: Player,
    pub nr_moves : u32,
}


impl<'de> Deserialize<'de> for Board {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    // Default implementation just delegates to `deserialize` impl.
    *place = Deserialize::deserialize(deserializer)?;
    Ok(())
}
}
impl FromStr for Board {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b : Board  = Board::new();
        for column in s.chars()  {
            let columnnr : u32 = column.to_digit(10).ok_or("Was not digit")?;
            b.play(columnnr - 1);  //Subtract 1 to zero index it
        }
        return Ok(b);

    }
}
pub struct BoardError;
impl From<BoardError> for String  {
    fn from(_: BoardError) -> Self {
        todo!()
    }
}
pub struct Position {
    row: u32,
    column: u32,
}


impl Board {
    pub fn new() -> Board {
        Board {
            positions: [[State::EMPTY; COL_COUNT]; ROW_COUNT],
            current_player : Player::BLACK,
            nr_moves : 0,
            height: [0; COL_COUNT],
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
    pub fn can_play(&self, column: u32) -> bool {
        self.height[column as usize] < ROW_COUNT as u32
    }
    pub fn negamax(&self) -> i32 {
        if self.nr_moves == COL_COUNT as u32 * ROW_COUNT as u32 {
            return 0;
        }
        for x in 0..COL_COUNT {
            if self.can_play(x as u32) && self.check_move_for_win(x as u32) {
                return ( (COL_COUNT as i32) * (ROW_COUNT as i32) + 1 - self.nr_moves as i32 ) / 2;
            }
        }
        let mut best_score :i32 = -( (COL_COUNT as i32) * (ROW_COUNT as i32) );
        for x in 0..COL_COUNT {
            if self.can_play(x as u32) {
                let mut b_copy  = self.clone();
                b_copy.play(x as u32);
                let score = -b_copy.negamax();
                if score > best_score {
                    best_score = score;
                }
            }
        }
        println!("current best_score = {}", best_score);
        return best_score;
    }
    // This a move for a win
    pub fn check_move_for_win(&self, column: u32) -> bool {
        let column = column as usize;
        if self.height[column] >= 3  
            && self.positions[column][self.height[column] as usize -1 ] == State::Player(self.current_player)
            && self.positions[column][self.height[column] as usize -2 ] == State::Player(self.current_player)
            && self.positions[column][self.height[column] as usize -3 ] == State::Player(self.current_player)
        {
            return true;
        }

        for dy in -1i32..1 {    // Iterate on horizontal (dy = 0) or two diagonal directions (dy = -1 or dy = 1).
            let mut nb = 0;                       // counter of the number of stones of current player surronding the played stone in tested direction.
            for dx in (-1i32..1).step_by(2) { // count continuous stones of current player on the left, then right of the played column.
                let mut x = (column as i32) + dx;
                let mut y = self.height[column] as i32 + dx * dy;
                nb = nb + 1;

                while x >= 0 
                        && x < COL_COUNT as i32 
                        && y >= 0 
                        && y < ROW_COUNT as i32 
                        && self.positions[x as usize][y as usize] == State::Player(self.current_player) {
                    x = x + dx;
                    y = y + dx * dy;
                }
                println!("nb: {}", nb);

                if nb >= 3 { return true }; // there is an aligment if at least 3 other stones of the current user 
            }
        }
        return false;
    }
    pub fn play(&mut self, column: u32) -> () {
        assert!(column < COL_COUNT as u32);
        assert!(self.height[column as usize] < ROW_COUNT as u32);
        let column = column as usize;
        self.positions[self.height[column] as usize][column as usize] = State::Player(self.current_player);
        self.height[column] = self.height[column] + 1;
        self.nr_moves = self.nr_moves + 1;
        self.current_player = !self.current_player;
    }
}
