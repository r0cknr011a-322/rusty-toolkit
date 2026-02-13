use core::fmt::{ self, Write };
use core::cell::{ RefCell };
use core::time::{ Duration };
use core::array::{ self };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };
use crate::task::{ SendByteQueue, ByteQueueErr, Poll };

pub trait Timer {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Write {
    fn time(&self) -> Duration;
}

/*
 * runtime reference
 */
#[derive(Clone, Copy)]
struct RuntimeRef<'a, T, Q, const NR: usize, const L: usize> {
    logidx: usize,
    inner: &'a RuntimeInner<T, Q, NR, L>,
}

impl<'a, T, Q, const NR: usize, const L: usize>
Write for RuntimeRef<'a, T, Q, NR, L>
where Q: SendByteQueue {
    fn write_str(&mut self, data: &str) -> Result<(), fmt::Error> {
        // self.logbuf.log(self.logidx, data);
        Ok(())
    }
}

impl<'a, T, Q, const NR: usize, const L: usize>
Runtime for RuntimeRef<'a, T, Q, NR, L>
where T: Timer, Q: SendByteQueue {
    fn time(&self) -> Duration {
        self.inner.time()
    }
}

/*
 * runtime inner
 */
pub struct RuntimeInner<T, Q, const LOGNR: usize, const LOGL: usize> {
    timer: RefCell<T>,
    logbuf: RefCell<[CharBuf<LOGL>; LOGNR]>,
    logq: RefCell<Q>,
}

impl<T, Q, const NR: usize, const L: usize>
RuntimeInner<T, Q, NR, L> {
    pub fn new(timer: T, logq: Q) -> Self {
        Self {
            timer: RefCell::new(timer),
            logbuf: RefCell::new(array::from_fn(|_| { CharBuf::default() })),
            logq: RefCell::new(logq),
        }
    }
}

impl<T, Q, const NR: usize, const L: usize>
RuntimeInner<T, Q, NR, L>
where T: Timer, Q: SendByteQueue {
    pub fn chan(&self, idx: usize) -> Option<impl Runtime> {
        let Some(logidx) = self.logbuf.borrow().get(idx) else {
            return None;
        };
        Some(RuntimeRef {
            logidx: idx,
            inner: &self,
        })
    }
}


impl<T, Q, const NR: usize, const L: usize>
RuntimeInner<T, Q, NR, L>
where T: Timer {
    pub fn time(&self) -> Duration {
        self.timer.borrow_mut().time()
    }
}

impl<T, Q, const NR: usize, const L: usize>
RuntimeInner<T, Q, NR, L>
where Q: SendByteQueue {
    pub fn log(&mut self, idx: usize, data: &str) {
        let mut logbuf = self.logbuf.borrow_mut();
        let mut buf = &mut logbuf[idx];
        if data.len() < buf.free() {
            buf.push(data.as_bytes());
            return;
        }
    }
}

/*
 * char buffer
 */
struct CharBuf<const L: usize> {
    buf: Deque<u8, L>,
}

impl<const L: usize>
Default for CharBuf<L> {
    fn default() -> Self {
        Self {
            buf: Deque::default(),
        }
    }
}

impl<const L: usize>
CharBuf<L> {
    fn as_slices(&self) -> (&[u8], &[u8]) {
        self.buf.as_slices()        
    }

    fn len(&self) -> usize {
        self.buf.len()
    }

    fn free(&self) -> usize {
        self.buf.free()
    }

    fn push(&mut self, data: &[u8]) {
        for b in data {
            self.buf.push(*b);
        }
    }

    fn pop(&mut self, cnt: usize) {
        for _ in 0..cnt {
            let _ = self.buf.pop();
        }
    }
}
