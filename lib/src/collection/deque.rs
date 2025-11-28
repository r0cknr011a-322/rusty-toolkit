#[derive(Clone, Copy)]
struct Cursor<const SIZE: usize> {
    pos: usize,
    empty: bool,
    straight: bool,
}

impl<const SIZE: usize> Cursor<SIZE> {
    fn new(pos: usize, straight: bool) -> Self {
        Self {
            pos: pos,
            straight: straight,
            empty: true,
        }
    }

    fn empty(&self) -> bool {
        self.empty
    }

    fn set_empty(&mut self, value: bool) {
        self.empty = value;
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn next(&mut self) {
        self.pos = match self.straight {
            true => match self.pos == SIZE - 1 {
                true => 0,
                false => self.pos + 1,
            },
            false => match self.pos == 0 {
                true => SIZE - 1,
                false => self.pos - 1,
            },
        };
    }

    fn prev(&mut self) {
        self.pos = match self.straight {
            true => match self.pos == 0 {
                true => SIZE - 1,
                false => self.pos - 1,
            },
            false => match self.pos == SIZE - 1 {
                true => 0,
                false => self.pos + 1,
            },
        };
    }
}

#[derive(PartialEq, Debug)]
pub enum UhvDequeError {
    Fatal,
}

#[derive(Clone, Copy)]
pub struct UhvDeque<E, const SIZE: usize>
where E: Copy + Default {
    buf: [E; SIZE],
    front: Cursor<SIZE>,
    back: Cursor<SIZE>,
}

impl<E, const SIZE: usize> UhvDeque<E, SIZE>
where E: Copy + Default {
    pub fn new() -> Self {
        Self {
            buf: [E::default(); SIZE],
            front: Cursor::new(0, true), back: Cursor::new(SIZE - 1, false),
        }
    }

    pub(crate) fn get_front(&self) -> (usize, bool) {
        (self.front.pos(), self.front.empty())
    }

    pub(crate) fn get_back(&self) -> (usize, bool) {
        (self.back.pos(), self.back.empty())
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn len(&self) -> usize {
        let mut sum = 0;
        if !self.front.empty() && !self.back.empty() {
            return SIZE;
        }
        if !self.front.empty() || !self.back.empty() {
            sum += 1;
        }
        sum += if self.front.pos() < self.back.pos() {
            self.front.pos() + (SIZE - 1 - self.back.pos())
        } else {
            self.front.pos() - self.back.pos() - 1
        };
        sum
    }

    pub fn push_back(&mut self, entry: E) -> Result<(), UhvDequeError> {
        let len = self.len();
        if len == SIZE {
            return Err(UhvDequeError::Fatal);
        }
        if len == SIZE - 1 {
            if self.back.empty() {
                self.back.set_empty(false);
            } else {
                self.front.prev();
                self.front.set_empty(false);
                self.back.next();
            }
            self.buf[self.back.pos()] = entry;
            return Ok(());
        }
        if len == SIZE - 2 {
           self.buf[self.back.pos()] = entry;
           self.back.set_empty(false);
           return Ok(());
        }
        self.buf[self.back.pos()] = entry;
        self.back.next();
        Ok(())
    }

    pub fn push_front(&mut self, entry: E) -> Result<(), UhvDequeError> {
        let len = self.len();
        if len == SIZE {
            return Err(UhvDequeError::Fatal);
        }
        if len == SIZE - 1 {
            if self.front.empty() {
                self.front.set_empty(false);
            } else {
                self.back.prev();
                self.back.set_empty(false);
                self.front.next();
            }
            self.buf[self.front.pos()] = entry;
            return Ok(())
        }
        if len == SIZE - 2 {
           self.buf[self.front.pos()] = entry;
           self.front.set_empty(false);
           return Ok(());
        }
        self.buf[self.front.pos()] = entry;
        self.front.next();
        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<E> {
        let len = self.len();
        if len == 0 {
            return None;
        }
        if len == SIZE {
            self.back.set_empty(true);
            return Some(self.buf[self.back.pos()]);
        }
        if len == SIZE - 1 {
            if !self.back.empty() {
                self.back.set_empty(true);
                return Some(self.buf[self.back.pos()]);
            }
            if !self.front.empty() {
                self.front.next();
                self.back.prev();
                self.front.set_empty(true);
                return Some(self.buf[self.back.pos()]);
            }
        }
        self.back.prev();
        Some(self.buf[self.back.pos()])
    }

    pub fn pop_front(&mut self) -> Option<E> {
        let len = self.len();
        if len == 0 {
            return None;
        }
        if len == SIZE {
            self.front.set_empty(true);
            return Some(self.buf[self.front.pos()]);
        }
        if len == SIZE - 1 {
            if !self.front.empty() {
                self.front.set_empty(true);
                return Some(self.buf[self.front.pos()]);
            }
            if !self.back.empty() {
                self.back.next();
                self.front.prev();
                self.back.set_empty(true);
                return Some(self.buf[self.front.pos()]);
            }
        }
        self.front.prev();
        Some(self.buf[self.front.pos()])
    }

    fn iter(&self) -> Iter<SIZE> {
        let mut front = Cursor::new(self.front.pos(), false);
        let mut back = Cursor::new(self.back.pos(), true);
        let mut end = false;

        let len = self.len();
        if len == 0 {
            end = true;
            front.next();
        } else if len >= 1 && len < SIZE - 1 {
            front.next();
            back.next();
        } else if len == SIZE - 1 {
            if self.front.empty() {
                front.next();
            }
            if self.back.empty() {
                back.next();
            }
        }

        Iter {
            front: front,
            back: back,
            end: end,
        }
    }
}

struct Iter<const SIZE: usize> {
    front: Cursor<SIZE>,
    back: Cursor<SIZE>,
    end: bool,
}

impl<const SIZE: usize> Iter<SIZE> {
    fn get_front(&self) -> (usize, bool) {
        (self.front.pos(), self.front.empty())
    }

    fn get_back(&self) -> (usize, bool) {
        (self.back.pos(), self.back.empty())
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
        self.front.pos() + (SIZE - 1 - self.front.pos()) + 1
    }

    fn next(&mut self) -> Option<usize> {
        if self.end {
            return None;
        }
        let len = self.len();
        if len == 1 {
            self.front.set_empty(true);
            self.back.set_empty(true);
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
            self.front.set_empty(true);
            self.back.set_empty(true);
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
impl<E, const SIZE: usize> IntoIterator for UhvDeque<E, SIZE>
where E: Copy + Default {
    type Item = E;
    type IntoIter = UhvDequeIter<E, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        UhvDequeIter {
            iter: self.iter(),
            deque: self,
        }
    }
}

pub struct UhvDequeIter<E, const SIZE: usize>
where E: Copy + Default {
    deque: UhvDeque<E, SIZE>,
    iter: Iter<SIZE>,
}

impl<E, const SIZE: usize> UhvDequeIter<E, SIZE>
where E: Copy + Default {
    pub(crate) fn get_front(&self) -> (usize, bool) {
        self.iter.get_front()
    }

    pub(crate) fn get_back(&self) -> (usize, bool) {
        self.iter.get_back()
    }
}

impl<E, const SIZE: usize> Iterator for UhvDequeIter<E, SIZE>
where E: Copy + Default {
    type Item = E;

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

impl<E, const SIZE: usize> DoubleEndedIterator for UhvDequeIter<E, SIZE>
where E: Copy + Default {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.prev() else {
            return None;
        };
        Some(self.deque.buf[idx])
    }
}

impl<E, const SIZE: usize> ExactSizeIterator for UhvDequeIter<E, SIZE>
where E: Copy + Default { }

/*
 * reference iterator
 */
impl<'a, E, const SIZE: usize> IntoIterator for &'a UhvDeque<E, SIZE>
where E: Copy + Default {
    type Item = &'a E;
    type IntoIter = UhvDequeRefIter<'a, E, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        UhvDequeRefIter {
            iter: self.iter(),
            deque: &self,
        }
    }
}

pub struct UhvDequeRefIter<'a, E, const SIZE: usize>
where E: Copy + Default {
    deque: &'a UhvDeque<E, SIZE>,
    iter: Iter<SIZE>,
}

impl<'a, E, const SIZE: usize> ExactSizeIterator for UhvDequeRefIter<'a, E, SIZE>
where E: Copy + Default { }

impl<'a, E, const SIZE: usize> Iterator for UhvDequeRefIter<'a, E, SIZE>
where E: Copy + Default {
    type Item = &'a E;

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

impl<'a, E, const SIZE: usize> DoubleEndedIterator for UhvDequeRefIter<'a, E, SIZE>
where E: Copy + Default {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.prev() else {
            return None;
        };
        Some(&self.deque.buf[idx])
    }
}
