use crate::collection::deque::{ Deque };
use toolkit_unsafe::{ RawBuf };

#[derive(Default, Clone, Copy)]
pub enum Cmd {
    #[default]
    Noop,
    Read(RawBuf),
    Write(RawBuf),
}

#[derive(Default, Clone, Copy)]
pub struct Request<const LEN: usize> {
    deque: Deque<Cmd, LEN>,
}

impl<const LEN: usize> Request<LEN> {
    pub fn push(&mut self, cmd: Cmd) {
        if let Cmd::Noop = cmd {
            return;
        }
        if self.deque.is_full() {
            let _ = self.deque.pop_front();
        }
        self.deque.push_back(cmd);
    }

    pub fn pop(&mut self) -> Option<Cmd> {
        self.deque.pop_front()
    }
}

#[derive(Default, Clone, Copy)]
pub enum Status {
    #[default]
    Ok,
}

#[derive(Default, Clone, Copy)]
pub struct Response<const LEN: usize> {
    deque: Deque<Status, LEN>,
}

impl<const LEN: usize> Response<LEN> {
    pub fn push(&mut self, status: Status) {
        if self.deque.is_full() {
            let _ = self.deque.pop_front();
        }
        self.deque.push_back(status);
    }
    pub fn pop(&mut self) -> Option<Status> {
        self.deque.pop_front()
    }
}

pub enum Error {
    Fatal,
}
