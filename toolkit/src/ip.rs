use crate::cmd::{ Queue, Poll };
use crate::cmd::rw::{ Request, Response, Error };
use toolkit_unsafe::{ IPCByteBuf };

pub struct IPQueue
<Q> {
// <Q, const BUFNR: usize, const RWNR: usize, const WR: usize, const RD: usize> {
    rwqueue: Q,
    // ipcbuf: Deque<IPCByteBuf, BUFNR>,
    // wrbuf: Deque<Deque<u8, WR>, RWNR>,
    // rdbuf: Deque<Deque<u8, RD>, RWNR>,
}

impl<Q>
// impl<Q, const BUFNR: usize, const RWNR: usize, const WR: usize, const RD: usize>
IPQueue<Q> {
    pub fn new(rwqueue: Q) -> Self {
        Self {
            rwqueue: rwqueue,
        }
    }
}


impl<Q>
// impl<Q, const BUFNR: usize, const RWNR: usize, const WR: usize, const RD: usize>
Queue for IPQueue<Q>
where Q: Queue<Request=Request, Response=Response, Error=Error> {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn push(&mut self, _req: Request) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn pop(&mut self) -> Poll<Result<Response, Error>> {
        Poll::Ready(Ok(Response::Ok))
    }
}
