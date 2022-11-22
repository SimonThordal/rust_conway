/// Gets a random coordinate
/// 
/// # Examples 
/// 
/// ```
/// let n = 10;
/// let coord = conway::get_coordinates(n);
/// assert!(coord[0] < n)
/// assert!(coord[1] < n)
/// ``` 
pub fn get_coordinates(n: u32) -> (u32, u32) {
    (1,1)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coordinates() {
        assert_eq!(get_coordinates(10), (1,1));
    }
}