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

    fn wr_log(&self, data: &[u8], idx: usize) {
        let mut borrow = self.logbuf.borrow_mut();
        let logbuf = &mut borrow[idx];

        if data.len() > logbuf.free() {
            panic!("logger buffer (channel: {idx}) overflow");
        }

        let (bufl, bufr) = logbuf.as_mut_slices();
        if data.len() <= bufl.len() {
            let (tocopy, _) = bufl.split_at_mut(data.len());
            tocopy.copy_from_slice(data);
            return;
        }

        let (datal, datar) = data.split_at(bufl.len());
        bufl.copy_from_slice(datal);

        let (tocopy, _) = bufr.split_at_mut(datar.len());
        tocopy.copy_from_slice(datar);
    }

    pub fn rd_log(&self, data: &mut [u8], idx: usize) -> usize {
        let mut borrow = self.logbuf.borrow_mut();
        let logbuf = &mut borrow[idx];

        let len = logbuf.len();
        let (bufl, bufr) = logbuf.as_mut_slices();

        if data.len() > len {
            let (datal, datar) = data.split_at_mut(bufl.len());
            datal.copy_from_slice(bufl);
            let (tocopy, _) = datar.split_at_mut(bufr.len());
            tocopy.copy_from_slice(bufr);
            return len;
        }

        if data.len() < bufl.len() {
            let (fromcopy, _) = bufl.split_at(data.len());
            data.copy_from_slice(fromcopy);
            return data.len();
        }

        let (datal, datar) = data.split_at_mut(bufl.len());
        datal.copy_from_slice(bufl);
        let (fromcopy, _) = bufr.split_at(datar.len());
        datar.copy_from_slice(fromcopy);

        data.len()
    }
}

impl<T, const NR: usize, const L: usize>
RuntimeInner<T, NR, L>
where T: Timer {
    pub fn time(&self) -> Duration {
        self.timer.borrow_mut().time()
    }

    pub fn chan(&self, idx: usize) -> Option<impl Runtime> {
        let Some(_) = self.logbuf.borrow().get(idx) else {
            return None;
        };
        Some(RuntimeRef::new(idx, self))
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
        self.inner.wr_log(data.as_bytes(), self.logidx);
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
