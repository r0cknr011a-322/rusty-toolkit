use core::fmt::{ self, Write };
use core::cell::{ RefCell };
use core::time::{ Duration };
use core::array::{ self };
use core::cmp::{ self };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };
use crate::bytechan::{ Session, ByteChan, Error as BBErr, Poll };

pub trait Timer {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Write {
    fn time(&self) -> Duration;
}

pub enum Error {
    Fatal,
}

/*
 * runtime reference
 */
#[derive(Clone, Copy)]
struct RuntimeRef<'a, T, B, const NR: usize, const L: usize> {
    logidx: usize,
    inner: &'a RuntimeInner<T, B, NR, L>,
}

impl<'a, T, B, const NR: usize, const L: usize>
Write for RuntimeRef<'a, T, B, NR, L>
where B: ByteChan {
    fn write_str(&mut self, data: &str) -> Result<(), fmt::Error> {
        self.inner.push(data.as_bytes(), self.logidx);
        Ok(())
    }
}

impl<'a, T, B, const NR: usize, const L: usize>
Runtime for RuntimeRef<'a, T, B, NR, L>
where T: Timer, B: ByteChan {
    fn time(&self) -> Duration {
        self.inner.time()
    }
}

/*
 * runtime inner
 */
pub struct RuntimeInner<T, B, const LOGNR: usize, const LOGL: usize> {
    timer: RefCell<T>,
    logbuf: RefCell<[CharBuf<LOGL>; LOGNR]>,
    logio: RefCell<B>,
}

impl<T, B, const NR: usize, const L: usize>
RuntimeInner<T, B, NR, L> {
    pub fn new<S: Session<Arg=A, Buf=B>, A>(timer: T, mut session: S, arg: A) -> Result<Self, Error> {
        let Poll::Ready(Ok(bytebuf)) = session.init(arg) else {
            return Err(Error::Fatal);
        };
        Ok(Self {
            timer: RefCell::new(timer),
            logbuf: RefCell::new(array::from_fn(|_| { CharBuf::default() })),
            logio: RefCell::new(bytebuf),
        })
    }
}

impl<T, B, const NR: usize, const L: usize>
RuntimeInner<T, B, NR, L>
where T: Timer, B: ByteChan {
    pub fn chan(&self, idx: usize) -> Option<impl Runtime> {
        let Some(logidx) = self.logbuf.borrow().get(idx) else {
            return None;
        };
        Some(RuntimeRef {
            logidx: idx, inner: &self,
        })
    }

    
}

impl<T, B, const NR: usize, const L: usize>
RuntimeInner<T, B, NR, L>
where B: ByteChan {
    fn push(&self, data: &[u8], idx: usize) {
        let mut borrow = self.logbuf.borrow_mut();
        let mut charbuf = &mut borrow[idx];
        let mut bytebuf = self.logio.borrow_mut();

        let mut saved = false;
        if data.len() < charbuf.free() {
            charbuf.push(data);
            saved = true;
        }

        let mut lvl = charbuf.capacity() / 2;
        if !saved {
            lvl = 0;
        }

        while charbuf.len() > lvl {
            let cnt = {
                let mut cnt = 0;
                let (datal, datar) = charbuf.as_slices();

                let Poll::Ready(res) = bytebuf.send(datal) else {
                    continue;
                };
                let Ok(cntl) = res else {
                    return;
                };
                cnt += cntl;

                let Poll::Ready(res) = bytebuf.send(datar) else {
                    continue;
                };
                let Ok(cntr) = res else {
                    return;
                };
                cnt += cntr;
                cnt
            };
            charbuf.pop(cnt);
        }

        if saved {
            return;
        }

        let mut sent = 0;
        while data.len() - sent < charbuf.capacity() {
            let Poll::Ready(res) = bytebuf.send(&data[sent..]) else {
                continue;
            };
            let Ok(cnt) = res else {
                return;
            };
            sent += cnt;
        }

        charbuf.push(&data[sent..]);
    }
}

impl<T, Q, const NR: usize, const L: usize>
RuntimeInner<T, Q, NR, L>
where T: Timer {
    pub fn time(&self) -> Duration {
        self.timer.borrow_mut().time()
    }
}

impl<T, B, const NR: usize, const L: usize>
RuntimeInner<T, B, NR, L>
where B: ByteChan {
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

    fn capacity(&self) -> usize {
        self.buf.capacity()
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
