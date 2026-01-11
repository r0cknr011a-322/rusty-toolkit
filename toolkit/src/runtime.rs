use core::fmt::{ self, Write };
use core::cell::{ Cell };
use core::time::{ Duration };
use crate::collection::deque::{ Deque, DequeIter };
use crate::cmd::{ Queue };
use toolkit_unsafe::{ IPCByteBuf };

pub trait Time {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Write + Time { }

/*
 * runtime channel
 */
#[derive(Clone, Copy)]
pub struct RuntimeChan
<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize> {
    name: &'static str,
    rt: &'a RuntimeMain<T, Q, BUFNR, CHL, CHNR>,
}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
RuntimeChan<'a, T, Q, BUFNR, CHL, CHNR> {
    fn new(name: &'static str, rt: &'a RuntimeMain<T, Q, BUFNR, CHL, CHNR>) -> Self {
        Self {
            name: name, rt: rt,
        }
    }
}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
Write for RuntimeChan<'a, T, Q, BUFNR, CHL, CHNR> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let chanbuf = self.rt.logchanbuf.get();
        for mut chan in chanbuf {
            if chan.name == self.name {
                chan.write_str(s);
            }
        }
        Ok(())
    }
}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
Time for RuntimeChan<'a, T, Q, BUFNR, CHL, CHNR>
where T: Time + Copy {
    fn time(&mut self) -> Duration {
        let mut timer = self.rt.timer.get();
        timer.time()
    }
}

/*
 * runtime
 */
pub struct RuntimeMain<T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize> {
    timer: Cell<T>,
    queue: Cell<Q>,
    bufbuf: Cell<Deque<IPCByteBuf, BUFNR>>,
    logchanbuf: Cell<LogChanBuf<CHL, CHNR>>,
}

impl<T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
RuntimeMain<T, Q, BUFNR, CHL, CHNR> {
    pub fn new<C: FnMut(usize) -> &'static str,
               B: FnMut(usize) -> IPCByteBuf>
        (timer: T, queue: Q, mut chanctr: C, mut bufctr: B) -> Self {
        Self {
            timer: Cell::new(timer), queue: Cell::new(queue),
            bufbuf: Cell::new(Deque::new(|idx| bufctr(idx))),
            logchanbuf: Cell::new(LogChanBuf::new(|idx| chanctr(idx))),
        }
    }

    pub fn chan(&self, name: &'static str) -> Option<RuntimeChan<T, Q, BUFNR, CHL, CHNR>> {
        let buf = self.logchanbuf.get();
        for chan in buf {
            if chan.name == name {
                return Some(RuntimeChan::new(name, self));
            }
        }
        None
    }
}

/*
 * logger channel buffer
 */
#[derive(Clone, Copy)]
struct LogChanBuf<const L: usize, const NR: usize> {
    deque: Deque<LogChan<L>, NR>,
}

impl<const L: usize, const NR: usize>
LogChanBuf<L, NR> {
    fn new<C: FnMut(usize) -> &'static str>(mut ctr: C) -> Self {
        Self {
            deque: Deque::new(|idx| LogChan::new(ctr(idx))),
        }
    }
}

impl<const L: usize, const NR: usize>
IntoIterator for LogChanBuf<L, NR> {
    type Item = LogChan<L>;
    type IntoIter = DequeIter<LogChan<L>, NR>;

    fn into_iter(self) -> Self::IntoIter {
        self.deque.into_iter()
    }
}

/*
 * logger channel
 */
#[derive(Clone, Copy)]
struct LogChan<const L: usize> {
    name: &'static str,
    data: Deque<u8, L>,
}

impl<const L: usize> LogChan<L> {
    fn new(name: &'static str) -> Self {
        Self {
            name: name, data: Deque::default(),
        }
    }
}

impl<const L: usize>
Write for LogChan<L> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if self.data.is_full() {
            for _ in 0..s.len() {
                self.data.pop_front();
            }
        }
        for b in s.as_bytes() {
            self.data.push_back(*b);
        }
        Ok(())
    }
}
