use getset::Getters;
use std::ops::Range;

#[derive(Debug, Getters)]
pub struct Slice<T: Clone> {
    #[getset(get = "pub")]
    vec: Vec<T>,
    #[getset(get = "pub")]
    range: Range<usize>,
}

impl<T: Clone> Slice<T> {
    pub fn new(vec: Vec<T>, start: usize, end: EndIndex) -> Self {
        let end = match end {
            EndIndex::I(i) => {
                if i < vec.len() {
                    i
                } else {
                    vec.len()
                }
            }
            EndIndex::ToEnd => vec.len(),
        };

        Self {
            vec,
            range: Range { start, end },
        }
    }

    pub fn clone(&self) -> Self {
        Self::new(
            self.vec().clone(),
            self.range().start,
            EndIndex::I(self.range().end),
        )
    }
}

pub enum EndIndex {
    I(usize),
    ToEnd,
}
