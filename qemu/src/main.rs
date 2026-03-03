#![no_std]
#![no_main]

use core::panic::{ PanicInfo };
use core::fmt::{ Write };
use core::array::{ self };
use core::time::{ Duration };

use toolkit::runtime::{ RuntimeInner, RuntimeRef, Timer, Runtime };
use toolkit::extbytebuf::{ ExtByteBuf, VolatileByteBuf };
use toolkit::io::{ SendByteChan, Poll, Error };
use toolkit::uart16550::{ Uart16550 };
use toolkit::virtio::serial::{ SerialDevice, VirtQueue };

use core::arch::global_asm;
global_asm!(include_str!("trap.s"));

struct VirtTimer { }

impl Timer for VirtTimer {
    fn time(&mut self) -> Duration {
        Duration::default()
    }
}

const LOG_BUF_LEN: usize = 0x1000;

struct LoggerTask<'a> {
    uart: Uart16550<'a>,
    runtime: RuntimeInner<VirtTimer, 4, LOG_BUF_LEN>,
}

impl<'a> Default for LoggerTask<'a> {
    fn default() -> Self {
        Self {
            runtime: RuntimeInner::new(VirtTimer { }),
            uart: Uart16550::new((0x1000_0000, 8)),
        }
    }
}

impl<'a> LoggerTask<'a> {
    fn chan(&self, idx: usize) -> RuntimeRef<VirtTimer, 4, LOG_BUF_LEN> {
        let Some(chan) = self.runtime.chan(0) else {
            panic!("logger get channel failed");
        };
        chan
    }

    fn run(&mut self) {
        let mut buf: [u8; LOG_BUF_LEN] = array::from_fn(|_| 0);
        let rd = self.runtime.rd_log(0, &mut buf);

        let mut total = 0;
        let mut wr = 0;
        for _ in 0..4 {
            let Poll::Ready(res) = self.uart.try_send(&buf[total..rd]) else {
                continue;
            };
            let Ok(wr) = res else {
                continue;
            };

            total += wr;
            if total >= buf.len() {
                break;
            }
        }
    }
}

const EXT_MEM: usize = 0x80200000;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let mut logger = LoggerTask::default();

    let mut log0 = logger.chan(0);
    writeln!(log0, "hello world!!!");

    logger.run();

    let rdqueue = VirtQueue::new(
        (EXT_MEM + 0x0000, 0x1000),
        (EXT_MEM + 0x1000, 0x1000),
        (EXT_MEM + 0x2000, 0x1000),
    );
    let wrqueue = VirtQueue::new(
        (EXT_MEM + 0x3000, 0x1000),
        (EXT_MEM + 0x4000, 0x1000),
        (EXT_MEM + 0x5000, 0x1000),
    );

    let mut serial0 = SerialDevice::new(
        logger.chan(3), (0x1000_8000, 0x1000),
        (EXT_MEM + 0x6000, 0x1000), (EXT_MEM + 0x7000, 0x1000),
        rdqueue, wrqueue,
    );

    logger.run();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
