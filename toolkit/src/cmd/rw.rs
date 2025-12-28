use crate::collection::deque::{ Deque };
use toolkit_unsafe::{ RawBuf };

pub enum Operation {
    Read(RawBuf),
    Write(RawBuf),
}

#[derive(Clone, Copy)]
pub struct Cmd<const LEN: usize> {
    buf: Deque<Operation, LEN>,
}

impl<const L: usize> Cmd<L> {
    pub fn new<F: FnMut(usize) -> I>(f: F) -> Self<L> {
        Self {
            buf: Deque::new(f),
        }
    }
}

fn rw_new(idx: usize) -> Request {
}

pub enum Status {
    Ok,
}

pub enum Error {
    Fatal,
}
