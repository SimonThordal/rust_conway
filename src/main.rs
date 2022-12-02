use rand::{thread_rng, Rng};

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
        // Given a max board size
        let n = 10;
        // Get a coordinate on the board
        let coord = get_coordinates(n);
        // The coordinate will point to a cell on the board
        assert!(coord[0] < n);
        assert!(coord[1] < n);
    }

    #[test]
    fn test_can_get_neighbors_from_coordinates() {
        // Given a set of coordinates
        let coords = [3,4];
        // Given a board size
        let n = 6;
        // And its neighbors
        let expected = [[3,5], [3,3], [4,4], [2, 4]];
        // We can get the neighbors of the coordinates
        let actual = get_coordinate_neighbors(coords, n);
        assert_eq!(actual, expected);
        // And coordinates on the edge of that board
        let coords = [0, 2];
        // We expect the neigbors to wrap around the board
        let expected = [[0,3], [0,1], [1,2], [5,2]];
        let actual = get_coordinate_neighbors(coords, n);
        assert_eq!(actual, expected);
    }

    #[test]
    fn can_get_set_of_neighbors() {
        // Given a set of neighbors
        // We can get the count of each coordinate
    }
}