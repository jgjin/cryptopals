pub struct Permutations {
    cur_perm: Vec<usize>,
    first: bool,
    possibilities: Vec<Vec<u8>>,
    ranges: Vec<usize>,
}

impl Permutations {
    pub fn new(
        possibilities: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            cur_perm: vec![0; possibilities.len()],
            ranges: possibilities.iter().map(|place| {
                place.len() - 1
            }).collect(),
            possibilities: possibilities,
            first: true,
        }
    }

    fn make_perm(
        &self,
    ) -> Option<Vec<u8>> {
        self.cur_perm.iter().enumerate().map(|(place, index)| {
            self.possibilities.get(place).and_then(|place_poss| {
                place_poss.get(*index).map(|byte| {
                    *byte
                })
            })
        }).collect()
    }
    
    pub fn next(
        &mut self,
    ) -> Option<Vec<u8>> {
        if self.first {
            self.first = false;
            return self.make_perm();
        }

        let mut idx_to_change = 0;
        while idx_to_change < self.cur_perm.len() &&
            self.cur_perm[idx_to_change] == self.ranges[idx_to_change] {
                self.cur_perm[idx_to_change] = 0;
                idx_to_change += 1;
            }

        if idx_to_change == self.cur_perm.len() {
            return None;
        }

        self.cur_perm[idx_to_change] += 1;
        return self.make_perm();
    }
}
