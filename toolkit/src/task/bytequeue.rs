use crate::collection::deque::{ Deque };
use crate::bytebuf::{ RawByteBuf };
use crate::task::{ Pipe, Poll };
use core::array::{ from_fn };

pub enum ByteQueueErr {
    Fatal,
}

pub trait SendByteQueue {
    fn send(&mut self, data: &[u8]) -> Poll<Result<usize, ByteQueueErr>>;
}

pub trait RecvByteQueue {
    fn recv(&mut self, data: &mut [u8]) -> Poll<Result<usize, ByteQueueErr>>;
}

/*
 * implementation
 */
pub struct ByteQueue<'a, P, const L: usize, const D: usize> {
    pipe: P,
    buf: Deque<u8, L>,
    data: [RawByteBuf<'a>; D],
}

impl<'a, P, const L: usize, const D: usize>
ByteQueue<'a, P, L, D> {
    pub fn new<C>(pipe: P, databufctr: C) -> Self
    where C: FnMut(usize) -> RawByteBuf<'a> {
        Self {
            pipe: pipe, buf: Deque::default(), data: from_fn(databufctr),
        }
    }
}

impl<'a, P, const L: usize, const D: usize>
RecvByteQueue for ByteQueue<'a, P, L, D>
where P: Pipe<Req=usize, Rsp=&'a [u8], Err=PipeErr> {
    fn recv(&mut self, data: &mut [u8]) -> Poll<Result<usize, ByteQueueErr>> {
        if data.len() <= self.buf.len() {
            let (bufl, bufr) = self.buf.as_slices();
            if data.len() <= bufl.len() {
                let (tocopy, _) = bufl.split_at(data.len());
                data.copy_from_slice(tocopy);
                return Poll::Ready(Ok(data.len()));
            }

            let (datal, datar) = data.split_at_mut(bufl.len());
            datal.copy_from_slice(bufl);
            let (tocopy, _) = bufr.split_at(datar.len());
            datar.copy_from_slice(tocopy);
        }

        Poll::Ready(Err(ByteQueueErr::Fatal))
    }
}

impl<'a, P, const L: usize, const D: usize>
SendByteQueue for ByteQueue<'a, P, L, D>
where P: Pipe<Req=&'a [u8], Rsp=usize, Err=PipeErr> {
    fn send(&mut self, data: &[u8]) -> Poll<Result<usize, ByteQueueErr>> {
        Poll::Ready(Err(ByteQueueErr::Fatal))
    }
}

pub enum PipeErr {
    Fatal,
    Timeout,
}
