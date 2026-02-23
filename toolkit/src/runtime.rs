#[cfg(test)]
mod test;

mod rtref;

use core::fmt::{ self };
use core::cell::{ RefCell };
use core::time::{ Duration };
use core::array::{ self };

use crate::runtime::rtref::{ RuntimeRef };
use crate::collection::deque::{ Deque, DequeRefIter, DequeMutRefIter };


pub trait Timer {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Timer + fmt::Write {

}

pub struct RuntimeInner<T, const NR: usize, const L: usize> {
    timer: RefCell<T>,
    logbuf: RefCell<[Deque<u8, L>; NR]>,
}

impl<T, const NR: usize, const L: usize>
RuntimeInner<T, NR, L> {
    pub fn new(timer: T) -> Self {
        Self {
            timer: RefCell::new(timer),
            logbuf: RefCell::new(array::from_fn(|_| { Deque::default() })),
        }
    }
}

impl<T, const NR: usize, const L: usize>
RuntimeInner<T, NR, L>
where T: Timer {
    pub fn time(&self) -> Duration {
        self.timer.borrow_mut().time()
    }

    pub fn chan(&self, idx: usize) -> Option<impl Runtime> {
        let Some(_) = self.logbuf.borrow().get(idx) else {
            return None;
        };
        Some(RuntimeRef::new(idx, self))
    }
}

impl<T, const NR: usize, const L: usize>
RuntimeInner<T, NR, L> {
    fn log(&self, data: &[u8], idx: usize) {
        /*
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
        */
    }
}
