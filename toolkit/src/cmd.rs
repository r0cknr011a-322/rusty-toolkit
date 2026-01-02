pub mod rw;

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait CmdQueue {
    type Request;
    type Response;
    type Error;

    fn push(&mut self, req: Self::Request) -> Poll<Result<(), Self::Error>>;
    fn pop(&mut self) -> Poll<Result<Self::Response, Self::Error>>;
}
