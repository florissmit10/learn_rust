use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<x1>\d*),(?P<y1>\d*) -> (?P<x2>\d*),(?P<y2>\d*)").unwrap();
        }

#[derive(Debug)]
pub struct VentPoint {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct VentLine {
    start_point: VentPoint,
    end_point: VentPoint,
}

impl VentLine {
    pub fn new(input: &str) -> Self {
        let (x1, y1, x2, y2) =
            RE.captures(input).unwrap()
                .iter()
                .filter_map(|m| m?.as_str().parse::<u32>().ok())
                .next_tuple().unwrap();

        VentLine {
            start_point: VentPoint { x: x1, y: y1 },
            end_point: VentPoint { x: x2, y: y2 },
        }
    }

    pub fn is_straight(&self) -> bool {
        self.is_horizontal() ||
            self.is_vertical()
    }


    pub fn is_diagonal(&self) -> bool {
        let y_diff = self.start_point.y as i32 - self.end_point.y as i32;
        let x_diff = self.start_point.x as i32 - self.end_point.x as i32;

        x_diff.abs() == y_diff.abs()
    }

    pub fn is_horizontal(&self) -> bool {
        self.start_point.y == self.end_point.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start_point.x == self.end_point.x
    }

    pub fn get_all_points(&self) -> Vec<VentPoint> {
        if self.is_horizontal() {
            return get_point_range(self.start_point.x, self.end_point.x)
                .iter()
                .map(|x| VentPoint { x: *x, y: self.start_point.y })
                .collect::<Vec<_>>();
        } else if self.is_vertical() {
            return get_point_range(self.start_point.y, self.end_point.y)
                .iter()
                .map(|y| VentPoint { x: self.start_point.x, y: *y })
                .collect::<Vec<_>>();
        } else if self.is_diagonal() {
            let x_range = get_point_range(self.start_point.x, self.end_point.x);
            let y_range = get_point_range(self.start_point.y, self.end_point.y);
            return x_range.iter().zip(y_range.iter())
                .map(|(x, y)| VentPoint { x: *x, y: *y })
                .collect::<Vec<_>>();
        } else {
            panic!("VentLine is neither diagonal, vertical or horizontal {:?}", self)
        }
    }
}


fn get_point_range(p1: u32, p2: u32) -> Vec<u32> {
    if p1 < p2 { (p1..=p2).collect() } else { (p2..=p1).rev().collect_vec() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_points_range() {
        let (x, y) = (5, 1);

        let range = get_point_range(x, y);

        assert_eq!(range, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_new_vent_point() {
        let input = "1,2 -> 1,4";

        let line = VentLine::new(input);

        assert_eq!(line.start_point.x, 1);
        assert_eq!(line.start_point.y, 2);
        assert_eq!(line.end_point.x, 1);
        assert_eq!(line.end_point.y, 4);
    }

    #[test]
    fn test_new_vent_point_reverse() {
        let input = "1,4 -> 1,2";

        let line = VentLine::new(input);

        assert_eq!(line.start_point.x, 1);
        assert_eq!(line.start_point.y, 4);
        assert_eq!(line.end_point.x, 1);
        assert_eq!(line.end_point.y, 2);
    }

    #[test]
    fn test_get_all_points_horizontal() {
        let input = VentLine::new("0,1 -> 5,1");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1)])
    }

    #[test]
    fn test_get_all_points_horizontal_reverse() {
        let input = VentLine::new("5,1 -> 0,1");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(5, 1), (4, 1), (3, 1), (2, 1), (1, 1), (0, 1)])
    }

    #[test]
    fn test_get_all_points_vertical() {
        let input = VentLine::new("1,1 -> 1,5");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(1, 1), (1, 2), (1, 3), (1, 4), (1, 5)])
    }

    #[test]
    fn test_get_all_points_vertical_reverse() {
        let input = VentLine::new("1,5 -> 1,1");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(1, 5), (1, 4), (1, 3), (1, 2), (1, 1)])
    }

    #[test]
    fn test_get_all_points_diagonal_offset_y() {
        let input = VentLine::new("1,10 -> 5,14");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(1, 10), (2, 11), (3, 12), (4, 13), (5, 14)])
    }

    #[test]
    fn test_get_all_points_diagonal_offset_y_reverse() {
        let input = VentLine::new("5,14 -> 1,10");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(5, 14), (4, 13), (3, 12), (2, 11), (1, 10)])
    }

    #[test]
    fn test_get_all_points_diagonal_offset_x() {
        let input = VentLine::new("10,1 -> 14,5");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(10, 1), (11, 2), (12, 3), (13, 4), (14, 5)])
    }

    #[test]
    fn test_get_all_points_diagonal_offset_x_reverse() {
        let input = VentLine::new("14,5 -> 10,1");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(14, 5), (13, 4), (12, 3), (11, 2), (10, 1)])
    }

    #[test]
    fn test_get_all_points_diagonal_double_reverse() {
        let input = VentLine::new("1,5 -> 4,2");

        let result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(result, vec![(1, 5), (2, 4), (3, 3), (4, 2)])
    }

    #[should_panic]
    #[test]
    fn test_get_all_points_sloped() {
        let input = VentLine::new("1,1 -> 3,6");

        let _result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();
    }

    #[should_panic]
    #[test]
    fn test_get_all_points_sloped_reverse() {
        let input = VentLine::new("3,6 -> 1,1");

        let _result: Vec<(u32, u32)> = input.get_all_points().iter().map(|p| (p.x, p.y)).collect();
    }
}
