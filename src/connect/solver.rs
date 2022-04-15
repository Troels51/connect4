use std::collections::{HashMap, binary_heap::{self, BinaryHeap}};

use priority_queue::PriorityQueue;

use super::board;
pub struct Solver {
    node_count : u32,
    column_order : [u32; board::COL_COUNT],
    transposition_table: HashMap<board::Board,i32>,

}
impl Solver {
    pub fn new() -> Solver {
        Solver {
            node_count: 0,
            column_order: [3, 4, 2, 5, 1, 6, 0],
            transposition_table: HashMap::new(),
        }
    }
    pub fn negamax(&mut self, board : board::Board, alpha : i32, beta : i32) -> i32 {
        assert!(alpha < beta);
        assert!(!board.can_win_next());
        self.node_count = self.node_count + 1;
        let mut alpha = alpha;
        let mut beta = beta;
        

        let possible = board.possible_non_loosing_moves();
        if possible == 0 {
            return -(board::COL_COUNT as i32 * board::ROW_COUNT as i32 - board.nr_moves as i32) / 2;
        }
        // Check for drawn game
        if board.nr_moves >= board::COL_COUNT as u32 * board::ROW_COUNT as u32 - 2 {
            return 0;
        }
        let min = -(board::COL_COUNT as i32 * board::ROW_COUNT as i32 - 2 - board.nr_moves as i32) / 2;	// lower bound of score as opponent cannot win next move
        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }
        let mut max : i32 = ((board::COL_COUNT as i32 * board::ROW_COUNT as i32) - 1 - board.nr_moves as i32) / 2;
        if let Some(val) = self.transposition_table.get(&board) {
            max = val + board::MIN_SCORE - 1;
        }
        
        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta
            };
        }
        let mut pqueue = PriorityQueue::new();
        for i in self.column_order {
            let position =  possible & board::Board::column_mask(i.into());
            if position != 0 {
                let move_score = board.move_score(position);
                pqueue.push(position, move_score);
            }

        }
        for next_move in pqueue.into_sorted_iter() {
            let mut b_copy = board.clone();
            b_copy.play_bitmove(next_move.0);
            let score = -self.negamax(b_copy, -beta, -alpha);
            if score >= beta {
                return score;
            }
            if score > alpha {alpha = score;}
        }
        self.transposition_table.insert(board, alpha - board::MIN_SCORE + 1);
        return alpha;
    }

    pub fn solve(&mut self, board : board::Board) -> i32 {
        if board.can_win_next() {
            return (board::COL_COUNT as i32 * board::ROW_COUNT as i32 + 1 - board.nr_moves as i32) / 2;
        }
        let mut min: i32 = -(board::COL_COUNT as i32 * board::ROW_COUNT as i32 - board.nr_moves as i32)/2;
        let mut max: i32 = (board::COL_COUNT as i32 * board::ROW_COUNT as i32 + 1 - board.nr_moves as i32)/2;
        while min < max {
            let mut med = min + (max - min) / 2;
            if med <= 0 && min / 2 < med {
                med = min / 2;
            }
            else if med >= 0 && max / 2 > med {
                med = max / 2;
            }
            let r = self.negamax(board, med, med + 1);
            if r <= med {
                max = r;
            }
            else {
                min = r;
            }
        }
        min
    }
}
#[test]
fn easy_negamax_test() {
    use std::str::FromStr;
    let b : board::Board = board::Board::from_str("2252576253462244111563365343671351441").unwrap();
    let mut solver : Solver = Solver::new();
    let score = solver.negamax(b, -1, 1);
    println!("Score: {} ", score);
    println!("node count: {}", solver.node_count);
    assert_eq!(score, -1);
}

#[test]
fn easy_negamax_test_2() {
    use std::str::FromStr;
    let b : board::Board = board::Board::from_str("7422341735647741166133573473242566").unwrap();
    let mut solver : Solver = Solver::new();
    let score = solver.negamax(b, -1, 1);
    println!("Score: {}, with board {}, with current player: {}", score, b, b.current_player);
    println!("node count: {}", solver.node_count);
    assert_eq!(score, 1);
}

#[test]
fn easy_negamax_test_3() {
    use std::str::FromStr;
    let b : board::Board = board::Board::from_str("67152117737262713366376314254").unwrap();
    let mut solver : Solver = Solver::new();
    let score = solver.solve(b);
    println!("Score: {}", score);
    println!("node count: {}", solver.node_count);
    assert_eq!(score, 6, "This board {}, should score 6, but scored {}", b, score);
}

 
#[test]
fn middle_negamax_test_2() {
    use std::str::FromStr;
    let b : board::Board = board::Board::from_str("37313333717124171162542").unwrap();
    let mut solver : Solver = Solver::new();
    let score = solver.solve(b);
    println!("Score: {} ", score);
    println!("node count: {}", solver.node_count);
    assert_eq!(score, 3, "This board {}, should score 3, but scored {}", b, score);
}
