
pub struct LanternFish {
    pub reproduce_in_days: u32,
}

impl LanternFish {
    fn proceed_day(&mut self) -> Option<LanternFish> {
        match self.reproduce_in_days {
            0 => {
                self.reproduce_in_days = 6;
                Some(LanternFish{reproduce_in_days: 8})
            }
            _ => {
                self.reproduce_in_days -= 1;
                None
            }
        }
    }
}

pub struct Sea {
    pub fish: Vec<LanternFish>
}

impl Sea {
    pub fn new(input: &str) -> Self{
        let fish = input.split(',')
            .map(|num| LanternFish{reproduce_in_days: num.parse().unwrap()} )
            .collect();
        Sea{fish}
    }

    fn proceed_day(&mut self) {
        let new_fish =
            self.fish.iter_mut().filter_map(|f|f.proceed_day()).collect::<Vec<_>>();
        self.fish.extend(new_fish);
    }

    pub fn proceed_days(&mut self, days: u32) {
        for _ in 0..days {
            self.proceed_day();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lantern_fish_proceed() {
        let mut lantern_fish = LanternFish{ reproduce_in_days: 5};
        let result = lantern_fish.proceed_day();

        assert!(result.is_none());
        assert_eq!(lantern_fish.reproduce_in_days, 4)
    }

    #[test]
    fn test_lantern_fish_proceed_zero() {
        let mut lantern_fish = LanternFish{ reproduce_in_days: 1};
        let result = lantern_fish.proceed_day();

        assert!(result.is_none());
        assert_eq!(lantern_fish.reproduce_in_days, 0)
    }

    #[test]
    fn test_lantern_fish_proceed_new() {
        let mut lantern_fish = LanternFish{ reproduce_in_days: 0};
        let result = lantern_fish.proceed_day();

        assert!(result.is_some());
        assert_eq!(result.unwrap().reproduce_in_days, 8);
        assert_eq!(lantern_fish.reproduce_in_days, 6)
    }

    #[test]
    fn test_new_sea() {
        let input = "2,4,1";
        let sea = Sea::new(input);

        assert_eq!(sea.fish.iter().len(), 3);
        assert_eq!(sea.fish.iter().map(|f| f.reproduce_in_days).collect::<Vec<_>>(), vec![2,4,1]);
    }

    #[test]
    fn test_sea_proceed_day() {
        let input = "2,4,0";
        let mut sea = Sea::new(input);

        sea.proceed_day();

        assert_eq!(sea.fish.iter().len(), 4);
        assert_eq!(sea.fish.iter().map(|f| f.reproduce_in_days).collect::<Vec<_>>(), vec![1,3,6,8]);
    }

    #[test]
    fn test_sea_proceed_day_twice() {
        let input = "2,4,0";
        let mut sea = Sea::new(input);

        sea.proceed_day();
        sea.proceed_day();

        assert_eq!(sea.fish.iter().len(), 4);
        assert_eq!(sea.fish.iter().map(|f| f.reproduce_in_days).collect::<Vec<_>>(), vec![0,2,5,7]);
    }

    #[test]
    fn test_sea_proceed_days() {
        let input = "2,4,0";
        let mut sea = Sea::new(input);

        sea.proceed_days(2);

        assert_eq!(sea.fish.iter().len(), 4);
        assert_eq!(sea.fish.iter().map(|f| f.reproduce_in_days).collect::<Vec<_>>(), vec![0,2,5,7]);
    }
}