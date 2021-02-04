use serde::{Deserialize, Serialize};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default,
)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Span {
        assert!(end >= start);

        Span { start, end }
    }

    pub fn start(self) -> u32 {
        self.start
    }

    pub fn end(self) -> u32 {
        self.end
    }
}
