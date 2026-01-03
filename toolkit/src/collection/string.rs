use crate::collection::deque::{ Deque };

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct String<const LEN: usize> {
    deque: Deque<u8, LEN>,
}

impl<const LEN: usize> String<LEN> {
    pub fn new(s: &str) -> Self {
        let mut deque = Deque::<u8, LEN>::default();
        for b in s.as_bytes() {
            if deque.is_full() {
                deque.pop_front();
            }
            deque.push_back(*b);
        }
        Self {
            deque: deque,
        }
    }
}
