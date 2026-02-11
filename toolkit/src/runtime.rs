use core::fmt::{ self, Write };
use core::cell::{ RefCell };
use core::time::{ Duration };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };
use crate::task::{ Poll };
use crate::task::bytequeue::{ SendByteQueue, ByteQueueErr };

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
pub struct RuntimeMain<T, Q, const LOGNR: usize, const LOGL: usize> {
    timer: RefCell<T>,
    logbuf: RefCell<LogBuf<LOGNR, LOGL>>,
    logq: RefCell<Q>,
}

impl<T, Q, const NR: usize, const L: usize>
RuntimeMain<T, Q, NR, L> {
    pub fn new(timer: T, logq: Q) -> Self {
        Self {
            timer: RefCell::new(timer),
            logbuf: RefCell::new(LogBuf::default()),
            logq: RefCell::new(logq),
        }
    }
}

impl<T, Q, const NR: usize, const L: usize>
Runtime for RuntimeMain<T, Q, NR, L>
where T: Timer, Q: SendByteQueue {
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
where Q: SendByteQueue {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let mut logbuf = self.rt.logbuf.borrow_mut();
        if let Some(mut buf) = logbuf.iter_mut().nth(self.idx) {
            let data = s.as_bytes();
            if data.len() < buf.free() {
                buf.push(data);
                return Ok(());
            }
        }
        Ok(())
    }
}

/*
 * log buffer buffer
 */
struct LogBuf<const NR: usize, const L: usize> {
    deque: Deque<CharBuf<L>, NR>,
}

impl<const NR: usize, const L: usize>
Default for LogBuf<NR, L> {
    fn default() -> Self {
        Self {
            deque: Deque::default(),
        }
    }
}

impl<const NR: usize, const L: usize>
LogBuf<NR, L> {
    fn iter(&self) -> DequeRefIter<'_, CharBuf<L>> {
        self.deque.iter()
    }

    fn iter_mut(&mut self) -> DequeMutRefIter<'_, CharBuf<L>> {
        self.deque.iter_mut()
    }
}

/*
 * char buffer
 */
#[derive(Clone, Copy)]
struct CharBuf<const L: usize> {
    data: Deque<u8, L>,
}

impl<const L: usize>
Default for CharBuf<L> {
    fn default() -> Self {
        Self {
            data: Deque::default(),
        }
    }
}

impl<const L: usize>
CharBuf<L> {
    fn as_slices(&self) -> (&[u8], &[u8]) {
        self.data.as_slices()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn free(&self) -> usize {
        self.data.free()
    }

    fn push(&mut self, data: &[u8]) {
        for b in data {
            self.data.push(*b);
        }
    }

    fn pop(&mut self, cnt: usize) {
        for _ in 0..cnt {
            let _ = self.data.pop();
        }
    }
}
