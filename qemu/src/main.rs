#![no_std]
#![no_main]

use core::panic::{ PanicInfo };
use core::fmt::{ Write };
use core::time::{ Duration };
use toolkit::runtime::{ RuntimeInner, Timer, Runtime };
use toolkit::extbytebuf::{ ExtByteBuf, VolatileByteBuf };
use toolkit::uart16550::{ Uart16550 };

use core::arch::global_asm;
global_asm!(include_str!("trap.s"));

struct VirtTimer { }

impl Timer for VirtTimer {
    fn time(&mut self) -> Duration {
        Duration::default()
    }
}

const BYTE_BLOCK_LEN: usize = 0x0400;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let uart = Uart16550::new(ExtByteBuf::new(0x1000_0000, 8));

    let runtime = RuntimeInner::<VirtTimer, Uart16550<BYTE_BLOCK_LEN>, 4, 4, BYTE_BLOCK_LEN>::new(
        VirtTimer { }, uart
    );

    let Some(mut log0) = runtime.chan(0) else {
        panic!("logger get channel 0 failed");
    };

    writeln!(log0, "hello world!!!");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
