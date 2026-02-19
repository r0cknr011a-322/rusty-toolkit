use core::fmt::{ self, Write };
use core::cell::{ RefCell };
use core::time::{ Duration };
use core::array::{ self };
use core::cmp::{ self };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };
use crate::collection::asynque::{ Asynque, Poll };

const SEND_RETRY_NR: u32 = 4;
const SEND_WAIT_NR: u32  = 4;

pub trait Timer {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Timer + Write { }

pub enum Error {
    Fatal,
}

/*
 * runtime reference
 */
#[derive(Clone, Copy)]
struct RuntimeRef<'a, T, Q, const NR: usize, const B: usize, const L: usize>
where Q: Asynque {
    logidx: usize,
    inner: &'a RuntimeInner<T, Q, NR, B, L>,
}

impl<'a, T, Q, const NR: usize, const B: usize, const L: usize>
Write for RuntimeRef<'a, T, Q, NR, B, L>
where Q: Asynque<Req=CharBlock<L>, Rsp=(), Err=LogErr> {
    fn write_str(&mut self, data: &str) -> Result<(), fmt::Error> {
        self.inner.log(data.as_bytes(), self.logidx);
        Ok(())
    }
}

impl<'a, T, Q, const NR: usize, const B: usize, const L: usize>
Timer for RuntimeRef<'a, T, Q, NR, B, L>
where T: Timer, Q: Asynque {
    fn time(&mut self) -> Duration {
        self.inner.time()
    }
}

impl<'a, T, Q, const NR: usize, const B: usize, const L: usize>
Runtime for RuntimeRef<'a, T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=CharBlock<L>, Rsp=(), Err=LogErr> {

}

/*
 * runtime inner
 */
pub struct RuntimeInner<T, Q, const NR: usize, const B: usize, const L: usize>
where Q: Asynque {
    timer: RefCell<T>,
    logbuf: RefCell<[CharBuf<B, L>; NR]>,
    iobuf: RefCell<Q>,
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeInner<T, Q, NR, B, L>
where Q: Asynque {
    pub fn new(timer: T, asynque: Q) -> Self {
        Self {
            timer: RefCell::new(timer),
            logbuf: RefCell::new(array::from_fn(|_| { CharBuf::default() })),
            iobuf: RefCell::new(asynque),
        }
    }
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeInner<T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=CharBlock<L>, Rsp=(), Err=LogErr> {
    pub fn chan(&self, idx: usize) -> Option<impl Runtime> {
        let Some(logidx) = self.logbuf.borrow().get(idx) else {
            return None;
        };
        Some(RuntimeRef {
            logidx: idx, inner: &self,
        })
    }
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeInner<T, Q, NR, B, L>
where Q: Asynque<Req=CharBlock<L>> {
    fn log(&self, data: &[u8], idx: usize) {
        let mut logbuf = self.logbuf.borrow_mut();
        let mut charbuf = &mut logbuf[idx];
        let mut iobuf = self.iobuf.borrow_mut();

        let mut new = data.len() / L;
        if data.len() % L > 0 {
            new += 1;
        }

        if charbuf.len() + new < charbuf.capacity() {
            let mut ptr = 0;
            for i in 0..new {
                let mut block = CharBlock::default();
                block.wr_slice(&data[i + L..]);
                if data.len() < block.capacity() {
                }
            }
            return;
        }

        let mut cnt = 0;
        while new - cnt < charbuf.capacity() {
            for block in charbuf.iter() {
                for _ in 0..SEND_RETRY_NR {
                    if let Poll::Ready(res) = iobuf.try_push(*block) {
                        let Ok(()) = res else {
                            panic!("logging failed");
                        };
                        cnt += 1;
                        break;
                    }
                }
            }

            for _ in 0..cnt {
                let _ = charbuf.pop();
            }
        }

        let mut cnt = data.len() / L + 1;
        while data.len() - cnt > 0 {
        }

        if charbuf.len() == 0 {
        }
    }
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeInner<T, Q, NR, B, L>
where T: Timer, Q: Asynque {
    pub fn time(&self) -> Duration {
        self.timer.borrow_mut().time()
    }
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
Drop for RuntimeInner<T, Q, NR, B, L>
where Q: Asynque {
    fn drop(&mut self) {
        let mut iobuf = self.iobuf.borrow_mut();
        for _ in 0..SEND_WAIT_NR {
            if let Poll::Ready(_) = iobuf.poll_push() {
                return;
            }
        }
    }
}

pub enum LogErr {
    Fatal,
}

/*
 * char buffer
 */
#[derive(Default)]
struct CharBuf<const B: usize, const L: usize> {
    blockbuf: Deque<CharBlock<L>, B>,
}

// impl<const L: usize> Default for CharBuf<L> {
//     fn default() -> Self {
//         Self {
//             buf: Deque::default(),
//         }
//     }
// }

impl<const B: usize, const L: usize>
CharBuf<B, L> {
    fn push(&mut self, block: CharBlock<L>) {
        self.blockbuf.push(block);
    }

    fn pop(&mut self) -> Option<CharBlock<L>> {
        self.blockbuf.pop()
    }

    fn iter(&self) -> DequeRefIter<'_, CharBlock<L>> {
        self.blockbuf.iter()
    }

    fn iter_mut(&mut self) -> DequeMutRefIter<'_, CharBlock<L>> {
        self.blockbuf.iter_mut()
    }

    fn capacity(&self) -> usize {
        self.blockbuf.capacity()
    }

    fn len(&self) -> usize {
        self.blockbuf.len()
    }

    fn free(&self) -> usize {
        self.blockbuf.free()
    }
}

#[derive(Clone, Copy)]
pub struct CharBlock<const L: usize> {
    buf: [u8; L],
    len: usize,
}

impl<const L: usize> Default for CharBlock<L> {
    fn default() -> Self {
        Self {
            buf: array::from_fn(|_| { 0 }),
            len: 0,
        }
    }
}

impl<const L: usize> CharBlock<L> {
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn free(&self) -> usize {
        self.buf.len() - self.len
    }

    pub fn rd_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn wr_slice(&mut self, data: &[u8]) {
        let slice = &mut self.buf[self.len..];
        if data.len() < slice.len() {
            let (dst, _) = slice.split_at_mut(data.len());
            dst.copy_from_slice(data);
            self.len += data.len();
            return;
        }
        if data.len() > slice.len() {
            let (src, _) = data.split_at(slice.len());
            slice.copy_from_slice(src);
            self.len += slice.len();
            return;
        }
        slice.copy_from_slice(data);
        self.len += data.len();
    }
}
