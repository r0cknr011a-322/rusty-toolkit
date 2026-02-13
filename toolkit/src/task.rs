pub mod bytequeue;
pub mod pipe;

use crate::bytebuf::{ RawByteBuf };

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq)]
pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Session {
    type Arg;
    type Err;

    fn init(&mut self, arg: Self::Arg) -> Poll<Result<(), Self::Err>>;
    fn exit(&mut self) -> Poll<()>;
}

#[derive(PartialEq, Eq)]
pub enum ByteQueueErr {
    Fatal,
}

pub trait SendByteQueue {
    fn send(&mut self, data: &[u8]) -> Poll<Result<usize, ByteQueueErr>>;
}

pub trait RecvByteQueue {
    fn recv(&mut self, data: &mut [u8]) -> Poll<Result<usize, ByteQueueErr>>;
}
