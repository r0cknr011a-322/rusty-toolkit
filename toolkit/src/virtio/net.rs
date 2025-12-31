use crate::collection::deque::{ Deque };
use crate::runtime::{ Runtime };
use crate::volatile::{ VolatileBuf };
use crate::cmd::{ Queue, Poll };
use crate::cmd::rw::{ Request, Response, Error };
use toolkit_unsafe::{ RawBuf };

pub struct NetDevDrv
<RT, IO,
const REQNR: usize, const RSPNR: usize,
const CMDLEN: usize, const BUFLEN: usize, const DATALEN:usize> {
    rt: RT,
    io: IO,
    reqbuf: Deque<Request<REQNR>, CMDLEN>,
    rspbuf: Deque<Response<RSPNR>, CMDLEN>,
    cmdbuf: Deque<RawBuf, BUFLEN>,
    databuf: Deque<RawBuf, DATALEN>,
}

impl<RT, IO,
const REQNR: usize, const RSPNR: usize,
const CMDLEN: usize, const BUFLEN: usize, const DATALEN: usize>
NetDevDrv<RT, IO, REQNR, RSPNR, CMDLEN, BUFLEN, DATALEN> {
    pub fn new(
        rt: RT, io: IO,
        cmdbuf: Deque<RawBuf, BUFLEN>, databuf: Deque<RawBuf, DATALEN>
    ) -> Self {
        Self {
            rt: rt, io: io,
            reqbuf: Deque::default(), rspbuf: Deque::default(),
            cmdbuf: cmdbuf, databuf: databuf,
        }
    }
}

impl<RT, IO,
const REQNR: usize, const RSPNR: usize,
const CMDLEN: usize, const BUFLEN: usize, const DATALEN: usize>
Queue for NetDevDrv<RT, IO, REQNR, RSPNR, CMDLEN, BUFLEN, DATALEN>
where RT: Runtime, IO: VolatileBuf {
    type Request = Request<REQNR>;
    type Response = Response<RSPNR>;
    type Error = Error;

    fn push(&mut self, req: Request<REQNR>) -> Poll<Result<(), Error>> {
        Poll::Ready(Err(Error::Fatal))
    }

    fn pop(&mut self) -> Poll<Result<Response<RSPNR>, Error>> {
        Poll::Ready(Err(Error::Fatal))
    }
}
