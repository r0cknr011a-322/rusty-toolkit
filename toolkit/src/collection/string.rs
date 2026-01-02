use crate::collection::deque::{ Deque };

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct String<const LEN: usize> {
    deque: Deque<u8, LEN>,
}

impl<const LEN: usize> String {
    pub fn new(s: &str) -> Self {
        let deque = Deque::default();
        for b in s.as_bytes() {
            deque.push_back(b);
        }
    }
}
