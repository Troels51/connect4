# Connect 4
Basic Connect 4 solver, created in Rust.
This uses negamax with  to solve games from the midgame.
It uses a couple of optimizations:
 - Alpha-beta pruning to reduce the game tree
 - Bitboard to represent the board state
The solver is not finished, more optimizations need to be added before it can solve boards from the beginning. The main one being a cached opening book

