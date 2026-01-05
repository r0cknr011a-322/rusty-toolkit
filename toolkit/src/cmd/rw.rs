use toolkit_unsafe::{ IPCByteBuf };

#[derive(Clone, Copy)]
pub enum Request {
    Read(IPCByteBuf),
    Write(IPCByteBuf),
}

#[derive(Clone, Copy)]
pub enum Response {
    Ok,
}

pub enum Error {
    Fatal,
}
