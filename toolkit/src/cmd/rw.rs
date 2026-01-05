use toolkit_unsafe::{ RawBuf };

#[derive(Clone, Copy)]
pub enum Request {
    Read(RawBuf),
    Write(RawBuf),
}

#[derive(Clone, Copy)]
pub enum Response {
    Ok,
}

pub enum Error {
    Fatal,
}
