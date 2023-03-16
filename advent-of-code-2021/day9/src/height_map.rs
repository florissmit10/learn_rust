pub struct HeightMap {
    points: Vec<Vec<u8>>,
}

impl HeightMap {
    pub fn new(points: Vec<Vec<u8>>) -> Self { HeightMap { points } }
    pub fn get_danger_level(&self, x: usize, y: usize) -> u8 {
        0
    }

    fn get_adjacent_heights(&self, x: usize, y: usize) -> Vec<u8> {

        let mut result: Vec<u8> = vec![];
        for x in x-1..=x+1 {
            for y in y-1..=y+1 {
                let point = self.points.get(y).map(|row| row.get(x));

                 match point {
                     Some(Some(p)) => {result.push(*p)},
                     Some(None) => {},
                     None => {},
                 }

            }
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adjacent_heights() {
        let input = vec![
            vec![1,2,3,4],
            vec![11,12,13,14],
            vec![21,22,23,24],
            vec![31,32,33,34]
        ];

        let map = HeightMap::new(input);

        assert_eq!(vec![2,11], map.get_adjacent_heights(0,0));
        assert_eq!(vec![1,3,12], map.get_adjacent_heights(1,0));

        assert_eq!(vec![2, 11,13,22], map.get_adjacent_heights(1,1));

        assert_eq!(vec![33,24], map.get_adjacent_heights(3,3));
    }
}