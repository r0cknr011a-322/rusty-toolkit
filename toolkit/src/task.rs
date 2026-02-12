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
