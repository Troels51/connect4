let mut win_seq = 0;
// Horizontal check
for c in 0..(COL_COUNT - 3) {
    for r in 0..ROW_COUNT {
        for i in 0..4 {
            if self.positions[r][c + i] == State::Player(self.current_player) {
                win_seq = win_seq + 1;
            }
            if win_seq == 4 {
                return true;
            }
        }
        win_seq = 0;
    }
}
// Vertical check
for c in 0..COL_COUNT {
    for r in 0..(ROW_COUNT - 3) {
        for i in 0..4 {
            if self.positions[r + i][c] == State::Player(self.current_player) {
                win_seq = win_seq + 1;
            }
            if win_seq == 4 {
                return true;
            }
        }
        win_seq = 0;
    }
}
//Diagonal checks
for c in 0..(COL_COUNT - 3) {
    for r in 3..ROW_COUNT {
        for i in 0..4 {
            if self.positions[r - i][c + i] == State::Player(self.current_player) {
                win_seq = win_seq + 1;
            }
            if win_seq == 4 {
                return true;
            }
        }
        win_seq = 0;
    }
}
for c in 0..(COL_COUNT - 3) {
    for r in 0..(ROW_COUNT - 3) {
        for i in 0..4 {
            if self.positions[r + i][c + i] == State::Player(self.current_player) {
                win_seq = win_seq + 1;
            }
            if win_seq == 4 {
                return true;
            }
        }
        win_seq = 0;
    }
}
return false;