#![no_std]
#![no_main]

use core::panic::{ PanicInfo };
use core::fmt::{ self, Write };
use core::arch::global_asm;
use rusty_scrapyard_lib::io::{ AsyncWrite, AsyncRead, Poll as IOPoll, Error as IOError };
use rusty_scrapyard_iomem::{ IOBufMem };
use rusty_scrapyard_lib::uart16550::{ UART16550 };

global_asm!(include_str!("trap.S"));

#[derive(Copy, Clone, Default)]
struct LogChan { }

impl Write for LogChan {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        Ok(())
    }
}

enum Error {
    Fatal,
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let iomem = IOBufMem::new(0x1000_0000, 0x08);
    let mut uart = UART16550::new(iomem);
    let mut data: [u8; 16] = [0; 16];
    while let IOPoll::Pending = uart.poll_read(&mut data[..]) { }

    let msg = "got some bytes";
    while let IOPoll::Pending = uart.poll_write(msg.as_bytes()) { }

    let msg = "
        hello world! hello world! hello world! hello world!
        hello world! hello world! hello world! hello world!
    ";
    while let IOPoll::Pending = uart.poll_write(msg.as_bytes()) { }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
