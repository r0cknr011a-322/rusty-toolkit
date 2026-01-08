use crate::cmd::{ Queue };
use toolkit_unsafe::{ IPCByteBuf };

pub trait BufRefQueue: Queue {
    type Request = IPCByteBuf;
    type Response = Response;
    type Error = Error;

    fn push(&mut self, req: Self::Request) -> Poll<Result<(), Self::Error>> {
        self.push(req)
    }

    fn pop(&mut self) -> Poll<Result<Self::Response, Self::Error>> {
        self.pop()
    }
}

pub enum Response {
    Ok,
}

pub enum Error {
    Fatal,
}
