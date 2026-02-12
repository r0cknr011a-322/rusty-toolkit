use crate::collection::deque::{ Deque };
use crate::bytebuf::{ RawByteBuf };
use crate::task::{ Poll };
use crate::task::pipe::{ Pipe };
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
pub struct PipeHead<'a, P, const D: usize, const R: usize, const W: usize> {
    pipe: P,
    data: [RawByteBuf<'a>; D],
    rdbuf: Deque<u8, R>,
    wrbuf: Deque<u8, W>,
}

impl<'a, P, const D: usize, const R: usize, const W: usize>
PipeHead<'a, P, D, R, W> {
    pub fn new<C>(pipe: P, databufctr: C) -> Self
    where C: FnMut(usize) -> RawByteBuf<'a> {
        Self {
            pipe: pipe, data: from_fn(databufctr),
            rdbuf: Deque::default(), wrbuf: Deque::default(),
        }
    }

    fn rdbuf_flush(&mut self, data: &mut [u8]) -> usize {
        if data.len() < self.rdbuf.len() {
            let (bufl, bufr) = self.rdbuf.as_slices();
            if data.len() <= bufl.len() {
                let (tocopy, _) = bufl.split_at(data.len());
                data.copy_from_slice(tocopy);
                return data.len();
            }

            let (datal, datar) = data.split_at_mut(bufl.len());
            datal.copy_from_slice(bufl);
            let (tocopy, _) = bufr.split_at(datar.len());
            datar.copy_from_slice(tocopy);
            return data.len();
        }

        let (fromcopy, _) = data.split_at_mut(self.rdbuf.len());
        fromcopy.copy_from_slice(self.rdbuf);
        self.rdbuf.len()
    }
}

impl<'a, P, const D: usize, const R: usize, const W: usize>
RecvByteQueue for PipeHead<'a, P, D, R, W>
where P: Pipe<Req=usize, Rsp=&'a [u8], Err=PipeErr> {
    fn recv(&mut self, data: &mut [u8]) -> Poll<Result<usize, ByteQueueErr>> {
        let flushed = self.rdbuf_flush(data);
        for _ in 0..flushed {
            let _ = self.rdbuf.pop();
        }

        if flushed == data.len() {
            return Poll::Ready(Ok(flushed));
        }

        Poll::Ready(Err(ByteQueueErr::Fatal))
    }
}

impl<'a, 'b, P, const D: usize, const R: usize, const W: usize>
SendByteQueue for PipeHead<'a, P, D, R, W>
where P: Pipe<Req=&'a [u8], Rsp=usize, Err=PipeErr> {
    fn send(&mut self, data: &[u8]) -> Poll<Result<usize, ByteQueueErr>> {
        /*
        if !self.wrready {
            let Poll::Ready(res) = self.pipe.push(&mut self.data, data) else {
                return Poll::Pending;
            };
            let Ok(()) = res else {
                return Poll::Ready(Err(ByteQueueErr::Fatal));
            };
            self.wrready = true;
        }
        let Poll::Ready(res) = self.pipe.pop(&mut self.data) else {
            return Poll::Pending;
        };
        let Ok(cnt) = res else {
            return Poll::Ready(Err(ByteQueueErr::Fatal));
        };
        Poll::Ready(Ok(cnt))
        */
        Poll::Ready(Err(ByteQueueErr::Fatal))
    }
}

pub enum PipeErr {
    Fatal,
    Timeout,
}
