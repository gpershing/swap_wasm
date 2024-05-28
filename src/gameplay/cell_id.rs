use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub struct CellId(NonZeroUsize);

pub struct CellIdProvider {
    at: NonZeroUsize
}

impl CellIdProvider {
    pub const fn new() -> CellIdProvider {
        Self { at: NonZeroUsize::MIN }
    }
    
    pub fn next(&mut self) -> CellId {
        let id = CellId(self.at);
        self.at = self.at.saturating_add(1);
        id
    }
}