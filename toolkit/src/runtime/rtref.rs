use core::fmt::{ self };
use core::time::{ Duration };

use crate::runtime::{ Timer, Runtime, RuntimeInner };


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
        self.inner.log(data.as_bytes(), self.logidx);
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
