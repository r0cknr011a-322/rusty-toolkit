use core::time::{ Duration };

use crate::runtime::{ Timer, RuntimeInner, LogErr };
use crate::collection::deque::{ Deque };
use crate::collection::byteblock::{ ByteBlock };
use crate::collection::asynque::{ Asynque, Poll };


#[derive(Default)]
struct TestTimer {

}

impl Timer for TestTimer {
    fn time(&mut self) -> Duration {
        Duration::default()
    }
}

#[derive(Default)]
struct TestAsynque<const B: usize, const L: usize> {
    blkbuf: Deque<ByteBlock<L>, B>,
}

impl<const B: usize, const L: usize>
Asynque for TestAsynque<B, L> {
    type Req = ByteBlock<L>;
    type Rsp = ();
    type Err = LogErr;

    fn try_push(&mut self, byteblk: ByteBlock<L>) -> Poll<Result<(), LogErr>> {
        Poll::Ready(Err(LogErr::Fatal))
    }

    fn poll_push(&mut self) -> Poll<Result<(), LogErr>> {
        Poll::Ready(Err(LogErr::Fatal))
    }

    fn try_pop(&mut self) -> Poll<Result<(), LogErr>> {
        Poll::Ready(Err(LogErr::Fatal))
    }

    fn poll_pop(&mut self) -> Poll<Result<(), LogErr>> {
        Poll::Ready(Err(LogErr::Fatal))
    }
}


const BUF_NR: usize     = 8;
const BLK_NR: usize     = 8;
const BLK_LEN: usize    = 0x80;

#[test]
fn log() {
    let rt = RuntimeInner::<TestTimer, TestAsynque<BLK_NR, BLK_LEN>, BUF_NR, BLK_NR, BLK_LEN>::new(
        TestTimer::default(), TestAsynque::default()
    );
}
