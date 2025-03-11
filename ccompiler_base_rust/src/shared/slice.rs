use getset::Getters;
use std::ops::Range;

#[derive(Debug, Getters)]
pub struct Slice<'a, T> {
    #[getset(get = "pub")]
    vec: &'a Vec<T>,
    #[getset(get = "pub")]
    range: Range<usize>,
}

impl<'a, T> Slice<'a, T> {
    pub fn new(vec: &'a Vec<T>, start: usize, end: EndIndex) -> Self {
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
}

pub enum EndIndex {
    I(usize),
    ToEnd,
}
