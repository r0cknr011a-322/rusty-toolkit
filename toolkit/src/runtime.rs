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
    pub fn new<I, D>(timer: T, queue: Q, mut ipcbufctr: I, mut devmemctr: D) -> Self
    where I: FnMut(usize) -> MemByteBuf<'a>, D: FnMut(usize) -> MemByteBuf<'a> {
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
Runtime for &'a RuntimeMain<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR>
where T: Time {
    fn log(&self, idx: usize) -> Option<impl Write> {
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

    fn dev(&self, idx: usize) -> Option<impl VolatileByteBuf> {
        if let Some(_) = self.devmembuf.borrow().iter().nth(idx) {
            return Some(DevMemRef { idx: idx, rt: self });
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
struct DevMemRef<'a, T, Q,
const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize> {
    idx: usize,
    rt: &'a RuntimeMain<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR>,
}

impl<'a, T, Q, const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize>
VolatileByteBuf for DevMemRef<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR> {
    fn wr8_volatile(&mut self, off: usize, value: u8) {
        let mut bufref = self.rt.devmembuf.borrow_mut();
        if let Some(buf) = bufref.iter_mut().nth(self.idx) {
            buf.wr8_volatile(off, value);
        }
    }

    fn rd8_volatile(&mut self, off: usize) -> u8 {
        let mut bufref = self.rt.devmembuf.borrow_mut();
        let Some(bytebuf) = bufref.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd8_volatile(off)
    }

    fn wr16_volatile(&mut self, off: usize, value: u16) {
        let mut bufref = self.rt.devmembuf.borrow_mut();
        if let Some(buf) = bufref.iter_mut().nth(self.idx) {
            buf.wr16_volatile(off, value);
        }
    }

    fn rd16_volatile(&mut self, off: usize) -> u16 {
        let mut bufref = self.rt.devmembuf.borrow_mut();
        let Some(bytebuf) = bufref.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd16_volatile(off)
    }

    fn wr32_volatile(&mut self, off: usize, value: u32) {
        let mut bufref = self.rt.devmembuf.borrow_mut();
        if let Some(bytebuf) = bufref.iter_mut().nth(self.idx) {
            bytebuf.wr32_volatile(off, value);
        }
    }

    fn rd32_volatile(&mut self, off: usize) -> u32 {
        let mut bufref = self.rt.devmembuf.borrow_mut();
        let Some(bytebuf) = bufref.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd32_volatile(off)
    }

    fn wr64_volatile(&mut self, off: usize, value: u64) {
        let mut bufref = self.rt.devmembuf.borrow_mut();
        if let Some(buf) = bufref.iter_mut().nth(self.idx) {
            buf.wr64_volatile(off, value);
        }
    }

    fn rd64_volatile(&mut self, off: usize) -> u64 {
        let mut bufref = self.rt.devmembuf.borrow_mut();
        let Some(bytebuf) = bufref.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd64_volatile(off)
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
    fn addr(&self) -> usize {
        let bufref = self.rt.ipcbufbuf.borrow();
        let Some(bytebuf) = bufref.iter().nth(self.idx) else {
            return 0;
        };
        bytebuf.addr()
    }

    fn len(&self) -> usize {
        let bufref = self.rt.ipcbufbuf.borrow();
        let Some(bytebuf) = bufref.iter().nth(self.idx) else {
            return 0;
        };
        bytebuf.len()
    }

    fn copy_to(&mut self, off: usize, buf: &mut [u8]) {
        let mut bufref = self.rt.ipcbufbuf.borrow_mut();
        if let Some(bytebuf) = bufref.iter_mut().nth(self.idx) {
            bytebuf.copy_to(off, buf);
        }
    }

    fn copy_from(&mut self, off: usize, buf: &[u8]) {
        let mut bufref = self.rt.ipcbufbuf.borrow_mut();
        if let Some(bytebuf) = bufref.iter_mut().nth(self.idx) {
            bytebuf.copy_from(off, buf);
        }
    }

    fn wr8(&mut self, off: usize, value: u8) {
        let mut bufref = self.rt.ipcbufbuf.borrow_mut();
        if let Some(bytebuf) = bufref.iter_mut().nth(self.idx) {
            bytebuf.wr8(off, value);
        }
    }

    fn rd8(&mut self, off: usize) -> u8 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd8(off)
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

    fn wr32(&mut self, off: usize, value: u32) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr32(off, value);
        }
    }

    fn rd32(&mut self, off: usize) -> u32 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd32(off)
    }

    fn wr64(&mut self, off: usize, value: u64) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr64(off, value);
        }
    }

    fn rd64(&mut self, off: usize) -> u64 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd64(off)
    }
}

impl<'a, T, Q, const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize>
VolatileByteBuf for IPCBufRef<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR> {
    fn wr8_volatile(&mut self, off: usize, value: u8) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr8_volatile(off, value);
        }
    }

    fn rd8_volatile(&mut self, off: usize) -> u8 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd8_volatile(off)
    }

    fn wr16_volatile(&mut self, off: usize, value: u16) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr16_volatile(off, value);
        }
    }

    fn rd16_volatile(&mut self, off: usize) -> u16 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd16_volatile(off)
    }

    fn wr32_volatile(&mut self, off: usize, value: u32) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr32_volatile(off, value);
        }
    }

    fn rd32_volatile(&mut self, off: usize) -> u32 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd32_volatile(off)
    }

    fn wr64_volatile(&mut self, off: usize, value: u64) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr64_volatile(off, value);
        }
    }

    fn rd64_volatile(&mut self, off: usize) -> u64 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd64_volatile(off)
    }
}

impl<'a, T, Q, const IPCBUFNR: usize, const DEVMEMNR: usize, const CHL: usize, const CHNR: usize>
AtomicByteBuf for IPCBufRef<'a, T, Q, IPCBUFNR, DEVMEMNR, CHL, CHNR> {
    fn wr8_atomic(&mut self, off: usize, value: u8) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr8_atomic(off, value);
        }
    }

    fn rd8_atomic(&mut self, off: usize) -> u8 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd8_atomic(off)
    }

    fn wr16_atomic(&mut self, off: usize, value: u16) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr16_atomic(off, value);
        }
    }

    fn rd16_atomic(&mut self, off: usize) -> u16 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd16_atomic(off)
    }

    fn wr32_atomic(&mut self, off: usize, value: u32) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr32_atomic(off, value);
        }
    }

    fn rd32_atomic(&mut self, off: usize) -> u32 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd32_atomic(off)
    }

    fn wr64_atomic(&mut self, off: usize, value: u64) {
        if let Some(buf) = self.rt.ipcbufbuf.borrow_mut().iter_mut().nth(self.idx) {
            buf.wr64_atomic(off, value);
        }
    }

    fn rd64_atomic(&mut self, off: usize) -> u64 {
        let mut buf = self.rt.ipcbufbuf.borrow_mut();
        let Some(bytebuf) = buf.iter_mut().nth(self.idx) else {
            return 0;
        };
        bytebuf.rd64_atomic(off)
    }
}


