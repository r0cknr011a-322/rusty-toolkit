use crate::collection::deque::{ Deque };
use crate::cmd::{ Queue, Poll };
use crate::cmd::rw::{ Response, Error };

struct IPCBufQueue<Q, const REQNR: usize, const RSPNR: usize> {
    queue: Q,
    // reqbuf: Deque<IPCByteBuf, REQNR>,
    rspbuf: Deque<Response, RSPNR>,
}

impl<Q, const REQNR: usize, const RSPNR: usize>
IPCBufQueue<Q, REQNR, RSPNR> {
    fn new(queue: Q) -> Self {
        Self {
            queue: queue,
            // reqbuf: Deque::new(|_| IPCByteBuf::new(0, 0)),
            rspbuf: Deque::new(|_| Response::Ok),
        }
    }
}

/*
 * send queue
 */
pub struct SendIPCBufQueue<Q, const REQNR: usize, const RSPNR: usize> {
    queue: IPCBufQueue<Q, REQNR, RSPNR>,
}

impl<Q, const REQNR: usize, const RSPNR: usize>
SendIPCBufQueue<Q, REQNR, RSPNR> {
    pub fn new(queue: Q) -> Self {
        Self {
            queue: IPCBufQueue::new(queue),
        }
    }
}

impl<Q, const REQNR: usize, const RSPNR: usize>
Queue for SendIPCBufQueue<Q, REQNR, RSPNR>
where Q: Queue<Request=usize, Response=Response, Error=Error> {
    type Request = usize;
    type Response = Response;
    type Error = Error;

    fn push(&mut self, _req: usize) -> Poll<Result<(), Error>> {
        Poll::Ready(Err(Error::Fatal))
    }

    fn pop(&mut self) -> Poll<Result<Response, Error>> {
        Poll::Ready(Err(Error::Fatal))
    }
}

/*
 * recv queue
 */
pub struct RecvIPCBufQueue<Q, const REQNR: usize, const RSPNR: usize> {
    queue: IPCBufQueue<Q, REQNR, RSPNR>,
}
