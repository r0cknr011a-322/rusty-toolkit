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


struct LoggerTask<'a> {
    runtime: &'a RuntimeInner<VirtTimer, LOG_BUF_NR, LOG_BUF_LEN>,
    uart: Uart16550<'a>,
}

impl<'a> LoggerTask<'a> {
    fn new(runtime: &'a RuntimeInner<VirtTimer, LOG_BUF_NR, LOG_BUF_LEN>) -> Self {
        Self {
            runtime: runtime, uart: Uart16550::new((0x1000_0000, 8)),
        }
    }

    fn run(&mut self) {
        let mut buf: [u8; LOG_BUF_LEN] = array::from_fn(|_| 0);
        for idx in 0..LOG_BUF_NR {
            let rd = self.runtime.rd_log(idx, &mut buf);
            if rd == 0 {
                continue;
            }
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
}

const EXT_MEM: usize = 0x80200000;

struct SerialTask<'a, RT> {
    runtime: RT,
    device: SerialDevice<'a, RT>,
}

impl<'a, RT> SerialTask<'a, RT>
where RT: Runtime {
    fn new(runtime: RT, devrt: RT) -> Self {
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
        let device = SerialDevice::new(
            devrt, (0x1000_8000, 0x1000),
            (EXT_MEM + 0x6000, 0x1000), (EXT_MEM + 0x7000, 0x1000),
            rdqueue, wrqueue,
        );
        Self {
            runtime: runtime, device: device,
        }
    }

    fn init(&mut self) {
        if self.device.init().is_err() {
            writeln!(self.runtime, "serial device init failed");
        }
    }
}


const LOG_BUF_NR: usize  = 4;
const LOG_BUF_LEN: usize = 0x1000;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let mut runtime = RuntimeInner::<VirtTimer, LOG_BUF_NR, LOG_BUF_LEN>::new(VirtTimer { });
    let Some(mut main_logchan) = runtime.chan(0) else {
        panic!("runtime get channel 0 failed");
    };
    let Some(mut serial_logchan0) = runtime.chan(3) else {
        panic!("runtime get channel 3 failed");
    };
    let Some(mut serial_logchan1) = runtime.chan(3) else {
        panic!("runtime get channel 3 failed");
    };

    writeln!(main_logchan, "hello world!!!");

    let mut logger = LoggerTask::new(&runtime);
    logger.run();

    let mut serial = SerialTask::new(serial_logchan0, serial_logchan1);
    serial.init();

    logger.run();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
