use std::{
    ops::{
        Range,
    },
};

pub struct Permutations {
    cur: Vec<u8>,
    range: Range<u8>,
    first: bool,
}

impl Permutations {
    pub fn new(
        range: Range<u8>,
        size: usize,
    ) -> Self {
        Self {
            cur: vec![range.start; size],
            range: range,
            first: true,
        }
    }

    pub fn next(
        &mut self,
    ) -> Option<Vec<u8>> {
        if self.first {
            self.first = false;
            return Some(self.cur.clone());
        }
        
        let mut idx_to_change = 0;
        while idx_to_change < self.cur.len() &&
            self.cur[idx_to_change] == self.range.end {
                self.cur[idx_to_change] = 0;
                idx_to_change += 1;
            }

        if idx_to_change == self.cur.len() {
            return None;
        }

        self.cur[idx_to_change] += 1;
    
        return Some(self.cur.clone());
    }
}
