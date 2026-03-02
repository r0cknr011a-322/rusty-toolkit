pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub enum Error {
    Fatal,
}

pub trait SendByteChan {
    fn try_send(&mut self, data: &[u8]) -> Poll<Result<usize, Error>>;
    fn poll_send(&mut self) -> Poll<Result<(), Error>>;
}

pub trait RecvByteChan {
    fn try_recv(&mut self, data: &mut [u8]) -> Poll<Result<usize, Error>>;
    fn poll_recv(&mut self) -> Poll<Result<(), Error>>;
}
