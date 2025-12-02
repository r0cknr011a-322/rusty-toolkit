use core::cmp::{ min };
use crate::io::{ AsyncWrite, AsyncRead, Error as IOError, Poll as IOPoll };
use rusty_scrapyard_iomem::{ RWVolatile8 };

pub struct UART16550<RW: RWVolatile8> {
    io: RW,
}

impl<RW: RWVolatile8> UART16550<RW> {
    pub fn new(io: RW) -> Self {
        Self {
            io: io,
        }
    }
}

const DATA_OFF: usize = 0x00;
const LINE_STATUS_OFF: usize = 0x05;

impl<RW: RWVolatile8> AsyncWrite for UART16550<RW> {
    fn poll_write(&mut self, buf: &[u8]) -> IOPoll<Result<usize, IOError>> {
        for b in buf {
            self.io.wr8_volatile(DATA_OFF, *b);
        }
        IOPoll::Ready(Ok(buf.len()))
    }
}

const HAS_DATA: u8 = 0x01;

impl<RW: RWVolatile8> AsyncRead for UART16550<RW> {
    fn poll_read(&mut self, buf: &mut [u8]) -> IOPoll<Result<usize, IOError>> {
        let mut sent = 0;
        for i in 0..min(16, buf.len()) {
            if (self.io.rd8_volatile(LINE_STATUS_OFF) & HAS_DATA) == HAS_DATA {
                buf[i] = self.io.rd8_volatile(DATA_OFF);
                sent += 1;
            } else {
                break;
            }
        }

        if sent > 0 {
            return IOPoll::Ready(Ok(sent));
        }
        IOPoll::Pending
    }
}
