use crate::collection::deque::{ Deque };

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Queue<Request> {
    type Response;
    type Error;
    type Task: Task<Response = Self::Response, Error = Self::Error>;

    fn poll_queue(&mut self) -> Poll<Result<(), Self::Error>>;
    fn queue(&mut self, req: Request) -> Self::Task;
}

pub trait Task {
    type Response;
    type Error;

    fn poll(&mut self) -> Poll<Result<Self::Response, Self::Error>>;
}

pub enum RWRequest<'a, const LEN: usize> {
    Read(Deque<&'a mut [u8], LEN>),
    Write(Deque<&'a [u8], LEN>),
}

pub enum RWResponse {
    Ok,
}

pub enum RWError {
    Fatal,
}
