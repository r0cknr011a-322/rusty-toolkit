use crate::collection::deque::{ Deque };
use crate::cmd::{ Queue, Poll };
use crate::cmd::rw::{ Request as RWReq, Response as RWRsp, Error as RWErr };
use toolkit_unsafe::{ IPCByteBuf };

pub struct IPQueue
<Q, const CMDNR: usize> {
// <Q, const BUFNR: usize, const RWNR: usize, const WR: usize, const RD: usize> {
    rwqueue: Q,
    reqbuf: Deque<RWReq, CMDNR>,
    rspbuf: Deque<RWRsp, CMDNR>,
    // wrbuf: Deque<Deque<u8, WR>, RWNR>,
    // rdbuf: Deque<Deque<u8, RD>, RWNR>,
}

impl<Q, const BUFNR: usize>
// impl<Q, const BUFNR: usize, const RWNR: usize, const WR: usize, const RD: usize>
IPQueue<Q, BUFNR> {
    pub fn new(rwqueue: Q) -> Self {
        Self {
            rwqueue: rwqueue,
            reqbuf: Deque::new(|_| RWReq::Read(IPCByteBuf::new(0, 0))),
            rspbuf: Deque::new(|_| RWRsp::Ok),
        }
    }
}


impl<Q, const CMDNR: usize, Req: Iterator<Item=RWReq>>
// impl<Q, const BUFNR: usize, const RWNR: usize, const WR: usize, const RD: usize>
Queue for IPQueue<Q, CMDNR>
where Q: Queue<Shit> {
    type Response = Deque<RWRsp, CMDNR>;
    type Error = RWErr;

    fn push(&mut self, reqbuf: Req) -> Poll<Result<(), Self::Error>> {
        for req in reqbuf {
            if let RWReq::Read(buf) = req {
                if buf.len() < 128 {
                    return Poll::Ready(Err(RWErr::Fatal));
                }
            }
        }
        Poll::Ready(Ok(()))
    }

    fn pop(&mut self) -> Poll<Result<Self::Response, Self::Error>> {
        Poll::Ready(Ok(self.rspbuf))
    }
}
