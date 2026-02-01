#![no_std]
#![no_main]

use core::panic::{ PanicInfo };
use core::fmt::{ Write };
use core::time::{ Duration };
use toolkit::runtime::{ RuntimeMain, Time, Runtime };
use toolkit::bytebuf::{ RawByteBuf, VolatileByteBuf };
use toolkit::cmd::{ Queue, Poll };
use toolkit::cmd::rw::{ Response as RWRsp, Error as RWErr };
use toolkit::virtio::char::{ CharDevDrv };

use core::arch::global_asm;
global_asm!(include_str!("trap.S"));

struct Timer { }

impl Time for Timer {
    fn time(&mut self) -> Duration {
        Duration::default()
    }
}

const IPC_BUF_BUF_NR: usize = 8;
const IPCBufBuf: [(usize, usize); IPC_BUF_BUF_NR] = [
    (0x8001_8000, 0x1000),
    (0x8001_8000, 0x1000),
    (0x8001_8000, 0x1000),
    (0x8001_8000, 0x1000),
    (0x8001_8000, 0x1000),
    (0x8001_8000, 0x1000),
    (0x8001_8000, 0x1000),
    (0x8001_8000, 0x1000),
];

const DEV_MEM_BUF_NR: usize = 8;
const DevMemBuf: [(usize, usize); DEV_MEM_BUF_NR] = [
    (0x1000_1000, 0x1000),
    (0x1000_2000, 0x1000),
    (0x1000_3000, 0x1000),
    (0x1000_4000, 0x1000),
    (0x1000_5000, 0x1000),
    (0x1000_6000, 0x1000),
    (0x1000_7000, 0x1000),
    (0x1000_8000, 0x1000),
];

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

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let ipcbufctr = |idx| -> RawByteBuf<'_> {
        let (addr, len) = IPCBufBuf[idx];
        RawByteBuf::new(addr, len)
    };
    let devmemctr = |idx| -> RawByteBuf<'_> {
        let (addr, len) = DevMemBuf[idx];
        RawByteBuf::new(addr, len)
    };

    let runtime = RuntimeMain::<Timer, RTQueue, IPC_BUF_BUF_NR, DEV_MEM_BUF_NR, 0x1000, 4>::new(
        Timer { }, RTQueue { }, ipcbufctr, devmemctr
    );

    let rtref = &runtime;
    let Some(mut log0) = rtref.log(0) else {
        loop { }
    };

    let mut chardevdrv = CharDevDrv::new(rtref, 7);
    let magic = chardevdrv.get_magic();
    let version = chardevdrv.get_version();
    let id = chardevdrv.get_id();
    chardevdrv.reset();
    let msg = "hello world!!!\n";
    chardevdrv.emerg_wr(msg.as_bytes());
    writeln!(log0, "magic: {:X}; version: {:X}", magic, version);

    loop { }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
