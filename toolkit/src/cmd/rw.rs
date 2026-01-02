use crate::cmd::{ CmdQueue, Poll };
use toolkit_unsafe::{ RawBuf };

#[derive(Default)]
pub enum Request {
    #[default]
    Noop,
    Read(RawBuf),
    Write(RawBuf),
}

#[derive(Default)]
pub enum Response {
    #[default]
    Ok,
}

#[derive(Default)]
pub enum Error {
    #[default]
    Fatal,
}

pub trait Queue: CmdQueue<Request=Request, Response=&Response, Error=Error> {
    fn push(&mut self, req: Request) -> Poll<Result<(), Error>> {
        self.push(req)
    }

    fn pop(&mut self) -> Poll<Result<&Response, Error>> {
        self.pop()
    }
}
