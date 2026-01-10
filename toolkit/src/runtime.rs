use core::fmt::{ self, Write };
use core::cell::{ Cell };
use core::time::{ Duration };
use crate::collection::deque::{ Deque };
use crate::collection::string::{ String };
use crate::cmd::{ Queue };
use toolkit_unsafe::{ IPCByteBuf };

pub trait Time {
    fn time(&mut self) -> Duration;
}

pub trait Runtime: Write + Time { }

struct LogChan<const N: usize, const D: usize> {
    name: String<N>,
    data: Deque<u8, D>,
}

impl<const N: usize, const D: usize> LogChan<N, D> {
    fn new(name: &str) -> Self {
        Self {
            name: String::new(name),
            data: Deque::default(),
        }
    }
}

impl<const N: usize, const D: usize>
Write for LogChan<N, D> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if self.data.is_full() {
            for _ in 0..s.len() {
                self.data.pop_front();
            }
        }
        for b in s.as_bytes() {
            self.data.push_back(*b);
        }
        Ok(())
    }
}

struct LogChanBuf<const CHN: usize, const CHL: usize, const CHNR: usize> {
    deque: Deque<LogChan<CHN, CHL>, CHNR>,
}

impl<const CHN: usize, const CHL: usize, const CHNR: usize>
IntoIterator for LogChanBuf<CHN, CHL, CHNR> {
    type Item = LogChan<CHN, CHL>;
    type IntoIter = DequeIter<LogChan<>CHNR>;
}

impl<const CHN: usize, const CHL: usize, const CHNR: usize>
LogChanBuf<CHN, CHL, CHNR> {
    fn new<C: FnMut(usize) -> &'static str>(mut ctr: C) -> Self {
        Self {
            deque: Deque::new(|idx| LogChan::new(ctr(idx))),
        }
    }
}

/*
 * runtime channel
 */
#[derive(Clone, Copy)]
pub struct RuntimeChan
<'a, T, Q, const BUFNR: usize,
const CHN: usize, const CHL: usize, const CHNR: usize> {
    name: String<CHN>,
    rt: &'a RuntimeMain<T, Q, BUFNR, CHN, CHL, CHNR>,
}

impl<'a, T, Q, const BUFNR: usize,
const CHN: usize, const CHL: usize, const CHNR: usize>
RuntimeChan<'a, T, Q, BUFNR, CHN, CHL, CHNR> {
}

/*
 * runtime
 */
pub struct RuntimeMain
<T, Q, const BUFNR: usize, const CHN: usize, const CHL: usize, const CHNR: usize> {
    timer: T,
    queue: Q,
    bufbuf: Deque<IPCByteBuf, BUFNR>,
    logchanbuf: LogChanBuf<CHN, CHL, CHNR>,
}

impl<T, Q, const BUFNR: usize,
const CHN: usize, const CHL: usize, const CHNR: usize>
RuntimeMain<T, Q, BUFNR, CHN, CHL, CHNR> {
    pub fn new<C: FnMut(usize) -> &'static str,
               B: FnMut(usize) -> IPCByteBuf>
        (timer: T, queue: Q, mut chanctr: C, mut bufctr: B) -> Self {
        Self {
            timer: timer, queue: queue,
            bufbuf: Deque::new(|idx| bufctr(idx)),
            logchanbuf: LogChanBuf::new(|idx| chanctr(idx)),
        }
    }

    pub fn chan(&self, name: &'static str) ->
        Option<RuntimeChan<T, Q, BUFNR, CHN, CHL, CHNR>> {
        for chan in self.logchanbuf {
            if chan.name == name {
                return None; 
            }
        }
        None
    }
}

/*
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
pub struct RuntimeChan<'a, T, Q,
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
            time = *rt.time();
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
*/
