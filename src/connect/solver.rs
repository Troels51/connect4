use std::str::FromStr;

use super::board;

pub struct Solver {
    node_count : u32,
    column_order : [u32; board::COL_COUNT],
}
impl Solver {
    pub fn new() -> Solver {
        Solver {
            node_count: 0,
            column_order: [3, 4, 2, 5, 1, 6, 0],

        }
    }
    pub fn negamax(&mut self, board : board::Board, alpha : i32, beta : i32) -> i32 {
        assert!(alpha < beta);
        self.node_count = self.node_count + 1;
        //println!("negamax: {}, {}", alpha, beta);
        let mut alpha = alpha;
        let mut beta = beta;

        if board.nr_moves == board::COL_COUNT as u32 * board::ROW_COUNT as u32 {
            return 0;
        }

        for x in 0..board::COL_COUNT {
            if board.can_play(x as u32) && board.check_move_for_win(x as u32) {
                return (board::COL_COUNT as i32 * board::ROW_COUNT as i32 + 1 - board.nr_moves as i32) / 2;
            }
        }
        let max : i32 = i32::MAX;//(board::COL_COUNT as i32 * board::ROW_COUNT as i32 - 1 - board.nr_moves as i32) / 2;
        if beta > max {
            beta = max;
            if alpha >= beta { 
                return beta
            };
        }


        for x in 0..board::COL_COUNT {
            if board.can_play(self.column_order[x]) {
                let mut b_copy = board.clone();
                b_copy.play(self.column_order[x]);
                let score = - self.negamax(b_copy, -beta, -alpha);
                println!("Score: {}, alpha: {}, beta: {}", score, alpha, beta);
                if score >= beta { return score; }
                if score > alpha { alpha = score; }
            }
        }
        return alpha;
    }
    pub fn solve(&mut self, board : board::Board) -> i32 {
        self.negamax(board, -1, 1)
    }
}
#[test]
fn easy_negamax_test() {
    let b : board::Board = board::Board::from_str("2252576253462244111563365343671351441").unwrap();
    let mut solver : Solver = Solver::new();
    let score = solver.negamax(b, -1, 1);
    println!("node count: {}", solver.node_count);
    assert_eq!(score, -1);
}

#[test]
fn easy_negamax_test_2() {
    let b : board::Board = board::Board::from_str("7422341735647741166133573473242566").unwrap();
    let mut solver : Solver = Solver::new();
    let score = solver.negamax(b, -1, 1);
    println!("node count: {}", solver.node_count);
    assert_eq!(score, 1);
}

#[test]
fn easy_negamax_test_3() {
    let b : board::Board = board::Board::from_str("67152117737262713366376314254").unwrap();
    let mut solver : Solver = Solver::new();
    let score = solver.solve(b);
    println!("node count: {}", solver.node_count);
    assert_eq!(score, 6, "This board {}, should score 6, but scored {}", b, score);
}

 
