#![allow(dead_code)]
#![allow(unused_variables)]
use rand::{thread_rng, Rng};

fn print_board(board: &[bool]) {
	const LIVING: &str = "▢";
	let board_size = (board.len() as f64).sqrt() as usize;
	for i in 0..board_size {
		for j in 0..board_size {
			if board[i*board_size+j] {
				print!("{}", LIVING);
			}
			else {
				print!(" ");
			}
		}
		println!("");
	} 
}

/// Generates a randomized board
/// 
/// # Arguments
/// 
/// * `n` The dimension of the board
/// 
/// # Returns
/// 
/// An n by n board with randomly assigned boolean values
/// 
fn generate_board(n: u32) -> Vec<bool> {
	let mut board: Vec<bool> = vec![];
	let mut rng = thread_rng();
	for _ in 0..n.pow(2) {
		board.push(rng.gen_bool(0.5));
	}
	board
}

/// Checks whether a given row and column index are within the bounds of a square board with a
/// given size.
///
/// # Arguments
///
/// * `row` - The row index to check.
/// * `col` - The column index to check.
/// * `board_size` - The size of the board.
///
/// # Returns
///
/// True if the given row and column index are within the bounds of the board, false otherwise.
///
fn is_within_bounds(row: i32, col: i32, board_size: i32) -> bool {
    row >= 0 && row < board_size && col >= 0 && col < board_size
}

/// Counts the number of living neighbors of a cell at a given index on a square board represented
/// as a slice of bools. A living neighbor is a cell adjacent to the given cell that is also alive.
/// Diagonal neighbors are considered adjacent.
///
/// # Arguments
///
/// * `ix` - The index of the cell to check for living neighbors.
/// * `board` - The board represented as a slice of bools.
///
/// # Returns
///
/// The number of living neighbors of the cell at the given index.
///
fn living_neighbors_count(ix: i32, board: &[bool]) -> i32 {
    let mut count = 0;
    let board_size = (board.len() as f64).sqrt() as i32;
    let current_row = ix / board_size;
    for i in -1..=1 {
        let row_offset = i * board_size;
        for j in -1..=1 {
        	// Do not examine ix itself
            if i == 0 && j == 0 {
                continue;
            }
            let neighbor_row = current_row + i;
            let neighbor_col = (ix % board_size) + j;
            if is_within_bounds(neighbor_row, neighbor_col, board_size) {
                let neighbor_ix = neighbor_row * board_size + neighbor_col;
                if board[neighbor_ix as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

/// Determines whether a cell with a given living status and number of living neighbors will survive
/// to the next generation. A cell will survive if it is currently alive and has 2-3 living neighbors,
/// or if it is currently dead and has exactly 3 living neighbors.
///
/// # Arguments
///
/// * `is_living` - Whether the cell is currently living (true) or dead (false).
/// * `n_neighbors` - The number of living neighbors of the cell.
///
/// # Returns
///
/// True if the cell will survive to the next generation, false otherwise.
///
fn will_survive(is_living: bool, n_neighbors: u32) -> bool {
	if !is_living {
		if n_neighbors == 3 {
			return true
		}
		return false
	}
	let res = match n_neighbors {
		0..=1 => false,
		2..=3 => true,
		_ => false
	};
	res
}

/// Advances the state of the board to the next generation according to the rules of Conway's Game of Life.
///
/// # Arguments
///
/// * `board` - The current state of the board represented as a slice of bools.
///
/// # Returns
///
/// The next state of the board represented as a vector of bools.
///
fn advance_board(board: &[bool]) -> Vec<bool> {
	let mut next_state: Vec<bool> = vec![];
	for (i, x) in board.iter().enumerate() {
		let n = living_neighbors_count(i as i32, board);
		next_state.push(will_survive(*x, n as u32));
	}
	return next_state;
}

fn main() {
	let board_size = 9;
	let n_generations: u32 = 100;
	let mut t_curr = generate_board(board_size);
	for i in 0..n_generations {
		println!("Generation {}.", i);
		print_board(&t_curr);
		t_curr = advance_board(&t_curr);
		println!("");
	}
}

#[cfg(test)]
mod tests {

	use crate::*;

	fn get_test_board() -> Vec<bool> {
		vec![false, true, true, 
			 false, true, false, 
			 false, false, false]
	}

	#[test]
	fn test_is_within_bounds() {
		assert_eq!(is_within_bounds(1,2, 3), true);
		assert_eq!(is_within_bounds(-1, 2, 3), false);
		assert_eq!(is_within_bounds(0,4, 3), false);
	}

	#[test]
	fn can_generate_a_random_board() {
		let board = generate_board(3);
		assert_eq!(board.len(), 9);
	}

	#[test]
	fn can_increment_generation() {
		let mut test_board = get_test_board();
		test_board = advance_board(&test_board);
		let expected = vec![false, true, true,
							false, true, true,
							false, false, false];
		assert_eq!(test_board, expected);
	}

	#[test]
	fn can_get_neighbors() {
		let test_board = get_test_board();
		let test_cases = [2,2,2,
						  2,2,3,
						  1,1,1];
		for (i, case) in test_cases.iter().enumerate() {
			println!("At index {} with case: {}", i, case);
			let n = living_neighbors_count(i as i32, &test_board);
			assert_eq!(n, *case);
		}
	}

	#[test]
	fn dies_on_one_or_less_neighbors() {
		assert_eq!(false, will_survive(true, 0));
		assert_eq!(false, will_survive(true, 1));
	}

	#[test]
	fn survives_with_two_or_three_neighbors() {
		assert_eq!(true, will_survive(true, 2));
		assert_eq!(true, will_survive(true, 3));
	}

	#[test]
	fn dies_with_four_or_more_neighbors() {
		assert_eq!(false, will_survive(true, 4));
		assert_eq!(false, will_survive(true, 999));
	}

	#[test]
	fn born_with_exactly_three_neighbors() {
		assert_eq!(true, will_survive(false, 3));
	}
}