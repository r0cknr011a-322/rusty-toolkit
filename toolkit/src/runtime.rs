#[cfg(test)]
mod test;

mod rtref;

use core::fmt::{ self };
use core::cell::{ RefCell };
use core::time::{ Duration };
use core::array::{ self };

use crate::runtime::rtref::{ RuntimeRef };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };
use crate::collection::byteblock::{ ByteBlock };
use crate::collection::asynque::{ Asynque, Poll };


const SEND_RETRY_NR: u32 = 4;
const SEND_WAIT_NR: u32  = 4;

pub trait Timer {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Timer + fmt::Write { }

pub enum Error {
    Fatal,
}

pub struct RuntimeInner<T, Q, const NR: usize, const B: usize, const L: usize>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    timer: RefCell<T>,
    logchanbuf: RefCell<[LogBuf<B, L>; NR]>,
    iobuf: RefCell<Q>,
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeInner<T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    pub fn new(timer: T, asynque: Q) -> Self {
        Self {
            timer: RefCell::new(timer),
            logchanbuf: RefCell::new(array::from_fn(|_| { LogBuf::default() })),
            iobuf: RefCell::new(asynque),
        }
    }
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeInner<T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    pub fn chan(&self, idx: usize) -> Option<impl Runtime> {
        let Some(_) = self.logchanbuf.borrow().get(idx) else {
            return None;
        };
        Some(RuntimeRef::new(idx, self))
    }
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeInner<T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    pub fn time(&self) -> Duration {
        self.timer.borrow_mut().time()
    }
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
RuntimeInner<T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    fn flush(logbuf: &mut LogBuf<B, L>, asynque: &mut Q) -> Result<usize, LogErr> {
        let mut cnt = 0;
        for block in logbuf.iter() {
            for _ in 0..SEND_RETRY_NR {
                if let Poll::Ready(res) = asynque.try_push(*block) {
                    let Ok(()) = res else {
                        return Err(LogErr::Fatal);
                    };
                    break;
                }
            }
            cnt += 1;
        }

        for _ in 0..cnt {
            let _ = logbuf.pop();
        }

        Ok(cnt)
    }

    fn log(&self, data: &[u8], idx: usize) {
        let mut borrow = self.logchanbuf.borrow_mut();
        let logbuf = &mut borrow[idx];
        let mut iochan = self.iobuf.borrow_mut();

        let mut new = data.len() / L;
        if data.len() % L > 0 {
            new += 1;
        }

        let total = logbuf.len() + new;
        let mut cnt = 0;
        if total > logbuf.capacity() {
            let Ok(sent) = Self::flush(logbuf, &mut iochan) else {
                panic!("logging failed");
            };
            cnt += sent;
        }

        let mut ptr = 0;
        while total - cnt > logbuf.capacity() {
            let mut block: ByteBlock<L> = ByteBlock::default();
            block.wr_slice(&data[ptr..]);
            ptr += block.len();
            for _ in 0..SEND_RETRY_NR {
                if let Poll::Ready(res) = iochan.try_push(block) {
                    let Ok(()) = res else {
                        panic!("logging failed");
                    };
                    cnt += 1;
                    break;
                }
            }
        }

        for _ in 0..total - cnt {
            let mut block: ByteBlock<L> = ByteBlock::default();
            block.wr_slice(&data[ptr..]);
            ptr += block.len();
            logbuf.push(block);
        }
    }
}

impl<T, Q, const NR: usize, const B: usize, const L: usize>
Drop for RuntimeInner<T, Q, NR, B, L>
where T: Timer, Q: Asynque<Req=ByteBlock<L>, Rsp=(), Err=LogErr> {
    fn drop(&mut self) {
        let mut borrow = self.logchanbuf.borrow_mut();
        let mut iochan = self.iobuf.borrow_mut();

        for logbuf in borrow.iter_mut() {
            let _ = Self::flush(logbuf, &mut iochan);
        }

        for _ in 0..SEND_WAIT_NR {
            if let Poll::Ready(_) = iochan.poll_push() {
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
struct LogBuf<const B: usize, const L: usize> {
    blockbuf: Deque<ByteBlock<L>, B>,
}

impl<const B: usize, const L: usize> LogBuf<B, L> {
    fn push(&mut self, block: ByteBlock<L>) {
        self.blockbuf.push(block);
    }

    fn pop(&mut self) -> Option<ByteBlock<L>> {
        self.blockbuf.pop()
    }

    fn iter(&self) -> DequeRefIter<'_, ByteBlock<L>> {
        self.blockbuf.iter()
    }

    fn iter_mut(&mut self) -> DequeMutRefIter<'_, ByteBlock<L>> {
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
