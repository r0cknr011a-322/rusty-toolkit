use crate::collection::deque::{ Deque };
use toolkit_unsafe::{ RawBuf };

#[derive(Default)]
pub enum RWRequest {
    #[default]
    Noop,
    Read(RawBuf),
    Write(RawBuf),
}

#[derive(Default)]
pub enum RWResponse {
    #[default]
    Ok,
}

#[derive(Default)]
pub enum RWError {
    #[default]
    Fatal,
}

pub trait RWQueue<Request=Request, Response=&Response, Error=Error> {
    fn push(&mut self, req: Request) -> Poll<Result<(), Error>> {
        self.push(req)
    }

    fn pop(&mut self) -> Poll<Result<&Response, Error>> {
        self.pop()
    }
}
