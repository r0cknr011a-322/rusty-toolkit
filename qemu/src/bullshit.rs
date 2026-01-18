#![no_std]
#![no_main]

use core::panic::{ PanicInfo };
use core::fmt::{ Write };
use toolkit::runtime::{ RuntimeMain };
use toolkit::cmd::{ Buf, Poll, Error };
use toolkit_unsafe::{ IPCByteBuf };

use core::arch::global_asm;
global_asm!(include_str!("trap.S"));


fn some_one<const L: usize>(mut logger: LogChan<L>) {
    writeln!(logger, "some_one some_one some_one some_one");
}

fn some_two<const L: usize>(mut logger: LogChan<L>) {
    writeln!(logger, "some_two some_two some_two some_two");
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let bufdeque = Deque::<RawBuf, 8>::new(|idx| {
        
    });
    let runtime_inner = RuntimeInner::new(&logcell);

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
