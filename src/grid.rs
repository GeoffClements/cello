use core::fmt;
use std::ops::{Index, IndexMut};

pub struct Row {
    row: Vec<bool>,
    true_char: char,
    false_char: char,
}

impl Row {
    pub fn new(width: usize) -> Self {
        Self {
            row: vec![false; width],
            true_char: '\u{2588}',
            false_char: '\u{2B1A}',
        }
    }

    pub fn step(self) -> Self {
        Self {
            row: (0..self.row.len()).map(|loc| self.rule(loc)).collect(),
            true_char: self.true_char,
            false_char: self.false_char,
        }
    }

    fn prev_gen(&self, loc: usize) -> (bool, bool, bool) {
        (
            if loc == 0 { false } else { self[loc - 1] },
            self[loc],
            if loc + 1 == self.row.len() {
                false
            } else {
                self[loc + 1]
            },
        )
    }

    fn rule(&self, loc: usize) -> bool {
        let (x, y, z) = self.prev_gen(loc);
        (x&y&!z) | (x&!y&!z) | (!x&y&z) | (!x&!y&z)
        // (x&!y&!z) | (!x&y&z) | (!x&y&!z) | (!x&!y&z)
        // (x&y&!z) | (x&!y&z) | (!x&y&z) |(!x&y&!z) | (!x&!y&z)
    }
}

impl Index<usize> for Row {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.row[index]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.row[index]
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .row
            .iter()
            .fold(String::with_capacity(self.row.len()), |mut s, v| {
                if *v {
                    s.push(self.true_char)
                } else {
                    s.push(self.false_char)
                }
                s
            });
        write!(f, "{s}")
    }
}
