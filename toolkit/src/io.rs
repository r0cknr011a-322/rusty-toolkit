pub enum Error {
    Fatal,
    TimedOut,
    Unsupported,
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait AsyncWrite {
    fn poll_write(&mut self, buf: &[u8]) -> Poll<Result<usize, Error>>;
}

pub trait AsyncRead {
    fn poll_read(&mut self, buf: &mut [u8]) -> Poll<Result<usize, Error>>;
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait AsyncSeek {
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<Result<u64, Error>>;
}
