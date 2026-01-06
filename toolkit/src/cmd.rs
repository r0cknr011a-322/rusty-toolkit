pub mod rw;

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Queue {
    type Request;
    type Response;
    type Error;

    fn push<R: Request>(&mut self, req: Self::Request) -> Poll<Result<(), Self::Error>>;
    fn pop(&mut self) -> Poll<Result<Self::Response, Self::Error>>;
}
