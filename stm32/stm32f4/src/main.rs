#![no_std]
#![no_main]

use core::arch::global_asm;
global_asm!(include_str!("trap.S"));

use core::panic::{ PanicInfo };
use core::time::{ Duration };
use core::fmt::{ Write };
use toolkit::runtime::{ Time, RuntimeMain };
use toolkit::cmd::{ Queue, Poll };
use toolkit::cmd::rw::{ Response as RWRsp, Error as RWErr };
use toolkit_unsafe::{ IPCByteBuf };

#[derive(Clone, Copy)]
struct Timer { }

impl Time for Timer {
    fn time(&mut self) -> Duration {
        Duration::new(0, 0)
    }
}

struct RTQueue { }

impl Queue for RTQueue {
    type Request = u8;
    type Response = RWRsp;
    type Error = RWErr;

    fn push(&mut self, req: u8) -> Poll<Result<(), RWErr>> {
        Poll::Ready(Ok(()))
    }

    fn pop(&mut self) -> Poll<Result<RWRsp, RWErr>> {
        Poll::Ready(Ok(RWRsp::Ok))
    }
}

const IPC_BUF_NR: usize = 4;
fn ipcbufctr(idx: usize) -> IPCByteBuf {
    let buf0 = IPCByteBuf::new(0, 256);
    let buf1 = IPCByteBuf::new(0, 256);
    let buf2 = IPCByteBuf::new(0, 256);
    let buf3 = IPCByteBuf::new(0, 256);
    let ipcbufbuf: [IPCByteBuf; IPC_BUF_NR] = [ buf0, buf1, buf2, buf3 ];
    ipcbufbuf[idx]
}

const LOG_CHAN_NR: usize = 8;
fn channamectr(idx: usize) -> &'static str {
    let channamebuf: [&'static str; LOG_CHAN_NR] = [
        "bullshit-channel-0",
        "bullshit-channel-1",
        "bullshit-channel-2",
        "bullshit-channel-3",
        "bullshit-channel-0",
        "bullshit-channel-1",
        "bullshit-channel-2",
        "bullshit-channel-3",
    ];
    channamebuf[idx]
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let runtime = RuntimeMain::<Timer, RTQueue, IPC_BUF_NR, { (1 << 10) }, LOG_CHAN_NR>::new(
        Timer { }, RTQueue { }, channamectr, ipcbufctr,
    );

    let mut chan = runtime.chan("bullshit-channel-3");
    writeln!(chan, "hello world!!!");

    loop { }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    loop { }
}
