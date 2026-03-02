#![no_std]
#![no_main]

use core::panic::{ PanicInfo };
use core::fmt::{ Write };
use core::array::{ self };
use core::time::{ Duration };

use toolkit::runtime::{ RuntimeInner, Timer, Runtime };
use toolkit::extbytebuf::{ ExtByteBuf, VolatileByteBuf };
use toolkit::io::{ SendByteChan, Poll, Error };
use toolkit::uart16550::{ Uart16550 };

use core::arch::global_asm;
global_asm!(include_str!("trap.s"));

struct VirtTimer { }

impl Timer for VirtTimer {
    fn time(&mut self) -> Duration {
        Duration::default()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let runtime = RuntimeInner::<VirtTimer, 4, 0x1000>::new(VirtTimer { });
    let Some(mut log0) = runtime.chan(0) else {
        panic!("logger get channel 0 failed");
    };

    writeln!(log0, "hello world!!!");

    let mut buf: [u8; 256] = array::from_fn(|_| 0);
    let rd = runtime.rd_log(0, &mut buf);

    let mut uart = Uart16550::new(ExtByteBuf::new(0x1000_0000, 8));

    let mut total = 0;
    let mut wr = 0;
    for _ in 0..4 {
        let Poll::Ready(res) = uart.try_send(&buf[total..]) else {
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
