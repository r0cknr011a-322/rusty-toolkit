use core::array::{ from_fn };

#[derive(Clone, Copy)]
struct Cursor {
    pos: usize,
    max: usize,
    inc: bool,
}

impl Cursor {
    fn new(pos: usize, len: usize, inc: bool) -> Self {
        Self {
            pos: pos, max: len - 1, inc: inc,
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn next(&mut self) {
        self.pos = match self.inc {
            true => match self.pos == self.max {
                true => 0,
                false => self.pos + 1,
            },
            false => match self.pos == 0 {
                true => self.max,
                false => self.pos - 1,
            },
        };
    }

    fn prev(&mut self) {
        self.pos = match self.inc {
            true => match self.pos == 0 {
                true => self.max,
                false => self.pos - 1,
            },
            false => match self.pos == self.max {
                true => 0,
                false => self.pos + 1,
            },
        };
    }
}

#[derive(PartialEq, Debug)]
pub enum DequeError {
    Fatal,
}

#[derive(Clone, Copy)]
struct DequeCursor {
    cursor: Cursor,
    free: bool,
}

impl DequeCursor {
    fn new(cursor: Cursor) -> Self {
        Self {
            cursor: cursor, free: true,
        }
    }

    fn pos(&self) -> usize {
        self.cursor.pos()
    }

    fn is_free(&self) -> bool {
        self.free
    }

    fn set_free(&mut self, free: bool) {
        self.free = free;
    }

    fn next(&mut self) {
        self.cursor.next();
    }

    fn prev(&mut self) {
        self.cursor.prev();
    }
}

#[derive(Clone, Copy)]
pub struct Deque<I, const L: usize> {
    buf: [I; L],
    front: DequeCursor,
    back: DequeCursor,
}

impl<I, const LEN: usize> Default for Deque<I, LEN>
where I: Default + Copy {
    fn default() -> Self {
        Self {
            buf: [I::default(); LEN],
            front: DequeCursor::new(Cursor::new(0, LEN, true)),
            back: DequeCursor::new(Cursor::new(LEN - 1, LEN, false)),
        }
    }
}

impl<I, const LEN: usize> Deque<I, LEN>
where I: Copy {
    pub fn new<F: FnMut(usize) -> I>(f: F) -> Self {
        Self {
            buf: from_fn(f),
            front: DequeCursor::new(Cursor::new(0, LEN, true)),
            back: DequeCursor::new(Cursor::new(LEN - 1, LEN, false)),
        }
    }

    pub(crate) fn get_front(&self) -> (usize, bool) {
        (self.front.pos(), self.front.is_free())
    }

    pub(crate) fn get_back(&self) -> (usize, bool) {
        (self.back.pos(), self.back.is_free())
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn len(&self) -> usize {
        if !self.front.is_free() && !self.back.is_free() {
            return LEN;
        }
        let mut sum = 0;
        if !self.front.is_free() {
            sum += 1;
        };
        if !self.back.is_free() {
            sum += 1;
        };
        sum += if self.front.pos() < self.back.pos() {
            self.front.pos() + (LEN - 1 - self.back.pos())
        } else {
            self.front.pos() - self.back.pos() - 1
        };
        sum
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    pub fn push_back(&mut self, item: I) -> Result<(), DequeError> {
        let len = self.len();
        if len == LEN {
            return Err(DequeError::Fatal);
        }
        if len == LEN - 1 {
            if self.back.is_free() {
                self.back.set_free(false);
            } else {
                self.front.prev();
                self.front.set_free(false);
                self.back.next();
            }
            self.buf[self.back.pos()] = item;
            return Ok(());
        }
        if len == LEN - 2 {
           self.buf[self.back.pos()] = item;
           self.back.set_free(false);
           return Ok(());
        }
        self.buf[self.back.pos()] = item;
        self.back.next();
        Ok(())
    }

    pub fn push_front(&mut self, item: I) -> Result<(), DequeError> {
        let len = self.len();
        if len == LEN {
            return Err(DequeError::Fatal);
        }
        if len == LEN - 1 {
            if self.front.is_free() {
                self.front.set_free(false);
            } else {
                self.back.prev();
                self.back.set_free(false);
                self.front.next();
            }
            self.buf[self.front.pos()] = item;
            return Ok(());
        }
        if len == LEN - 2 {
           self.buf[self.front.pos()] = item;
           self.front.set_free(false);
           return Ok(());
        }
        self.buf[self.front.pos()] = item;
        self.front.next();
        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<I> {
        let len = self.len();
        if len == 0 {
            return None;
        }
        if len == LEN {
            self.back.set_free(true);
            return Some(self.buf[self.back.pos()]);
        }
        if len == LEN - 1 {
            if self.front.is_free() {
                self.back.set_free(true);
                return Some(self.buf[self.back.pos()]);
            }
            if self.back.is_free() {
                self.front.next();
                self.back.prev();
                self.front.set_free(true);
                return Some(self.buf[self.back.pos()]);
            }
        }
        self.back.prev();
        Some(self.buf[self.back.pos()])
    }

    pub fn pop_front(&mut self) -> Option<I> {
        let len = self.len();
        if len == 0 {
            return None;
        }
        if len == LEN {
            self.front.set_free(true);
            return Some(self.buf[self.front.pos()]);
        }
        if len == LEN - 1 {
            if !self.front.is_free() {
                self.front.set_free(true);
                return Some(self.buf[self.front.pos()]);
            }
            if !self.back.is_free() {
                self.back.next();
                self.front.prev();
                self.back.set_free(true);
                return Some(self.buf[self.front.pos()]);
            }
        }
        self.front.prev();
        Some(self.buf[self.front.pos()])
    }

    fn iter(&self) -> Iter {
        let mut front = Cursor::new(self.front.pos(), LEN, false);
        let mut back = Cursor::new(self.back.pos(), LEN, true);
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
}

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

/*
 * owning iterator
 */
impl<I, const LEN: usize> IntoIterator for Deque<I, LEN>
where I: Copy + Default {
    type Item = I;
    type IntoIter = DequeIter<I, LEN>;

    fn into_iter(self) -> Self::IntoIter {
        DequeIter {
            iter: self.iter(),
            deque: self,
        }
    }
}

impl<I, const L: usize> FromIterator<I> for Deque<I, L>
where I: Copy {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut deque = Self::default();
        for e in iter {
            deque.push_back(e);
        }
        deque
    }
}

pub struct DequeIter<I, const LEN: usize> {
    deque: Deque<I, LEN>,
    iter: Iter,
}

impl<I, const LEN: usize> DequeIter<I, LEN>
where I: Copy {
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

impl<I, const L: usize> ExactSizeIterator for DequeIter<I, L> { }

/*
 * reference iterator
 */
impl<'a, I, const L: usize> IntoIterator for &'a Deque<I, L>
where I: Copy + Default {
    type Item = &'a I;
    type IntoIter = DequeRefIter<'a, I, L>;

    fn into_iter(self) -> Self::IntoIter {
        DequeRefIter {
            iter: self.iter(),
            deque: &self,
        }
    }
}

pub struct DequeRefIter<'a, I, const LEN: usize>
where I: Copy + Default {
    deque: &'a Deque<I, LEN>,
    iter: Iter,
}

impl<'a, I, const LEN: usize> ExactSizeIterator for DequeRefIter<'a, I, LEN>
where I: Copy + Default { }

impl<'a, I, const LEN: usize> Iterator for DequeRefIter<'a, I, LEN>
where I: Copy + Default {
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

impl<'a, I, const LEN: usize> DoubleEndedIterator for DequeRefIter<'a, I, LEN>
where I: Copy + Default {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.prev() else {
            return None;
        };
        Some(&self.deque.buf[idx])
    }
}
