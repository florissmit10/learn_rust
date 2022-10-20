use std::fs;

fn main(){
    let input: Vec<i32> =
        fs::read_to_string("input.txt")
            .expect("Cannot read input.txt")
            .lines()
            .map(|l|l.trim().parse().unwrap())
            .collect();
    let result = larger_than_prev(input.clone());
    println!("{result} values larger than previous");

    let result_window = larger_window_than_pref(input, 3);
    println!("{result_window} values larger than previous")
}


fn larger_than_prev(input: Vec<i32>) -> i32 {
    input.windows(2).filter(|w| w[1] > w[0]).count() as i32
}

fn larger_window_than_pref(input: Vec<i32>, window_size: usize) -> i32 {
    let windows: Vec<i32> = input.windows(window_size).map(|w| w.iter().sum()).collect();

    larger_than_prev(windows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input = vec![];

        let result = larger_than_prev(input);

        assert_eq!(result, 0)
    }

    #[test]
    fn test_no_larger() {
        let input = vec![1,1,1,1];

        let result = larger_than_prev(input);

        assert_eq!(result, 0)
    }

    #[test]
    fn test_succes() {
        let input = vec![2,1, 2];

        let result = larger_than_prev(input);

        assert_eq!(result, 1)
    }


    #[test]
    fn test_succes_windows_larger() {
        let input = vec![1,1, 2,3,4,5,1,0];

        let result = larger_window_than_pref(input, 3);

        assert_eq!(result, 3)
    }

    #[test]
    fn test_windows_empty() {
        let input = vec![];

        let result = larger_window_than_pref(input, 3);

        assert_eq!(result, 0)
    }

    #[test]
    fn test_windows_no_larger() {
        let input = vec![1,1,1,1];

        let result = larger_window_than_pref(input, 3);

        assert_eq!(result, 0)
    }


    #[test]
    fn test_windows_succes() {
        let input = vec![2,1, 2,5,4,5,10];

        let result = larger_window_than_pref(input, 3);

        assert_eq!(result, 4)
    }
}