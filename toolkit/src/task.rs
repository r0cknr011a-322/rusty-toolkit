pub mod bytequeue;

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

pub trait Pipe {
    type Req;
    type Rsp;
    type Err;

    fn push(&mut self, data: &[&mut RawByteBuf], req: Self::Req) -> Poll<Result<(), Self::Err>>;
    fn pop(&mut self, data: &[&mut RawByteBuf]) -> Poll<Result<Self::Rsp, Self::Err>>;
}
