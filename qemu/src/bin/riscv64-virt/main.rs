#![no_std]
#![no_main]

use core::panic::{ PanicInfo };
use core::fmt::{ Write };
use core::time::{ Duration };
use toolkit::runtime::{ RuntimeMain, Time, Runtime };
use toolkit::bytebuf::{ RawByteBuf, VolatileByteBuf };
use toolkit::task::{ Session, Queue, Poll, GenRsp, GenErr };
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

struct RTQueue<'a> {
    output: RawByteBuf<'a>,
}

impl<'a>
Queue for RTQueue {
    type Request = &'a [u8];
    type Response = GenRsp;
    type Error = GenErr;

    fn push(&mut self, req: u8) -> Poll<Result<(), GenErr>> {
        Poll::Ready(Ok(()))
    }

    fn pop(&mut self) -> Poll<Result<GenRsp, GenErr>> {
        Poll::Ready(Ok(GenRsp::Ok))
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

    let mut chardevdrv = CharDevDrv::new(rtref,
        RawByteBuf::new(0x1000_8000, 0x1000),
        RawByteBuf::new(0x8001_0000, 0x1000),
        RawByteBuf::new(0x8001_1000, 0x1000),
        RawByteBuf::new(0x8001_2000, 0x1000),
    );

    let init = loop {
        if let Poll::Ready(rdy) = chardevdrv.init(()) {
            break rdy;
        }
    };

    if let Ok(()) = init {
        writeln!(log0, "init success!!!");
    }

    let msg = "hello world!!!\n";
    chardevdrv.emerg_wr(msg.as_bytes());

    loop { }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
