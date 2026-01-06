use toolkit_unsafe::{ IPCByteBuf };

pub enum Request {
    Read(IPCByteBuf),
    Write(IPCByteBuf),
}

pub enum Response {
    Ok,
}

pub enum Error {
    Fatal,
}
