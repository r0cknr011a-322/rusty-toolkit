// use crate::runtime::{ Runtime };
// use crate::volatile::{ VolatileBuf };
use crate::collection::deque::{ Deque };
use crate::cmd::{ Poll };
use crate::cmd::rw::{ Queue, Request, Response, Error };
use toolkit_unsafe::{ RawBuf };

pub struct NetDevDrv
<RT, IO,
const REQNR: usize, const RSPNR: usize,
const BUFLEN: usize, const DATALEN:usize> {
    rt: RT,
    io: IO,
    reqbuf: Deque<Request, REQNR>,
    rspbuf: Deque<Response, RSPNR>,
    cmdbuf: Deque<RawBuf, BUFLEN>,
    databuf: Deque<RawBuf, DATALEN>,
}

impl<RT, IO,
const REQNR: usize, const RSPNR: usize,
const BUFLEN: usize, const DATALEN:usize>
NetDevDrv<RT, IO, REQNR, RSPNR, BUFLEN, DATALEN> {
    pub fn new(
        rt: RT, io: IO,
        cmdbuf: Deque<RawBuf, BUFLEN>, databuf: Deque<RawBuf, DATALEN>
    ) -> Self {
        Self {
            rt: rt, io: io,
            reqbuf: Deque::new(|_| Request::Noop), rspbuf: Deque::new(|_| Response::Ok),
            cmdbuf: cmdbuf, databuf: databuf,
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
