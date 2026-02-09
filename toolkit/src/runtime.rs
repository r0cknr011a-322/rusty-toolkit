use core::fmt::{ self, Write };
use core::cell::{ RefCell };
use core::borrow::{ Borrow, BorrowMut };
use core::time::{ Duration };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };
use crate::task::{ Queue, Poll, GenRsp, GenErr };

pub trait Timer {
    fn time(&mut self) -> Duration;
}

pub trait Runtime {
    fn time(&self) -> Duration;
    fn log(&self, idx: usize) -> impl Write;
}

/*
 * runtime
 */
pub struct RuntimeMain<T, LOGQ, const LOGNR: usize, const LOGL: usize> {
    timer: RefCell<T>,
    logq: RefCell<LOGQ>,
    logbufbuf: RefCell<LogBufBuf<LOGNR, LOGL>>,
}

impl<T, Q, const NR: usize, const L: usize>
RuntimeMain<T, Q, NR, L> {
    pub fn new(timer: T, logq: Q) -> Self {
        Self {
            timer: RefCell::new(timer), logq: RefCell::new(logq),
            logbufbuf: RefCell::new(LogBufBuf::default()),
        }
    }
}

impl<T, Q, const NR: usize, const L: usize>
Runtime for RuntimeMain<T, Q, NR, L>
where T: Timer, Q: Queue<Request=u8, Response=GenRsp, Error=GenErr> {
    fn time(&self) -> Duration {
        self.timer.borrow_mut().time()
    }

    fn log(&self, idx: usize) -> impl Write {
        LogBufRef {
            idx: idx, rt: self,
        }
    }
}

/*
 * log buffer reference
 */
#[derive(Clone, Copy)]
struct LogBufRef<'a, T, Q, const NR: usize, const L: usize> {
    idx: usize,
    rt: &'a RuntimeMain<T, Q, NR, L>,
}

impl<'a, T, Q, const NR: usize, const L: usize>
Write for LogBufRef<'a, T, Q, NR, L>
where Q: Queue<Request=u8, Response=GenRsp, Error=GenErr> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let mut logbufbuf = self.rt.logbufbuf.borrow_mut();
        let Some(mut buf) = logbufbuf.iter_mut().nth(self.idx) else {
            return Ok(());
        };

        buf.write_str(s);

        let mut logq = self.rt.logq.borrow_mut();
        let mut cnt = 0;
        while cnt < buf.len() {
            while let Some(b) = buf.pop() {
                let Poll::Ready(res) = logq.push(b) else {
                    break;
                };
                let Ok(_) = res else {
                    return Ok(());
                };
            }
            cnt += loop {
                let mut cnt = 0;
                let Poll::Ready(res) = logq.pop() else {
                    break cnt;
                };
                let Ok(_) = res else {
                    return Ok(());
                };
                cnt += 1;
            };
        }

        Ok(())
    }
}

/*
 * log buffer buffer
 */
struct LogBufBuf<const NR: usize, const L: usize> {
    deque: Deque<LogBuf<L>, NR>,
}

impl<const NR: usize, const L: usize>
Default for LogBufBuf<NR, L> {
    fn default() -> Self {
        Self {
            deque: Deque::default(),
        }
    }
}

impl<const NR: usize, const L: usize>
LogBufBuf<NR, L> {
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
LogBuf<L> {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn pop(&mut self) -> Option<u8> {
        self.data.pop()
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
