use core::fmt::{ self, Write };
use core::cell::{ RefCell };
use core::borrow::{ Borrow, BorrowMut };
use core::time::{ Duration };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };
use crate::cmd::{ Queue };
use crate::bytebuf::{ MemByteBuf, ByteBuf, VolatileByteBuf, AtomicByteBuf };

pub trait Time {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Time {
    fn log<'a>(&'a self, idx: usize) -> Option<impl Write>;
    fn ipc(&self, idx: usize) -> Option<impl ByteBuf + VolatileByteBuf + AtomicByteBuf>;
    fn dev(&self, idx: usize) -> Option<impl VolatileByteBuf>;
}

/*
 * runtime
 */
pub struct RuntimeMain<'a, T, Q,
const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize> {
    timer: RefCell<T>,
    queue: RefCell<Q>,
    logbufbuf: RefCell<LogBufBuf<CHL, CHNR>>,
    ipcbufbuf: RefCell<Deque<MemByteBuf<'a>, IPCBUFNR>>,
    devmembuf: RefCell<Deque<MemByteBuf<'a>, DEVMEMNR>>,
}

impl<'a, T, Q, const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize>
RuntimeMain<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR> {
    pub fn new<
        I: FnMut(usize) -> MemByteBuf<'a>,
        D: FnMut(usize) -> MemByteBuf<'a>,
    >(timer: T, queue: Q, mut ipcbufctr: I, mut devmemctr: D) -> Self {
        Self {
            timer: RefCell::new(timer), queue: RefCell::new(queue),
            logbufbuf: RefCell::new(LogBufBuf::default()),
            ipcbufbuf: RefCell::new(Deque::new(|idx| ipcbufctr(idx))),
            devmembuf: RefCell::new(Deque::new(|idx| devmemctr(idx))),
        }
    }
}

impl<'a, T, Q, const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize>
Time for &RuntimeMain<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR>
where T: Time {
    fn time(&mut self) -> Duration {
        let mut timer = self.timer.borrow_mut();
        timer.time()
    }
}

impl<'a, T, Q, const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize>
Runtime for &RuntimeMain<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR>
where T: Time {
    fn log<'b>(&self, idx: usize) -> Option<impl Write> {
        if let Some(_) = self.logbufbuf.borrow().iter().nth(idx) {
            return Some(LogBufRef { idx: idx, rt: self });
        }
        None
    }

    fn ipc(&self, idx: usize) -> Option<impl ByteBuf + VolatileByteBuf + AtomicByteBuf> {
        if let Some(_) = self.ipcbufbuf.borrow().iter().nth(idx) {
            return Some(IPCBufRef { idx: idx, rt: self });
        }
        None
    }
}

#[derive(Clone, Copy)]
struct LogBufRef<'a, T, Q,
const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize> {
    idx: usize,
    rt: &'a RuntimeMain<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR>,
}

impl<'a, T, Q, const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize>
Write for LogBufRef<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if let Some(buf) = self.rt.logbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.write_str(s);
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct IPCBufRef<'a, T, Q,
const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize> {
    idx: usize,
    rt: &'a RuntimeMain<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR>,
}

impl<'a, T, Q, const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize>
ByteBuf for IPCBufRef<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR> {
    fn wr8(&mut self, off: usize, value: u8) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr8(off, value);
        }
    }

    fn rd8(&mut self, off: usize) -> u8 {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.rd8(off)
        }
        0
    }

    fn wr16(&mut self, off: usize, value: u16) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr16(off, value);
        }
    }

    fn rd16(&mut self, off: usize) -> u16 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd16(off)
    }
}

#[derive(Clone, Copy)]
struct DevBufRef<'a, T, Q,
const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize> {
    devbuf: usize,
    rt: &'a RuntimeMain<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR>,
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
    fn iter(&self) -> DequeRefIter<'_, LogBuf<L>> {
        self.deque.iter()
    }

    fn iter_mut(&mut self) -> DequeMutRefIter<'_, LogBuf<L>> {
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
fmt::Write for LogBuf<L> {
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
