// use crate::runtime::{ Runtime };
use crate::volatile::{ VolatileByteBuf };
use crate::collection::deque::{ Deque };
use crate::cmd::{ Queue, Poll };
use crate::cmd::rw::{ Request, Response, Error };
use toolkit_unsafe::{ IPCByteBuf };

pub struct NetDevDrv
<RT, IO, const REQNR: usize, const RSPNR: usize, const CMDBUFNR: usize> {
    rt: RT,
    io: IO,
    reqbuf: Deque<Request, REQNR>,
    rspbuf: Deque<Response, RSPNR>,
    cmdbuf: Deque<IPCByteBuf, CMDBUFNR>,
}

impl<RT, IO, const REQNR: usize, const RSPNR: usize, const CMDBUFNR: usize>
NetDevDrv<RT, IO, REQNR, RSPNR, CMDBUFNR> {
    pub fn new<CmdBufCtx: FnMut(usize) -> IPCByteBuf>(rt: RT, io: IO, ctx: CmdBufCtx) -> Self {
        Self {
            rt: rt, io: io,
            reqbuf: Deque::new(|_| Request::Read(IPCByteBuf::new(0, 0))),
            rspbuf: Deque::new(|_| Response::Ok),
            cmdbuf: Deque::new(ctx),
        }
    }
}

/*
impl<RT, IO,
const REQNR: usize, const RSPNR: usize,
const BUFLEN: usize, const DATALEN:usize>
Queue for NetDevDrv<RT, IO, REQNR, RSPNR, BUFLEN, DATALEN>
where RT: Runtime, IO: VolatileBuf {
    fn push(&mut self, _req: Request) -> Poll<Result<(), Error>> {
        Poll::Ready(Err(Error::Fatal))
    }

    fn pop(&mut self) -> Poll<Result<&Response, Error>> {
        Poll::Ready(Err(Error::Fatal))
    }
}
*/
