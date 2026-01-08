use crate::collection::deque::{ Deque };
use crate::cmd::{ Queue, Poll };
use crate::cmd::rw::{ Request, Response, Error };
use toolkit_unsafe::{ IPCByteBuf };

pub struct IPRefQueue
<OQ, const OUTLEN: usize, IQ, const INLEN: usize>
{
    outq: OQ,
    outreq: Deque<IPCByteBuf, REQNR>,
    outrsp: Deque<Response, RSPNR>,
    inq: IQ,
    rspbuf: Deque<IPCByteBuf, NR>,
}

impl
<ORQ, IRQ, OBQ, IBQ,
const REQNR: usize, const RSPNR: usize,
const WRLEN: usize, const RDLEN: usize>
IPQueue<ORQ, IRQ, OBQ, IBQ, REQNR, RSPNR, WRLEN, RDLEN> {
    pub fn new(rwqueue: Q) -> Self {
        Self {
            rwqueue: rwqueue,
            reqbuf: Deque::new(|_| RWReq::Read(IPCByteBuf::new(0, 0))),
            rspbuf: Deque::new(|_| RWRsp::Ok),
            wrbuf: Deque::default(), rdbuf: Deque::default(),
        }
    }
}


impl<Q,
const REQNR: usize, const RSPNR: usize,
const WRLEN: usize, const RDLEN: usize>
BufRefQueue for IPQueue<Q, REQNR, RSPNR, WRLEN, RDLEN> {
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
