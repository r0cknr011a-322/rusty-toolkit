#[derive(PartialEq, Eq)]
pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Queue {
    type Request;
    type Response;
    type Error;

    fn push(&mut self, req: Self::Request) -> Poll<Result<(), Self::Error>>;
    fn pop(&mut self) -> Poll<Result<Self::Response, Self::Error>>;
}

pub trait Session {
    type Arg;
    type Error;

    fn init(&mut self, arg: Self::Arg) -> Poll<Result<(), Self::Error>>;
    fn exit(&mut self) -> Poll<()>;
}

#[derive(PartialEq, Eq)]
pub enum GenRsp {
    Ok,
}

#[derive(PartialEq, Eq)]
pub enum GenErr {
    Fatal,
    Timeout,
}
