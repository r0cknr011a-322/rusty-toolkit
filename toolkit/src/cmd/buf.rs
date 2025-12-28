pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Buf<Request> {
    type Response;
    type Error;
    type Task: Task<Response = Self::Response, Error = Self::Error>;

    fn push(&mut self, req: Request) -> Poll<Result<Self::Task; Self::Error>>;
}

pub trait Task {
    type Response;
    type Error;

    fn pop(&mut self) -> Poll<Result<Self::Response, Self::Error>>;
}
