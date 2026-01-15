use core::fmt::{ self, Write };
use core::cell::{ Cell };
use core::time::{ Duration };
use crate::collection::deque::{ Deque };
use crate::cmd::{ Queue };
use toolkit_unsafe::{ IPCByteBuf };

pub trait Time {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Write + Time {
    fn chan(&self, name: &'static str) -> impl Runtime;
}

/*
 * runtime channel
 */
#[derive(Clone, Copy)]
struct RuntimeChan
<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize> {
    name: &'static str,
    rt: &'a RuntimeMain<T, Q, BUFNR, CHL, CHNR>,
}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
RuntimeChan<'a, T, Q, BUFNR, CHL, CHNR>
where T: Time + Copy {
    fn new(rt: &'a RuntimeMain<T, Q, BUFNR, CHL, CHNR>, name: &'static str) -> Self {
        Self {
            rt: rt, name: name,
        }
    }

}

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
Write for RuntimeChan<'a, T, Q, BUFNR, CHL, CHNR> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.rt.logchanbuf.update(|chanbuf| {
            /*
            chanbuf.into_iter().map(|chan| {
                chanbuf
            }).collect()
            */
            chanbuf
        });
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

impl<'a, T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
Runtime for RuntimeChan<'a, T, Q, BUFNR, CHL, CHNR>
where T: Time + Copy {
    fn chan(&self, name: &'static str) -> impl Runtime {
        RuntimeChan::new(self.rt, name)
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
}

impl<T, Q, const BUFNR: usize, const CHL: usize, const CHNR: usize>
RuntimeMain<T, Q, BUFNR, CHL, CHNR>
where T: Time + Copy {
    pub fn chan(&self, name: &'static str) -> impl Runtime {
        RuntimeChan::new(self, name)
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
                self.data.pop();
            }
        }
        for b in s.as_bytes() {
            self.data.push(*b);
        }
        Ok(())
    }
}
