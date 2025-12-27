use core::cmp::{ min };
use crate::trx::{ Queue, Poll, Task, RWRequest, RWResponse, RWError };
use crate::mem::{ DevBuf };

pub struct Uart16550<IO> {
    io: IO,
}

impl<IO> Uart16550<IO> {
    pub fn new(io: IO) -> Self {
        Self {
            io: io,
        }
    }
}

pub struct Uart16550Task<'a, const LEN: usize, IO> {
    io: IO,
    req: RWRequest<'a, LEN>,
}

/*
impl<IO: DevBuf> AsyncQueue<RWRequest> for Uart16550<IO> {
    type Response = RWResponse;
    type Error = RWError;

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn queue(&mut self, req: Request) {
    }

    fn poll_queue(&mut self) -> Poll<Result<Self::Response, Self::Error>> {
        Poll::Ready(Ok(RWResponse::Ok()))
    }
}

impl<IO: RWVolatile8> ServiceCall for Uart16550Call<IO> {
    fn poll_call(&mut self) -> Poll<Result<Self::Response, Self::Error>> {
        for b in self.req.buf {
            self.io.wr8_volatile(DATA_OFF, *b);
        }
        Poll::Ready(Ok(buf.len()))
    }
}

const DATA_OFF: usize = 0x00;
const HAS_DATA: u8 = 0x01;
const LINE_STATUS_OFF: usize = 0x05;
*/
