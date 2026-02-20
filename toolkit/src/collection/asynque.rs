pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Asynque {
    type Req;
    type Rsp;
    type Err;

    fn try_push(&mut self, req: Self::Req) -> Poll<Result<(), Self::Err>>;
    fn poll_push(&mut self) -> Poll<Result<(), Self::Err>>;

    fn try_pop(&mut self) -> Poll<Result<Self::Rsp, Self::Err>>;
    fn poll_pop(&mut self) -> Poll<Result<(), Self::Err>>;
}
