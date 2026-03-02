use crate::extbytebuf::{ ExtByteBuf, VolatileByteBuf };
use crate::collection::asynque::{ Asynque, Poll };
use crate::collection::byteblock::{ ByteBlock };
use crate::io::{ Error as IOErr };


pub struct Uart16550<'a, const L: usize> {
    regbuf: ExtByteBuf<'a>,
}

impl<'a, const L: usize> Uart16550<'a, L> {
    pub fn new(regbuf: ExtByteBuf<'a>) -> Self {
        Self {
            regbuf: regbuf,
        }
    }
}

impl<'a, const L: usize> Asynque for Uart16550<'a, L> {
    type Req = ByteBlock<L>;
    type Rsp = ();
    type Err = IOErr;

    fn try_push(&mut self, block: ByteBlock<L>) -> Poll<Result<(), IOErr>> {
        for byte in block.rd_slice() {
            self.regbuf.wr8_volatile(0x00, *byte);
        }
        Poll::Ready(Ok(()))
    }

    fn poll_push(&mut self) -> Poll<Result<(), IOErr>> {
        Poll::Ready(Ok(()))
    }

    fn try_pop(&mut self) -> Poll<Result<(), IOErr>> {
        Poll::Ready(Err(IOErr::Fatal))
    }

    fn poll_pop(&mut self) -> Poll<Result<(), IOErr>> {
        Poll::Ready(Err(IOErr::Fatal))
    }
}
