use core::fmt::{ self, Write };
use core::cell::{ Cell };
use core::time::{ Duration };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };
use crate::cmd::{ Queue };
use toolkit_unsafe::{ IPCByteBuf };

pub trait Time {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Time {
    fn logbuf(&mut self) -> impl Iterator<Item=&mut impl Write>;
    fn ipcbuf(&mut self) -> impl Iterator<Item=&mut IPCByteBuf>;
}

/*
 * runtime reference
 */
#[derive(Clone, Copy)]
struct RuntimeRef<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize> {
    rt: &'a RuntimeMain<'a, T, Q, BUFNR, CHL, CHNR>,
}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
Time for RuntimeRef<'a, T, Q, BUFNR, CHL, CHNR>
where T: Time {
    fn time(&mut self) -> Duration {
        let mut timer = self.rt.timer.take().unwrap();
        let time = timer.time();
        self.rt.timer.set(Some(timer));
        time
    }
}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
Runtime for RuntimeRef<'a, T, Q, BUFNR, CHL, CHNR>
where T: Time {
    fn logbuf(&self) -> impl Iterator<Item=&mut impl Write> {
        let mut buf = self.rt.logbufbuf.take();
        let logbuf = buf.iter_mut();
        self.rt.logbufbuf.set(buf);
        logbuf
    }
}

/*
 * runtime
 */
pub struct RuntimeMain<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize> {
    timer: Cell<Option<T>>,
    queue: Cell<Q>,
    logbufbuf: Cell<LogBufBuf<CHL, CHNR>>,
    ipcbufbuf: Cell<Deque<IPCByteBuf<'a>, BUFNR>>,
}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
RuntimeMain<'a, T, Q, BUFNR, CHL, CHNR> {
    pub fn new<B: FnMut(usize) -> IPCByteBuf<'a>>(timer: T, queue: Q, mut bufctr: B) -> Self {
        Self {
            timer: Cell::new(Some(timer)), queue: Cell::new(queue),
            ipcbufbuf: Cell::new(Deque::new(|idx| bufctr(idx))),
            logbufbuf: Cell::new(LogBufBuf::default()),
        }
    }
}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
RuntimeMain<'a, T, Q, BUFNR, CHL, CHNR>
where T: Time {
    pub fn as_ref(&'a self) -> impl Runtime {
        RuntimeRef {
            rt: self,
        }
    }
}

/*
 * log buffer buffer
 */
// #[derive(Clone, Copy)]
struct LogBufBuf<const L: usize, const NR: usize> {
    deque: Deque<LogBuf<L>, NR>,
}

impl<const L: usize, const NR: usize>
Default for LogBufBuf<L, NR> {
    fn default() -> Self {
        Self {
            deque: Deque::default(),
        }
    }
}

impl<const L: usize, const NR: usize>
LogBufBuf<L, NR> {
    fn iter(&self) -> DequeRefIter<LogBuf<L>> {
        self.deque.iter()
    }

    fn iter_mut(&mut self) -> DequeMutRefIter<LogBuf<L>> {
        self.deque.iter_mut()
    }
}

/*
 * log buffer
 */
#[derive(Clone, Copy)]
struct LogBuf<const L: usize> {
    data: Deque<u8, L>,
}

impl<const L: usize>
Default for LogBuf<L> {
    fn default() -> Self {
        Self {
            data: Deque::default(),
        }
    }
}

impl<const L: usize>
Write for LogBuf<L> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if self.data.is_full() {
            for _ in 0..s.len() {
                self.data.pop();
            }
        }
        for b in s.as_bytes() {
            self.data.push(*b);
        }
        Ok(())
    }
}
