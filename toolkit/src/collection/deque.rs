use core::array::{ from_fn };
use core::ops::{ Range };
use core::slice::{ Iter, IterMut };
use core::iter::{ FusedIterator };
use crate::collection::cursor::{ Cursor, Dir };

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
            lhead.prev();
            rhead.prev();
            if self.buf[lhead.pos()] != other.buf[rhead.pos()] {
                return false;
            }
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
        let (first, second) = self.slice_ranges();
        (&self.buf[first], &self.buf[second])
    }

    fn as_mut_slices(&mut self) -> (&mut [I], &mut [I]) {
        let (left, right) = self.slice_ranges();
        let (left_left, left_right) = self.buf.split_at_mut(left.end);
        let (right_left, right_right) = left_right.split_at_mut(right.start);
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

    /*
    fn iterator(&self) -> Iter {
        let mut head = Cursor::new(self.head.pos());
        let mut tail = Cursor::new(self.tail.pos());
        let mut end = false;

        let len = self.len();
        if len == 0 {
            end = true;
            front.next();
        } else if len >= 1 && len < LEN - 1 {
            front.next();
            back.next();
        } else if len == LEN - 1 {
            if self.front.is_free() {
                front.next();
            }
            if self.back.is_free() {
                back.next();
            }
        }

        Iter::new(front, back, LEN, end)
    }
    */

    /*
    pub fn iter(&self) -> DequeRefIter<I, LEN> {
        let (first, second) = self.as_slices();
        DequeRefIter {
            first: first.iter(), second: second.iter(),
        }
    }
    */

    pub fn iter_mut(&mut self) -> DequeMutRefIter<I> {
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
struct Iter {
    front: Cursor,
    back: Cursor,
    max: usize,
    end: bool,
}

impl Iter {
    fn new(front: Cursor, back: Cursor, len: usize, end: bool) -> Self {
        Self {
            front: front, back: back, max: len - 1, end: end,
        }
    }

    fn get_front(&self) -> usize {
        self.front.pos()
    }

    fn get_back(&self) -> usize {
        self.back.pos()
    }

    fn len(&self) -> usize {
        if self.end {
            return 0;
        }
        if self.back.pos() == self.front.pos() {
            return 1;
        }
        if self.back.pos() < self.front.pos() {
            return self.front.pos() - self.back.pos() + 1;
        }
        self.front.pos() + (self.max - self.front.pos()) + 1
    }

    fn next(&mut self) -> Option<usize> {
        if self.end {
            return None;
        }
        let len = self.len();
        if len == 1 {
            self.end = true;
            return Some(self.front.pos());
        }
        let res = self.front.pos();
        self.front.next();
        Some(res)
    }

    fn prev(&mut self) -> Option<usize> {
        if self.end {
            return None;
        }
        let len = self.len();
        if len == 1 {
            self.end = true;
            return Some(self.back.pos());
        }
        let res = self.back.pos();
        self.back.next();
        Some(res)
    }
}
*/

/*
 * iterator
 */
/*
pub struct DequeIter<I, const LEN: usize> {
    deque: Deque<I, LEN>,
    iter: Iter,
}

impl<I: Copy, const LEN: usize> DequeIter<I, LEN> {
    pub(crate) fn get_front(&self) -> usize {
        self.iter.get_front()
    }

    pub(crate) fn get_back(&self) -> usize {
        self.iter.get_back()
    }
}

impl<I: Copy, const L: usize> Iterator for DequeIter<I, L> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.next() else {
            return None;
        };
        Some(self.deque.buf[idx])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.iter.len(), Some(self.iter.len()))
    }
}

impl<I: Copy, const L: usize> DoubleEndedIterator for DequeIter<I, L> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.prev() else {
            return None;
        };
        Some(self.deque.buf[idx])
    }
}

impl<I: Copy, const L: usize> ExactSizeIterator for DequeIter<I, L> { }
*/

/*
 * owned into iterator
 */
/*
impl<I: Copy, const LEN: usize> IntoIterator for Deque<I, LEN> {
    type Item = I;
    type IntoIter = DequeIter<I, LEN>;

    fn into_iter(self) -> Self::IntoIter {
        DequeIter {
            iter: self.iterator(),
            deque: self,
        }
    }
}
*/

/*
 * reference into iterator
 */
/*
impl<'a, I, const L: usize> IntoIterator for &'a Deque<I, L> {
    type Item = &'a I;
    type IntoIter = DequeRefIter<'a, I, L>;

    fn into_iter(self) -> Self::IntoIter {
        DequeRefIter {
            iter: self.iterator(),
            deque: &self,
        }
    }
}
*/

pub struct DequeRefIter<'a, I, const LEN: usize> {
    // deque: &'a Deque<I, LEN>,
    // iter: Iter,
    first: Iter<'a, I>,
    second: Iter<'a, I>,
}

/*
impl<'a, I, const LEN: usize>
ExactSizeIterator for DequeRefIter<'a, I, LEN> { }

impl<'a, I, const LEN: usize> FusedIterator for DequeRefIter<'a, I, LEN> { }

impl<'a, I, const LEN: usize> Iterator for DequeRefIter<'a, I, LEN> {
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.next() else {
            return None;
        };
        Some(&self.deque.buf[idx])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.iter.len(), Some(self.iter.len()))
    }
}

impl<'a, I, const LEN: usize> DoubleEndedIterator for DequeRefIter<'a, I, LEN> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.prev() else {
            return None;
        };
        Some(&self.deque.buf[idx])
    }
}
*/

pub struct DequeMutRefIter<'a, I> {
    first: IterMut<'a, I>,
    second: IterMut<'a, I>,
}

/*
impl<'a, I, const LEN: usize> ExactSizeIterator for DequeMutRefIter<'a, I, LEN> { }

impl<'a, I, const LEN: usize> FusedIterator for DequeMutRefIter<'a, I, LEN> { }

impl<'a, I, const LEN: usize> Iterator for DequeMutRefIter<'a, I, LEN> {
    type Item = &'a mut I;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.next() else {
            return None;
        };
        Some(&mut self.deque.buf[idx])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.iter.len(), Some(self.iter.len()))
    }
}

impl<'a, I, const LEN: usize> DoubleEndedIterator for DequeMutRefIter<'a, I, LEN> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.prev() else {
            return None;
        };
        Some(&mut self.deque.buf[idx])
    }
}
*/
