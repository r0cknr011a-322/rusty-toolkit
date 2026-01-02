use core::fmt::{ self, Write };
use core::cell::{ Cell };
use core::time::{ Duration };
use crate::collection::{ Deque, String };
use crate::cmd::rw::{ Queue };
use toolkit_unsafe::{ RawBuf };

const LOG_CHAN_NAME_LEN: usize =
    option_env!("LOG_CHAN_NAME_LEN")
    .unwrap_or(16);
const LOG_CHAN_NAME_DEFAULT: &str =
    option_env!("LOG_CHAN_NAME_DEFAULT")
    .unwrap_or("void-chan");

pub trait Time {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Write + Time { }

pub struct RuntimeInner <T, Q,
const CHANLEN: usize, const CHANNR: usize, const BUFLEN: usize>
{
    logchanbuf: Deque<LogChan<CHANLEN>, CHANNR>,
    timer: T,
    rwqueue: Q,
    rwbufqueue: Deque<RawBuf, BUFLEN>,
}

impl<T, RW,
const CHANLEN: usize, const CHANNR: usize, const BUFLEN: usize>
RuntimeInner<T, RW, CHANLEN, CHANNR, BUFLEN>
{
    pub fn new<'a, ChanCtr: FnMut(usize) -> &'a str>(
        timer: T,
        rwcmd: RW, rwdata: Deque<RawBuf, BUFLEN>,
        chanmap: ChanCtr
    ) -> Self {
        let logchanbuf = Deque::new(|idx| {
            LogChan::new(chanmap(idx));
        });
        Self {
            timer: timer,
            rwqueue: rwcmd, rwbufqueue: rwdata,
            logchanbuf: logchanbuf,
        }
    }
}

impl<T: Time, Q,
const CHANLEN: usize, const CHANNR: usize, const BUFLEN: usize>
RuntimeInner<T, Q, CHANLEN, CHANNR, BUFLEN> {
    fn time(&mut self) -> Duration {
        self.timer.time()
    }
}

impl<T, Q: Queue,
const CHANLEN: usize, const CHANNR: usize, const BUFLEN: usize>
Write for RuntimeInner<T, Q, CHANLEN, CHANNR, BUFLEN> {
    fn write_str(&mut self, _s: &str) -> Result<(), fmt::Error> {
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
pub struct RuntimeCell<'a, T, Q,
const CHANLEN: usize, const CHANNR: usize, const BUFLEN: usize> {
    chan: String<LOG_CHAN_NAME_LEN>,
    cell: Cell<&'a RuntimeInner<T, Q, CHANLEN, CHANNR, BUFLEN>>,
}

impl<'a, T, Q,
const CHANLEN: usize, const CHANNR: usize, const BUFLEN: usize>
RuntimeCell<'_, T, Q, CHANLEN, CHANNR, BUFLEN> {
    pub fn find(rt: &RuntimeInner<T, Q, CHANLEN, CHANNR, BUFLEN>, name: &str) -> Option<Self> {
        let namestr = String::new(name);
        let Some(chan) = rt.logchanbuf.find(|&chan| namestr == chan.name) else {
            return None;
        };
        Some(Self {
            chan: namestr,
            cell: Cell::new(rt),
        })
    }
}

impl<'a, T: Time, Q,
const CHANLEN: usize, const CHANNR: usize, const BUFLEN: usize>
Time for RuntimeCell<'_, T, Q, CHANLEN, CHANNR, BUFLEN> {
    fn time(&mut self) -> Duration {
        let mut time = Duration::default();
        self.cell.update(|rt| {
            time = rt.time();
            rt
        });
        time
    }
}

impl<'a, T, Q,
const CHANLEN: usize, const CHANNR: usize, const BUFLEN: usize>
Write for RuntimeCell<'_, T, Q, CHANLEN, CHANNR, BUFLEN> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let res;
        self.cell.update(|rt| {
            res = rt.write_str(s);
            rt
        })
        res
    }
}

struct LogChan<const LEN: usize> {
    name: String<LOG_CHAN_NAME_LEN>,
    buf: Deque<u8, LEN>,
}

impl<const LEN: usize> LogChan<LEN> {
    fn new(name: &str) -> Self {
        Self {
            name: String::new(name),
            buf: Deque::default(),
        }
    }
}
