use core::array::{ from_fn };
use core::ops::{ Range };
use core::slice::{ Iter, IterMut };
use core::iter::{ FusedIterator };
use crate::collection::cursor::{ Cursor };

#[derive(Debug)]
pub struct Deque<I, const L: usize> {
    buf: [I; L],
    tail: Cursor<L, true>,
    head: Cursor<L, true>,
    full: bool,
    stack: bool,
}

impl<I, const L: usize>
Clone for Deque<I, L>
where I: Clone {
    fn clone(&self) -> Self {
        Self {
            buf: self.buf.clone(),
            head: self.head, tail: self.tail,
            full: self.full, stack: self.stack,
        }
    }
}

impl<I, const L: usize>
Copy for Deque<I, L>
where I: Copy { }

impl<I, const LEN: usize>
PartialEq<Self> for Deque<I, LEN>
where I: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut lhead = self.head;
        let mut rhead = other.head;
        for _ in 0..self.len() {
            if self.buf[lhead.pos()] != other.buf[rhead.pos()] {
                return false;
            }
            lhead.next();
            rhead.next();
        }
        true
    }
}

impl<I, const LEN: usize>
Eq for Deque<I, LEN>
where I: Eq { }

impl<I, const LEN: usize>
Default for Deque<I, LEN>
where I: Default {
    fn default() -> Self {
        Self {
            buf: from_fn(|_| I::default()),
            head: Cursor::new(0), tail: Cursor::new(0),
            full: false, stack: false,
        }
    }
}

impl<I, const LEN: usize>
Deque<I, LEN> {
    pub fn new<Ctr: FnMut(usize) -> I>(ctr: Ctr) -> Self {
        Self {
            buf: from_fn(ctr),
            head: Cursor::new(0), tail: Cursor::new(0),
            full: false, stack: false,
        }
    }

    pub fn is_stack(&self) -> bool {
        self.stack
    }

    pub fn set_stack(&mut self, stack: bool) {
        self.stack = stack;
    }

    pub(crate) fn head(&self) -> usize {
        self.head.pos()
    }

    pub(crate) fn tail(&self) -> usize {
        self.tail.pos()
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    fn slice_ranges(&self) -> (Range<usize>, Range<usize>) {
        if self.head.pos() == self.tail.pos() && !self.full {
            return (
                Range { start: 0, end: 0 },
                Range { start: 0, end: 0 },
            );
        }
        if self.head.pos() < self.tail.pos() {
            return (
                Range { start: self.head.pos(), end: self.tail.pos() },
                Range { start: 0, end: 0 },
            );
        }
        return (
            Range { start: self.head.pos(), end: LEN },
            Range { start: 0, end: self.tail.pos() },
        );
    }

    pub fn len(&self) -> usize {
        let (first, second) = self.slice_ranges();
        first.len() + second.len()
    }

    fn as_slices(&self) -> (&[I], &[I]) {
        let (left, right) = self.slice_ranges();
        let (left_left, left_right) = self.buf.split_at(left.end);
        let (_, right_right) = left_right.split_at(right.start);
        (left_left, right_right)
    }

    fn as_mut_slices(&mut self) -> (&mut [I], &mut [I]) {
        let (left, right) = self.slice_ranges();
        let (left_left, left_right) = self.buf.split_at_mut(left.end);
        let (_, right_right) = left_right.split_at_mut(right.start);
        (left_left, right_right)
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    pub fn free(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> DequeRefIter<'_, I> {
        let (first, second) = self.as_slices();
        DequeRefIter {
            first: first.iter(), second: second.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> DequeMutRefIter<'_, I> {
        let (first, second) = self.as_mut_slices();
        DequeMutRefIter {
            first: first.iter_mut(), second: second.iter_mut(),
        }
    }
}

impl<I, const L: usize>
Deque<I, L>
where I: Copy {
    pub fn push(&mut self, item: I) {
        if self.is_full() {
            return;
        }
        self.buf[self.tail.pos()] = item;
        self.tail.next();
        if self.tail.pos() == self.head.pos() {
            self.full = true;
        }
    }

    pub fn pop(&mut self) -> Option<I> {
        if self.is_empty() {
            return None;
        }
        let item = self.buf[self.head.pos()];
        if self.head.pos() == self.tail.pos() {
            self.full = false;
        }
        self.head.next();
        Some(item)
    }
}

impl<'a, I, const L: usize>
IntoIterator for &'a Deque<I, L> {
    type Item = &'a I;
    type IntoIter = DequeRefIter<'a, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<I, const LEN: usize>
IntoIterator for Deque<I, LEN>
where I: Copy {
    type Item = I;
    type IntoIter = DequeIter<I, LEN>;

    fn into_iter(self) -> Self::IntoIter {
        DequeIter {
            deque: self,
        }
    }
}

impl<I, const L: usize>
FromIterator<I> for Deque<I, L>
where I: Copy + Default {
    fn from_iter<IntoIter: IntoIterator<Item=I>>(other: IntoIter) -> Self {
        let mut deque = Deque::default();
        for item in other {
            if deque.is_full() {
                deque.pop();
            }
            deque.push(item);
        }
        deque
    }
}

/*
 * owned into iterator
 */
pub struct DequeIter<I, const L: usize> {
    deque: Deque<I, L>,
}

impl<I, const LEN: usize>
DequeIter<I, LEN> {
    pub(crate) fn head(&self) -> usize {
        self.deque.head()
    }

    pub(crate) fn tail(&self) -> usize {
        self.deque.tail()
    }
}

impl<I, const L: usize>
Iterator for DequeIter<I, L>
where I: Copy {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.deque.len(), Some(self.deque.len()))
    }
}

impl<I, const L: usize>
DoubleEndedIterator for DequeIter<I, L>
where I: Copy {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.deque.head.pos() == self.deque.tail.pos() {
            return None;
        }
        self.deque.tail.prev();
        Some(self.deque.buf[self.deque.tail.pos()])
    }
}

impl<I, const L: usize>
ExactSizeIterator for DequeIter<I, L>
where I: Copy { }


impl<I, const L: usize>
FusedIterator for DequeIter<I, L>
where I: Copy { }

/*
 * reference iterator
 */
pub struct DequeRefIter<'a, I> {
    first: Iter<'a, I>,
    second: Iter<'a, I>,
}

impl<'a, I>
Iterator for DequeRefIter<'a, I> {
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.first.next() {
            return Some(item);
        }
        self.second.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.first.len() + self.second.len(), Some(self.first.len() + self.second.len()))
    }
}

/*
impl<'a, I, const LEN: usize> DoubleEndedIterator for DequeRefIter<'a, I, LEN> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.prev() else {
            return None;
        };
        Some(&self.deque.buf[idx])
    }
}
*/

impl<'a, I>
ExactSizeIterator for DequeRefIter<'a, I> { }

impl<'a, I>
FusedIterator for DequeRefIter<'a, I> { }

pub struct DequeMutRefIter<'a, I> {
    first: IterMut<'a, I>,
    second: IterMut<'a, I>,
}

impl<'a, I>
Iterator for DequeMutRefIter<'a, I> {
    type Item = &'a mut I;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.first.next() {
            return Some(item);
        }
        self.second.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.first.len() + self.second.len(), Some(self.first.len() + self.second.len()))
    }
}

/*
impl<'a, I, const LEN: usize> DoubleEndedIterator for DequeMutRefIter<'a, I, LEN> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.prev() else {
            return None;
        };
        Some(&mut self.deque.buf[idx])
    }
}
*/

impl<'a, I>
ExactSizeIterator for DequeMutRefIter<'a, I> { }

impl<'a, I>
FusedIterator for DequeMutRefIter<'a, I> { }
