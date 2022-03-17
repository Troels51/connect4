use std::{fmt, str::FromStr, hash::Hasher};
use serde::{de, Deserialize, Deserializer};

use core::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Player(Player),
    EMPTY,
}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;

        write!(f, "| {} | {} | {} | {} | {} | {} | {} | \n", self.pos_to_state(0, 5), self.pos_to_state(1, 5), self.pos_to_state(2, 5), self.pos_to_state(3, 5), self.pos_to_state(4, 5), self.pos_to_state(5, 5), self.pos_to_state(6, 5))?;
        write!(f, "| {} | {} | {} | {} | {} | {} | {} | \n", self.pos_to_state(0, 4), self.pos_to_state(1, 4), self.pos_to_state(2, 4), self.pos_to_state(3, 4), self.pos_to_state(4, 4), self.pos_to_state(5, 4), self.pos_to_state(6, 4))?;
        write!(f, "| {} | {} | {} | {} | {} | {} | {} | \n", self.pos_to_state(0, 3), self.pos_to_state(1, 3), self.pos_to_state(2, 3), self.pos_to_state(3, 3), self.pos_to_state(4, 3), self.pos_to_state(5, 3), self.pos_to_state(6, 3))?;
        write!(f, "| {} | {} | {} | {} | {} | {} | {} | \n", self.pos_to_state(0, 2), self.pos_to_state(1, 2), self.pos_to_state(2, 2), self.pos_to_state(3, 2), self.pos_to_state(4, 2), self.pos_to_state(5, 2), self.pos_to_state(6, 2))?;
        write!(f, "| {} | {} | {} | {} | {} | {} | {} | \n", self.pos_to_state(0, 1), self.pos_to_state(1, 1), self.pos_to_state(2, 1), self.pos_to_state(3, 1), self.pos_to_state(4, 1), self.pos_to_state(5, 1), self.pos_to_state(6, 1))?;
        write!(f, "| {} | {} | {} | {} | {} | {} | {} | \n", self.pos_to_state(0, 0), self.pos_to_state(1, 0), self.pos_to_state(2, 0), self.pos_to_state(3, 0), self.pos_to_state(4, 0), self.pos_to_state(5, 0), self.pos_to_state(6, 0))?;
       
        write!(f, "\n")
    }
}
//Naming :(
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

pub const MIN_SCORE: i32 = -(ROW_COUNT as i32 * COL_COUNT as i32) / 2 + 3;
pub const MAX_SCORE: i32 = (ROW_COUNT as i32 * COL_COUNT as i32 +1) / 2 - 3;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board {
    // We count from bottom left.
    // [0][0] is bottom left. [0][7] is bottom right
    // [7][0] is top left, [7][7] is top right
    pub current_position: u64, //position is stored as a board, this is the stones for the current player
    pub mask: u64, //Mask for the non-empty positions

    pub current_player: Player,
    pub nr_moves: u32,
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.current_position + self.mask).hash(state);
    }
}

impl<'de> Deserialize<'de> for Board {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
        let mut b: Board = Board::new();
        for column in s.chars() {
            let columnnr: u32 = column.to_digit(10).ok_or("Was not digit")?;
            b.play(columnnr - 1); //Subtract 1 to zero index it
        }
        return Ok(b);
    }
}
pub struct BoardError;
impl From<BoardError> for String {
    fn from(_: BoardError) -> Self {
        todo!()
    }
}

impl Board {
    pub fn top_mask(column : u32) -> u64 {
        (1u64 << (ROW_COUNT - 1)) << (column as u64 * (ROW_COUNT as u64 + 1))
    }
    pub fn bottom_mask(column : u32) -> u64 {
        1u64 << (column as u64 * (ROW_COUNT as u64 + 1))
    }
    pub fn column_mask(column : u32) -> u64 {
        ((1 << ROW_COUNT) - 1) << (column as u64 * (ROW_COUNT as u64 + 1))
    }
    pub fn alignment(position : u64) -> bool{
        //horizontal
        let m  = position & (position >> (ROW_COUNT+1));
        if (m & (m >> (2*(ROW_COUNT+1)))) >= 1 {
            return true;
        }
        // diagonal 1
        let m_2 = position & (position >> ROW_COUNT);
        if (m_2 & (m_2 >> (2*ROW_COUNT))) >= 1 {
            return true;
        }

        // diagonal 2 
        let m_3 = position & (position >> (ROW_COUNT+2));
        if (m_3 & (m_3 >> (2*(ROW_COUNT+2)))) >= 1 {
             return true;
        }
        // vertical
        let m_4 = position & (position >> 1);
        if(m_4 & (m_4 >> 2)) >= 1 {
            return true;
        } 

        return false;
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            current_position: 0,
            mask: 0,
            current_player: Player::BLACK,
            nr_moves: 0,
        }
    }

    pub fn can_play(&self, column: u32) -> bool {
        (self.mask & Board::top_mask(column)) == 0
    }

    pub fn check_move_for_win(&self, column: u32) -> bool {
        let pos = self.current_position | 
                    ((self.mask + Board::bottom_mask(column)) & Board::column_mask(column));
        Board::alignment(pos)
    }
    pub fn play(&mut self, column: u32) -> () {
        assert!(column < COL_COUNT as u32);
        self.current_position ^= self.mask;
        self.mask |= self.mask + Board::bottom_mask(column);
        self.nr_moves = self.nr_moves + 1;
        self.current_player = !self.current_player;
    }
    fn pos_to_state(&self, column: u32, row: u32) -> State
    {
        if self.current_position & (1 << ((row) + (column*COL_COUNT as u32))) >= 1 {
            return State::Player(Player::RED)
        }
        else if self.mask & (1 << ((row) + (column*COL_COUNT as u32))) >= 1 {
            return State::Player(Player::BLACK)
        }
        State::EMPTY
    }
}

#[test]
fn check_win_test() {
    let b : Board = Board::from_str("121212").unwrap();
    assert!(b.check_move_for_win(0));
    assert!(!b.check_move_for_win(1));
    assert!(!b.check_move_for_win(2));
}


#[test]
fn can_play_test() {
    let mut b : Board = Board::from_str("11111222223333344444555556666677777").unwrap();
    for i in 0..COL_COUNT {
        assert!(b.can_play(i as u32));
        b.play(i as u32);
        assert!(!b.can_play(i as u32), "Should not be able to play column {}", i);
    }
}
#[test]
fn top_mask_test() {
    let m = Board::top_mask(6);
    assert_eq!(m, 0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000);
                                            //_11111000_00000000_00000000_00000000_00000000
    let m = Board::top_mask(5);
    assert_eq!(m, 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000);
    let m = Board::top_mask(4);
    assert_eq!(m, 0b00000000_00000000_00000000_00000010_00000000_00000000_00000000_00000000);
    let m = Board::top_mask(0);
    assert_eq!(m, 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100000);
}

#[test]
fn test_real() {
    let mut b : Board = Board::from_str("2252576253462244111563365343671351441").unwrap();
    assert!(!b.can_play(0));
    assert!(!b.can_play(1));
    assert!(!b.can_play(2));
    assert!(!b.can_play(3));
    assert!(!b.can_play(4));
    assert!(b.can_play(5)); //Can play last 2
    assert!(b.can_play(6));

    //No winning moves
    assert!(!b.check_move_for_win(0));
    assert!(!b.check_move_for_win(1));
    assert!(!b.check_move_for_win(2));
    assert!(!b.check_move_for_win(3));
    assert!(!b.check_move_for_win(4));
    assert!(!b.check_move_for_win(5));
    assert!(!b.check_move_for_win(6));

    b.play(6);

    assert!(!b.check_move_for_win(0));
    assert!(!b.check_move_for_win(1));
    assert!(!b.check_move_for_win(2));
    assert!(!b.check_move_for_win(3));
    assert!(!b.check_move_for_win(4));
    assert!(b.check_move_for_win(5));
    assert!(b.check_move_for_win(6));
}
#[test]
fn real_test_2() {
    let mut b : Board = Board::from_str("67152117737262713366376314254").unwrap();
    assert!(b.can_play(0));
    assert!(b.can_play(1));
    assert!(b.can_play(2));
    assert!(b.can_play(3));
    assert!(b.can_play(4));
    assert!(b.can_play(5));
    assert!(!b.can_play(6));

    //No winning moves
    assert!(!b.check_move_for_win(0));
    assert!(!b.check_move_for_win(1));
    assert!(!b.check_move_for_win(2));
    assert!(!b.check_move_for_win(3));
    assert!(!b.check_move_for_win(4));
    assert!(!b.check_move_for_win(5));
    assert!(!b.check_move_for_win(6));


    // b.play(6);

    // assert!(!b.check_move_for_win(0));
    // assert!(!b.check_move_for_win(1));
    // assert!(!b.check_move_for_win(2));
    // assert!(!b.check_move_for_win(3));
    // assert!(!b.check_move_for_win(4));
    // assert!(b.check_move_for_win(5));
    // assert!(b.check_move_for_win(6));
}


// Player one
#[test]
fn horizontal_win_test() {
    let b : Board = Board::from_str("1213142").unwrap();
    assert!(b.check_move_for_win(4));
    assert!(!b.check_move_for_win(5));
}

#[test]
fn vertical_win_test() {
    let b : Board = Board::from_str("121212").unwrap();
    assert!(b.check_move_for_win(0));
    assert!(!b.check_move_for_win(1));
}

#[test]
fn diagonal1_win_test() {
    let b : Board = Board::from_str("213233444").unwrap();
    assert!(b.check_move_for_win(3));
    assert!(!b.check_move_for_win(5));
}

#[test]
fn diagonal2_win_test() {
    let b : Board = Board::from_str("675655444").unwrap();
    assert!(b.check_move_for_win(3));
    assert!(!b.check_move_for_win(5));
}
//Player 2
#[test]
fn p2_horizontal_win_test() {
    let b : Board = Board::from_str("61213142").unwrap();
    assert!(b.check_move_for_win(4));
    assert!(!b.check_move_for_win(5));
}

#[test]
fn p2_vertical_win_test() {
    let b : Board = Board::from_str("6121212").unwrap();
    assert!(b.check_move_for_win(0));
    assert!(!b.check_move_for_win(1));
}

#[test]
fn p2_diagonal1_win_test() {
    let b : Board = Board::from_str("6213233444").unwrap();
    assert!(b.check_move_for_win(3));
    assert!(!b.check_move_for_win(5));
}

#[test]
fn p2_diagonal2_win_test() {
    let b : Board = Board::from_str("1675655444").unwrap();
    assert!(b.check_move_for_win(3));
    assert!(!b.check_move_for_win(5));
}