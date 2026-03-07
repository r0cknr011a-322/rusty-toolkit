#[cfg(test)]
mod test;

use core::fmt::{ self };
use core::cell::{ RefCell };
use core::time::{ Duration };
use core::array::{ self };

use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };


pub trait Timer {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Timer + fmt::Write {

}


pub struct RuntimeInner<T, const NR: usize, const L: usize> {
    timer: RefCell<T>,
    logbuf: RefCell<[Deque<u8, L>; NR]>,
}

impl<T, const NR: usize, const L: usize>
RuntimeInner<T, NR, L> {
    pub fn new(timer: T) -> Self {
        Self {
            timer: RefCell::new(timer),
            logbuf: RefCell::new(array::from_fn(|_| { Deque::default() })),
        }
    }

    pub fn wr_log(&self, idx: usize, data: &[u8]) -> usize {
        let mut borrow = self.logbuf.borrow_mut();
        let Some(logbuf) = borrow.get_mut(idx) else {
            return 0;
        };

        let mut buf = data;
        if data.len() > logbuf.free() {
            buf = buf.split_at(logbuf.free()).0;
        }

        for b in buf {
            logbuf.push(*b);
        }

        buf.len()
    }

    pub fn rd_log(&self, idx: usize, data: &mut [u8]) -> usize {
        let mut borrow = self.logbuf.borrow_mut();
        let Some(logbuf) = borrow.get_mut(idx) else {
            return 0;
        };

        let len = logbuf.len();
        let (bufl, bufr) = logbuf.as_slices();

        let mut cnt = 0;
        if data.len() > len {
            let (datal, datar) = data.split_at_mut(bufl.len());
            datal.copy_from_slice(bufl);
            let (tocopy, _) = datar.split_at_mut(bufr.len());
            tocopy.copy_from_slice(bufr);
            cnt = len;
        } else if data.len() > bufl.len() {
            let (datal, datar) = data.split_at_mut(bufl.len());
            datal.copy_from_slice(bufl);
            let (fromcopy, _) = bufr.split_at(datar.len());
            datar.copy_from_slice(fromcopy);
            cnt = data.len();
        } else {
            let (fromcopy, _) = bufl.split_at(data.len());
            data.copy_from_slice(fromcopy);
            cnt = data.len();
        }

        for _ in 0..cnt {
            let _ = logbuf.pop();
        }

        cnt
    }

    pub fn chan(&self, idx: usize) -> Option<RuntimeRef<T, NR, L>> {
        let borrow = self.logbuf.borrow();
        let Some(_) = borrow.get(idx) else {
            return None;
        };
        Some(RuntimeRef::new(idx, self))
    }

    pub fn chan_capacity(&self, idx: usize) -> Option<usize> {
        let borrow = self.logbuf.borrow();
        let Some(buf) = borrow.get(idx) else {
            return None;
        };
        Some(buf.capacity())
    }

    pub fn chan_len(&self, idx: usize) -> Option<usize> {
        let borrow = self.logbuf.borrow();
        let Some(buf) = borrow.get(idx) else {
            return None;
        };
        Some(buf.len())
    }

    pub fn chan_free(&self, idx: usize) -> Option<usize> {
        let borrow = self.logbuf.borrow();
        let Some(buf) = borrow.get(idx) else {
            return None;
        };
        Some(buf.free())
    }
}

impl<T, const NR: usize, const L: usize>
RuntimeInner<T, NR, L>
where T: Timer {
    pub fn time(&self) -> Duration {
        self.timer.borrow_mut().time()
    }
}


#[derive(Clone, Copy)]
pub struct RuntimeRef<'a, T, const NR: usize, const L: usize> {
    logidx: usize,
    inner: &'a RuntimeInner<T, NR, L>,
}

impl<'a, T, const NR: usize, const L: usize>
RuntimeRef<'a, T, NR, L> {
    pub fn new(logidx: usize, inner: &'a RuntimeInner<T, NR, L>) -> Self {
        Self {
            logidx: logidx, inner: inner,
        }
    }
}

impl<'a, T, const NR: usize, const L: usize>
fmt::Write for RuntimeRef<'a, T, NR, L> {
    fn write_str(&mut self, data: &str) -> Result<(), fmt::Error> {
        self.inner.wr_log(self.logidx, data.as_bytes());
        Ok(())
    }
}

impl<'a, T, const NR: usize, const L: usize>
Timer for RuntimeRef<'a, T, NR, L>
where T: Timer {
    fn time(&mut self) -> Duration {
        self.inner.time()
    }
}

impl<'a, T, const NR: usize, const L: usize>
Runtime for RuntimeRef<'a, T, NR, L>
where T: Timer {

}
