use core::fmt::{ self, Write };
use core::cell::{ Cell };
use core::time::{ Duration };
use crate::collection::deque::{ Deque };
use crate::cmd::{ Queue };
use crate::cmd::rw::{ Request, Response, Error };
use toolkit_unsafe::{ RawBuf };

pub trait Time {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Write + Timer { }

pub struct RuntimeInner<T, RW, const LOGLEN: usize, const BUFLEN: usize> {
    logbuf: Deque<char, LOGLEN>,
    timer: T,
    rwbufbuf: Deque<RawBuf, BUFLEN>,
    rw: RW,
}

impl<T, RW, const LOGLEN: usize, const BUFLEN: usize>
RuntimeInner<T, RW, LOGLEN, BUFLEN>
{
    pub fn new(timer: T, rw: RW, rwbuf: Deque<RawBuf, BUFLEN>) -> Self {
        Self {
            logbuf: Deque::default(),
            timer: timer,
            rw: rw, rwbufbuf: rwbuf,
        }
    }
}

impl<T, RW, const LOGLEN: usize, const BUFLEN: usize>
RuntimeInner<T, RW, LOGLEN, BUFLEN>
where T: Timer {
    fn time(&mut self) -> Duration {
        self.timer.time()
    }
}

impl<T, RW, const LOGLEN: usize, const BUFLEN: usize>
Write for RuntimeInner<T, RW, LOGLEN, BUFLEN>
where RW: Queue<Request=Request, Response=Response, Error=Error> {
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

#[derive(Clone, Copy)]
pub struct RuntimeCell<'a, T, RW, const LOGLEN: usize, const BUFLEN: usize> {
    cell: Cell<&'a RuntimeInner<T, RW, LOGLEN, BUFLEN>>,
}

impl<'a, T, RW, const LOGLEN: usize, const BUFLEN: usize>
Time for RuntimeCell<'_, T, RW, LOGLEN, BUFLEN> {
    fn time(&mut self) -> Duration {
        let time;
        self.cell.update(|rt| {
            time = rt.time();
            rt
        });
        time
    }
}

impl<'a, T, RW, const LOGLEN: usize, const BUFLEN: usize>
Write for RuntimeCell<'_, T, RW, LOGLEN, BUFLEN> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let res;
        self.cell.update(|rt| {
            res = rt.write_str(s);
            rt
        })
        res
    }
}
