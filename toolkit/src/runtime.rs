use core::fmt::{ self, Write };
use core::time::{ Duration };
use crate::collection::{ Deque };
use crate::cmd::{ Buf };
use crate::cmd::rw::{ Request };

pub trait Timer {
    fn time(&mut self) -> Duration;
}

pub struct Runtime<T, RW, const LOG_LEN: usize, const BUF_LEN: usize> {
    logdeque: Deque<char, LOG_LEN>,
    timer: T,
    bufdeque: Deque<RawBuf, BUF_LEN>,
    out: RW,
}

impl<T, RW, const LOGL: usize, const BUFL: usize> Runtime<Timer, RW, LOGL, BUFL> {
    pub fn new(timer: T, rw: RW, bufdeque: Deque<RawBuf, LEN>) -> Self {
        Self {
            logdeque: Deque::default(),
            timer: timer, rw: rw,
            bufdeque: bufdeque,
        }
    }

}

impl<T, RW, const LOGL: usize, const BUFL: usize> Runtime<Timer, RW, LOGL, BUFL>
where T: Timer {
    pub fn time(&mut self) -> Duration {
        self.timer.time()
    }
}

impl<Timer, RW, const LOGL: usize, const BUFL: usize> Write for Runtime<Timer, RW, LOGL, BUFL>
where RW: Buf, RW::Request: Request, RW::Response: Response, RW::Error: Error {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        Err(fmt::Error)
/*
        if self.deque.capacity() - self.deque.len() < s.len() {
            return Err(fmt::Error);
        }
        s.chars().for_each(|item| {
            self.deque.push_back(item);
        });
*/
    }
}
