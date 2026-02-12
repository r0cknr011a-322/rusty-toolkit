use crate::task::{ Poll };
use crate::bytebuf::{ RawByteBuf };

pub trait Pipe {
    type Req;
    type Rsp;
    type Err;

    fn push(&mut self, data: &mut [RawByteBuf], req: Self::Req) -> Poll<Result<(), Self::Err>>;
    fn pop(&mut self, data: &mut [RawByteBuf]) -> Poll<Result<Self::Rsp, Self::Err>>;
}
