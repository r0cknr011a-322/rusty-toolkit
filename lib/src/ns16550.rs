use core::pin::{ Pin };
use core::future::{ Future };
use core::task::{ Context, Poll };
use crate::io::{ Send as IOSend, Error as IOError };
use rusty_scrapyard_iomem::{ RWVolatile8 };

#[derive(Copy, Clone)]
pub struct NS16550<RW: RWVolatile8> {
    io: RW,
}

impl<RW: RWVolatile8> NS16550<RW> {
    pub fn new(io: RW) -> Self {
        Self {
            io: io,
        }
    }
}

impl<RW: RWVolatile8> IOSend for NS16550<RW> {
    fn send(self, data: &[u8]) -> impl Future<Output = Result<usize, IOError>> {
        SendTask {
            io: self.io,
            data: data,
        }
    }
}

struct SendTask<'a, RW: RWVolatile8> {
    io: RW,
    data: &'a [u8],
}

impl<'a, RW: RWVolatile8> Future for SendTask<'_, RW> {
    type Output = Result<usize, IOError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        for b in self.data {
            // self.io.wr8_volatile(0x00, *b);
        }
        Poll::Ready(Ok(self.data.len()))
    }
}
