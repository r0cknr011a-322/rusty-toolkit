use core::fmt::{ self, Write };
use crate::io::{ AsyncWrite };

/*
pub struct Logger<W> {
    out: W,
    pos: usize,
}

impl<W> Logger<W> {
    fn new(out: W) -> Self {
        Self {
            out: out, pos: 0,
        }
    }
}

impl<W: AsyncWrite> Write for Logger<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let free = self.deque.capacity() - self.deque.len();
        if free < s.len() {
            for _ in 0..s.len() {
                self.deque.pop_front();
            }
        }
        for c in s.as_bytes() {
            self.deque.push_back(*c);
        }
        Ok(())
    }
}

impl<const L: usize> IntoIterator for LogBuf<L> {
    type Item = u8;
    type IntoIter = LogBufIter<L>;

    fn into_iter(self) -> Self::IntoIter {
        LogBufIter::new(self.deque.into_iter())
    }
}

pub struct LogBufIter<const LEN: usize> {
    iter: DequeIter<u8, LEN>,
}

impl<const L: usize> LogBufIter<L> {
    fn new(iter: DequeIter<u8, L>) -> Self {
        Self {
            iter: iter,
        }
    }
}

impl<const L: usize> Iterator for LogBufIter<L> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.iter.len(), Some(self.iter.len()))
    }
}

impl<const L: usize> ExactSizeIterator for LogBufIter<L> { }
*/

/*
impl LogBuf {
    pub fn new() -> Self {
        Self { buf: [0; 32], pos: 0 }
    }

    pub fn get_data(&mut self) -> &[u8] {
        self.pos = 0;
        &self.buf
    }

    pub fn is_ready(&self) -> bool {
        self.buf.len() - self.pos == 0
    }
}

impl fmt::Write for LogBuf {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        let cnt = cmp::min(self.buf.len() - self.pos, data.len());
        let dst = &mut self.buf[self.pos..self.pos + cnt];
        let src = &data.as_bytes()[0..cnt];
        dst.copy_from_slice(src);
        self.pos += cnt;
        Ok(())
    }
}

pub struct LogCell<'a, Logger>
where Logger: fmt::Write {
    logcell: &'a RefCell<Logger>,
}

impl<'a, Logger> LogCell<'a, Logger>
where Logger: fmt::Write {
    pub fn new(logcell: &'a RefCell<Logger>) -> Self {
        Self { logcell }
    }
}

impl<Logger> Clone for LogCell<'_, Logger>
where Logger: fmt::Write {
    fn clone(&self) -> Self {
        Self { logcell: self.logcell }
    }
}

impl<Logger> Copy for LogCell<'_, Logger>
where Logger: fmt::Write {
    
}

impl<Logger> fmt::Write for LogCell<'_, Logger>
where Logger: fmt::Write {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        let mut logger = self.logcell.borrow_mut();
        logger.write_str(data)
    }
}
*/
