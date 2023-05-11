use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct Board {
    alive: Vec<Coordinates>,
    clock: i32,
    n: i32
}

fn will_be_alive(living_neighbors: usize) -> bool {
    if (living_neigbors > 2) {
        return true
    }
    return false
}

impl Board {
    // Creates a new n by n board
    fn new(n: i32) -> Board {
        Board {
            n,
            clock: 0,
            alive: vec![]
        }
    }

    fn tick(&mut self) {
        self.clock += 1
    }

    // Given a HashMap where each cell that has been a neighbor
    // is represented as 
    // {
    //     x_0: [y_0, y_0, y_1:]   
    // }

    // Gets the neighbors for the entire board returned as a HashMap
    // where (0,0), (0,1) and (2,0) would be represented as
    // {
    //   x_0: [y_0, y_1]
    //   x_2: [y_0]    
    //}
    fn get_neighbors(&self) -> HashMap<i32, Vec<i32>> {
        let mut neighbors = HashMap::new();
        for &living in self.alive.iter() {
            for neighbor in living.neighbors() {
                if !neighbors.contains_key(&neighbor.x) {
                    neighbors.insert(neighbor.x.clone(), vec![]);
                }
                let y_coords = neighbors.get_mut(&neighbor.x).unwrap();
                y_coords.push(neighbor.y.clone())
            }
        }
        neighbors
    }

    fn randomize(&mut self, n_living: i32) {
        for _ in 0..n_living {
            self.alive.push(Coordinates::random(self.n))
        }
    }

    fn set_living_cells(&mut self, alive: Vec<Coordinates>) {
        self.alive = alive;
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinates {
    x: i32,
    y: i32
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Coordinates {
        Coordinates {
            x: x,
            y: y
        }
    }

    fn random(n: i32) -> Coordinates {
        let mut rng = thread_rng();
        Coordinates {
            x: rng.gen_range(0..n),
            y: rng.gen_range(0..n)
        }
    }

    /// Get the coordinates of a cells neighbors
    /// wrapping to the other side of the board for edge cells
    fn neighbors(&self) -> Vec<Coordinates> {
        let n = 10;
        let mut res: Vec<Coordinates> = Vec::with_capacity(9);
        for i in -1..2 {
            for j in -1..2 {
                res.push(Coordinates::new((self.x+j).rem_euclid(n), (self.y+i).rem_euclid(n)));
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coordinates() {
        // Get a coordinate on the board
        let coords = Coordinates::new(1,2);
        assert_eq!(coords.x, 1);
        assert_eq!(coords.y, 2);
        // The coordinate will point to a cell on the board
        // Given a max board size get a random coordinate
        let n = 10;
        let coords = Coordinates::random(n);
        assert!(coords.x < n && coords.x >= 0);
        assert!(coords.y < n && coords.y >= 0);
    }

    #[test]
    fn test_can_get_neighbors_from_coordinates() {
        // Given a set of coordinates
        let coords = Coordinates::new(3,4);
        // Given a board size
        let n = 6;
        // And its neighbors
        let expected = [
            [2,3],
            [3,3],
            [4,3],
            [2,4],
            [3,4],
            [4,4],
            [2,5],
            [3,5],
            [4,5]
        ];
        // We can get the neighbors of the coordinates
        let actual = coords.neighbors();
        for i in 0..9 {
            let b = actual[i];
            assert_eq!(expected[i][0], b.x);
            assert_eq!(expected[i][1], b.y);

        }
    }

    #[test]
    fn can_get_a_new_board() {
        // Given a new board
        let board = Board::new(10);
        // It should not have any living cells
        assert_eq!(0, board.alive.len());
    }

    #[test]
    fn can_set_board_state() {
        let mut board = Board::new(10);
        let mut new_state: Vec<Coordinates> = vec![];
        new_state.push(Coordinates::new(1,2));
        new_state.push(Coordinates::new(3,4));
        board.set_living_cells(new_state.clone());
        assert_eq!(2, board.alive.len());
        for coord in board.alive {
            assert!(new_state.contains(&coord));
        }
    }

    #[test]
    fn can_get_neighbors() {
        let mut board = Board::new(10);
        // Given a single cell in (2,2) we can return its neighbors
        // in (1..3, 1..3)
        board.set_living_cells(vec![Coordinates::new(2,2)]);
        let neighbors =  board.get_neighbors();
        let mut expected = HashMap::new();
        expected.insert(1, vec![1,2,3]);
        expected.insert(2, vec![1,2,3]);
        expected.insert(3, vec![1,2,3]);
        for (key, val) in expected.iter() {
            assert!(neighbors.contains_key(&key));
            assert_eq!(*neighbors.get(&key).unwrap(), *val);
        }
        // Given the cells (2,2) and (3,3) the neighbor (2,3) appears twice
        board.set_living_cells(vec![Coordinates::new(2,2), Coordinates::new(3,3)]);
        let mut cnt = 0;
        let neighbors = board.get_neighbors();
        for y_coord in neighbors.get(&2).unwrap().iter() {
            if *y_coord == 3 {
                cnt += 1;
            }
        }
        assert_eq!(2, cnt);
    }

    #[test]
    fn can_increment_board_state() {
        let mut board = Board::new(10);
        // Given a set of currently living cells
        board.set_living_cells(vec![Coordinates::new(2,2), Coordinates::new(3,3)]);
        // It is possible to increment the clock of the board
        board.tick();
        assert_eq!(1, board.clock);
        // Get the new state of the board
        // If a cell appears twice more in the set it will be alvie in the next iteration
        let expected = [Coordinates::new(2,3), Coordinates::new(3,2)];
        assert_eq!(2, board.alive.len());
        for coord in board.alive {
            assert!(expected.contains(&coord));
        }
    }

    #[test]
    fn can_create_a_board_with_x_living_cells() {
        let mut board = Board::new(5);
        board.randomize(2);
        assert_eq!(2, board.alive.len());
    }
}