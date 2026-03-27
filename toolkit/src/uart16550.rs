use crate::extbytebuf::{ ExtByteBuf, VolatileByteBuf };
use crate::io::{ Poll, Error as IOErr, SendByteChan };


const RX_DATA: usize        = 0x00;
const TX_DATA: usize        = 0x00;
const IRQ_EN: usize         = 0x01;
const FIFO_CTRL: usize      = 0x02;
const LINE_CTRL: usize      = 0x03;
const MODEM_CTRL: usize     = 0x04;

const LINE_STATUS: usize    = 0x05;
const RX_READY: u8          = 0x01;
const TX_READY: u8          = 0x20;
const TX_EMPTY: u8          = 0x40;

const MODEM_STATUS: usize   = 0x06;


pub struct Uart16550<'a> {
    regbuf: ExtByteBuf<'a>,
}

impl<'a> Uart16550<'a> {
    pub fn new(regbuf: (usize, usize)) -> Self {
        Self {
            regbuf: ExtByteBuf::new(regbuf.0, regbuf.1),
        }
    }
}

impl<'a> SendByteChan for Uart16550<'a> {
    fn try_send(&mut self, data: &[u8]) -> Poll<Result<usize, IOErr>> {
        let mut status = 0;
        let mut wr = 0;
        for byte in data {
            status = self.regbuf.rd8_volatile(LINE_STATUS);
            if status & TX_READY == 0 {
                break;
            }
            self.regbuf.wr8_volatile(TX_DATA, *byte);
            wr += 1;
        }

        if wr == 0 {
            return Poll::Pending;
        }

        Poll::Ready(Ok(wr))
    }

    fn poll_send(&mut self) -> Poll<Result<(), IOErr>> {
        let mut status = self.regbuf.rd8_volatile(LINE_STATUS);
        if status & TX_EMPTY == 0 {
            return Poll::Pending;
        }
        Poll::Ready(Ok(()))
    }
}
