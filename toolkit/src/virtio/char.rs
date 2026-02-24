use crate::extbytebuf::{ ExtByteBuf, ByteBuf, VolatileByteBuf };
use crate::collection::deque::{ Deque };
use crate::collection::asynque::{ Asynque, Poll };
use crate::collection::byteblock::{ ByteBlock };
use crate::io::{ Error as IOErr };

const MAGIC: usize      = 0x0000;
const MAGIC_VAL: u32    = 0x74726976;

const VERSION: usize    = 0x0004;
const ID: usize         = 0x0008;
const ID_VAL: u32       = 3;

const DEV_FEAT_VAL: usize   = 0x0010;
const DEV_FEAT_SEL: usize   = 0x0014;
const DRV_FEAT_VAL: usize   = 0x0020;
const DRV_FEAT_SEL: usize   = 0x0024;
const FEAT_SIZE: u32        = 0x01;
const FEAT_MULTIPORT: u32   = 0x02;
const FEAT_EMERG_WR: u32    = 0x04;

const STATUS: usize         = 0x0070;
const STATUS_ACK: u32       = 0x0001;
const STATUS_DRV: u32       = 0x0002;
const STATUS_DRV_OK: u32    = 0x0004;
const STATUS_FEAT_OK: u32   = 0x0008;
const STATUS_RESET: u32     = 0x0040;
const STATUS_FAIL: u32      = 0x0080;

const CFG_EMERG_WR: usize   = 0x0108;


pub struct CharDev<'a, const L: usize> {
    regbuf: ExtByteBuf<'a>,
    descbuf: ExtByteBuf<'a>,
    drvbuf: ExtByteBuf<'a>,
    devbuf: ExtByteBuf<'a>,
}

pub enum Error {
    Fatal,
}

impl<'a, const L: usize> CharDev<'a, L> {
    pub fn new(
        regbuf: ExtByteBuf<'a>, descbuf: ExtByteBuf<'a>,
        drvbuf: ExtByteBuf<'a>, devbuf: ExtByteBuf<'a>
    ) -> Self {
        Self {
            regbuf: regbuf, descbuf: descbuf,
            drvbuf: drvbuf, devbuf: devbuf,
        }
    }

    pub fn emerg_wr(&mut self, data: &[u8]) {
        for item in data {
            self.regbuf.wr8_volatile(CFG_EMERG_WR, *item);
        }
    }

    fn init(&mut self) -> Result<(), Error> {
        let magic = self.regbuf.rd32_volatile(MAGIC);
        if magic != MAGIC_VAL {
            return Err(Error::Fatal);
        }

        let id = self.regbuf.rd32_volatile(ID);
        if id != ID_VAL {
            return Err(Error::Fatal);
        }

        let mut status = 0;
        self.regbuf.wr32_volatile(STATUS, status);

        status |= STATUS_ACK | STATUS_DRV;
        self.regbuf.wr32_volatile(STATUS, status);

        self.regbuf.wr32_volatile(DEV_FEAT_SEL, 0);
        let chardev_features = self.regbuf.rd32_volatile(DEV_FEAT_VAL);

        self.regbuf.wr32_volatile(DEV_FEAT_SEL, 1);
        let common_features = self.regbuf.rd32_volatile(DEV_FEAT_VAL);

        if chardev_features & FEAT_EMERG_WR != 0 {
            self.regbuf.wr32_volatile(DRV_FEAT_SEL, 0);
            self.regbuf.wr32_volatile(DRV_FEAT_VAL, FEAT_EMERG_WR);
        }

        status |= STATUS_FEAT_OK;
        self.regbuf.wr32_volatile(STATUS, status);

        let status_new = self.regbuf.rd32_volatile(STATUS);
        if status_new != status {
            return Err(Error::Fatal);
        }

        status |= STATUS_DRV_OK;
        self.regbuf.wr32_volatile(STATUS, status);

        Ok(())
    }

    fn exit(&mut self) {
        ()
    }
}

impl<'a, const L: usize> Asynque for CharDev<'a, L> {
    type Req = ByteBlock<L>;
    type Rsp = ();
    type Err = IOErr;

    fn try_push(&mut self, block: ByteBlock<L>) -> Poll<Result<(), IOErr>> {
        Poll::Ready(Err(IOErr::Fatal))
    }

    fn poll_push(&mut self) -> Poll<Result<(), IOErr>> {
        Poll::Ready(Err(IOErr::Fatal))
    }

    fn try_pop(&mut self) -> Poll<Result<(), IOErr>> {
        Poll::Ready(Err(IOErr::Fatal))
    }

    fn poll_pop(&mut self) -> Poll<Result<(), IOErr>> {
        Poll::Ready(Err(IOErr::Fatal))
    }
}
