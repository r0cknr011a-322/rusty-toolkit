use core::fmt::{ self };
use core::time::{ Duration };

use crate::collection::byteblock::{ ByteBlock };
use crate::collection::asynque::{ Asynque };
use crate::runtime::{ Timer, Runtime, RuntimeInner, LogErr };


#[derive(Clone, Copy)]
pub struct RuntimeRef<'a, T, Q, const NR: usize, const B: usize, const L: usize>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    logidx: usize,
    inner: &'a RuntimeInner<T, Q, NR, B, L>,
}

impl<'a, T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeRef<'a, T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    pub fn new(logidx: usize, inner: &'a RuntimeInner<T, Q, NR, B, L>) -> Self {
        Self {
            logidx: logidx, inner: inner,
        }
    }
}

impl<'a, T, Q, const NR: usize, const B: usize, const L: usize>
fmt::Write for RuntimeRef<'a, T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    fn write_str(&mut self, data: &str) -> Result<(), fmt::Error> {
        self.inner.log(data.as_bytes(), self.logidx);
        Ok(())
    }
}

impl<'a, T, Q, const NR: usize, const B: usize, const L: usize>
Timer for RuntimeRef<'a, T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    fn time(&mut self) -> Duration {
        self.inner.time()
    }
}

impl<'a, T, Q, const NR: usize, const B: usize, const L: usize>
Runtime for RuntimeRef<'a, T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {

}
