use crate::cmd::{ Queue, Poll };
use crate::cmd::rw::{ Request, Response, Error };
use toolkit_unsafe::{ RawBuf };

pub struct IPQueue<Q, const BUFLEN: usize,
const WR: usize, const RD: usize, const RWNR: usize> {
    rwqueue: Q,
    bufbuf: Deque<Option<RawBuf>, BUFLEN>,
    wrbuf: Deque<Deque<u8, WR>, RWNR>,
    rdbuf: Deque<Deque<u8, RD>, RWNR>,
}

/*
impl<Q, const L: usize> IPQueue {
    pub fn new(rwqueue: Q) -> Self {
        Self {
            rwqueue: rwqueue, databuf: Deque::new(ctx),
        }
    }
}

impl<Q, const L: usize>
Queue for IPQueue<Q, L>
where Q::Request: Iterator<Item=Request> {
    type Request = Deque<u8, 

}
*/
