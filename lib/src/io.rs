use core::future::{ Future };

pub enum Error {
    Fatal,
    TimedOut,
    Unsupported,
}

pub trait Send {
    fn send(self, buf: &[u8]) -> impl Future<Output = Result<usize, Error>>;
}

pub trait Recv {
    fn recv(self, buf: &mut [u8]) -> impl Future<Output = Result<usize, Error>>;
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait Seek {
    fn seek(self, pos: SeekFrom) -> impl Future<Output = Result<u64, Error>>;
}
