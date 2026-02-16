pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub enum Error {
    Fatal,
}

pub trait Session {
    type Arg;
    type Buf: ByteChan;

    fn init(&mut self, arg: Self::Arg) -> Poll<Result<Self::Buf, Error>>;
    fn exit(&mut self) -> Poll<()>;
}

pub trait ByteChan {
    fn send(&mut self, data: &[u8]) -> Poll<Result<usize, Error>>;
    fn recv(&mut self, data: &mut [u8]) -> Poll<Result<usize, Error>>;
}
