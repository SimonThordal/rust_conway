use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
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

    fn neighbors(&self) -> [Option<Coordinates>; 9] {
        let n = 10;
        let mut res: [Option<Coordinates>; 9] = [None; 9];
        let mut ix = 0;
        for i in -1..2 {
            for j in -1..2 {
                res[ix] = Some(Coordinates::new((self.x+j).rem_euclid(n), (self.y+i).rem_euclid(n)));
                ix += 1;
            }
        }
        return res;
    }
}

/// Gets the coordinates of a board cell
/// 
/// # Arguments
/// 
/// * `n` - The size of the board
/// 
/// # Examples 
/// 
/// ```
/// let n = 10;
/// let coord = conway::get_coordinates(n);
/// assert!(coord.0 < n)
/// assert!(coord.1 < n)
/// ``` 
pub fn get_coordinates(n: i32) -> [i32; 2] {
    let mut rng = thread_rng();
    [rng.gen_range(0..n), rng.gen_range(0..n)]
}

/// Get the coordinates of a cells neighbors
/// wrapping to the other side of the board for edge cells
/// 
/// # Arguments
/// 
/// * `coords` - The coordinates to find neighbors for
/// * `n` - Thes size of the board
pub fn get_coordinate_neighbors(coords: [i32; 2], n: i32) -> [[i32; 2]; 4] {
    let mut res: [[i32; 2]; 4] = [[0,0]; 4];
    res[0] = [coords[0], (coords[1]+1).rem_euclid(n)]; 
    res[1] = [coords[0], (coords[1]-1).rem_euclid(n)]; 
    res[2] = [(coords[0]+1).rem_euclid(n), coords[1]]; 
    res[3] = [(coords[0]-1).rem_euclid(n), coords[1]]; 
    return res;
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
            let b = actual[i].unwrap();
            assert_eq!(expected[i][0], b.x);
            assert_eq!(expected[i][1], b.y);

        }
    }

    #[test]
    fn can_get_set_of_neighbors() {
        // Given a set of neighbors
        // We can get the count of each coordinate
    }
}