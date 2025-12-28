#![no_std]
#![no_main]

use core::panic::{ PanicInfo };
use core::arch::global_asm;
use core::fmt::{ self, Write };
use core::cell::{ Cell };
use core::array::{ self };
use toolkit::service::{ Service, Poll as IOPoll, Error as IOError };
use toolkit::uart16550::{ UART16550 };
use toolkit_unsafe::{ RawBuf };

global_asm!(include_str!("trap.S"));

#[derive(Default)]
struct LogCell<const LEN: usize> {
    inner: Cell<LogBuf<LEN>>,
}

#[derive(Copy, Clone)]
struct LogChan<'a, const LEN: usize> {
    cell: &'a LogCell<LEN>,
}

impl<'a, const L: usize> LogChan<'a, L> {
    fn new(cell: &'a LogCell<L>) -> Self {
        Self {
            cell: cell,
        }
    }
}

impl<'a, const L: usize> fmt::Write for LogChan<'_, L> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.cell.inner.update(|mut logger| {
            logger.write_str(s);
            logger
        });
        Ok(())
    }
}

enum Error {
    Fatal,
}

fn some_one<const L: usize>(mut logger: LogChan<L>) {
    writeln!(logger, "some_one some_one some_one some_one");
}

fn some_two<const L: usize>(mut logger: LogChan<L>) {
    writeln!(logger, "some_two some_two some_two some_two");
}

fn logchan() -> impl AsyncWrite {
    let iomem = IOBufMem::new(0x1000_0000, 0x08);
    UART16550::new(iomem)
}

const LOG_BUF_LEN: usize = 512;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let logcell = LogCell::<LOG_BUF_LEN>::default();
    let logger = LogChan::new(&logcell);

    some_one(logger);
    some_two(logger);

    let tosend: [u8, LOG_BUF_LEN] = 

    let mut logchan = logchan();
    while let IOPoll::Pending = logchan.poll_write(msg.as_bytes()) { }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
