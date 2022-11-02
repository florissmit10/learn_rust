use std::collections::HashMap;


pub struct Sea {
    fish: HashMap<u8, u128>
}

impl Sea {
    pub fn new(input: Vec<u32>) -> Self{
        let fish = input.iter()
            .fold(HashMap::new(),|mut map, num|{
                *map.entry(*num as u8).or_insert(0)+=1;
                map
            });
        Sea{fish}
    }

    fn proceed_day(&mut self) {
        let mut next_generation: HashMap<u8, u128> = HashMap::new();
        next_generation.insert(6, *self.fish.get(&0).unwrap_or(&0));
        next_generation.insert(8, *self.fish.get(&0).unwrap_or(&0));

        for age in 1..=8 {
            let entry =(next_generation.entry(age-1)).or_insert(0);
            *entry+= *self.fish.entry(age).or_insert(0);
        }

        self.fish = next_generation.clone()
    }

    pub fn proceed_days(&mut self, days: u32) {
        for _ in 0..days {
            self.proceed_day();
        }
    }

    pub fn get_number_of_fish(&mut self) -> u128 {
        self.fish.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_sea(result: &Sea, expected: [u128; 9]) {
        let mut expected_map: HashMap<u8, u128> = HashMap::new();
        for i in 0..=8 {
            expected_map.insert(i as u8, *expected.get(i as usize).unwrap());
        }

        assert_eq!(result.fish, expected_map);

    }

    #[test]
    fn test_new_sea() {
        let input = vec![0,1,2,3,4,5,6,7,8];
        let sea = Sea::new(input);

        compare_sea(&sea, [1,1,1,1,1,1,1,1,1]);
    }

    #[test]
    fn test_sea_new_fish() {
        let input = vec![0];
        let mut sea = Sea::new(input);
        sea.proceed_day();

        compare_sea(&sea, [0,0,0,0,0,0,1,0,1]);
    }


    #[test]
    fn test_sea_proceed_day() {
        let input = vec![0,0,0,1,2,3,4,5,6,7,8];
        let mut sea = Sea::new(input);
        sea.proceed_day();

        compare_sea(&sea, [1,1,1,1,1,1,4,1,3]);
    }

    #[test]
    fn test_sea_proceed_days() {
        let input = vec![0,0,0,1,2,3,4,5,6,7,8];
        let mut sea = Sea::new(input);
        sea.proceed_day();
        compare_sea(&sea, [1,1,1,1,1,1,4,1,3]);
        sea.proceed_day();
        compare_sea(&sea, [1,1,1,1,1,4,2,3,1]);
        sea.proceed_day();
        compare_sea(&sea, [1,1,1,1,4,2,4,1,1]);
    }

}