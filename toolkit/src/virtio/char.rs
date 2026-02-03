use crate::runtime::{ Runtime };
use crate::bytebuf::{ RawByteBuf, ByteBuf, VolatileByteBuf };

const MAGIC: usize      = 0x0000;
const VERSION: usize    = 0x0004;
const ID: usize         = 0x0008;

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

pub enum CharDevDrvErr {
    Fatal,
    Timeout,
}

pub struct CharDevDrv<'a, RT> {
    rt: RT, 
    regbuf: RawByteBuf<'a>,
    descbuf: RawByteBuf<'a>,
    drvbuf: RawByteBuf<'a>,
    devbuf: RawByteBuf<'a>,
    databuf: RawByteBuf<'a>,
}

impl<'a, RT>
CharDevDrv<'a, RT>
where RT: Runtime {
    pub fn new(rt: RT, regbuf: RawByteBuf<'a>, databuf: RawByteBuf<'a>,
        descbuf: RawByteBuf<'a>, drvbuf: RawByteBuf<'a>, devbuf: RawByteBuf<'a>) -> Self {
        Self {
            rt: rt, regbuf: regbuf, databuf: databuf,
            descbuf: descbuf, drvbuf: drvbuf, devbuf: devbuf,
        }
    }

    pub fn regbuf(&mut self, regbuf: RawByteBuf<'a>) {
        self.regbuf = regbuf;
    }

    pub fn get_magic(&mut self) -> u32 {
        self.regbuf.rd32_volatile(MAGIC)
    }

    pub fn get_version(&mut self) -> u32 {
        self.regbuf.rd32_volatile(VERSION)
    }

    pub fn get_id(&mut self) -> u32 {
        self.regbuf.rd32_volatile(ID)
    }

    pub fn init(&mut self) -> Result<(), CharDevDrvErr> {
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
            return Err(CharDevDrvErr::Fatal);
        }

        status |= STATUS_DRV_OK;
        self.regbuf.wr32_volatile(STATUS, status);
        Ok(())
    }

    pub fn emerg_wr(&mut self, data: &[u8]) {
        for item in data {
            self.regbuf.wr8_volatile(CFG_EMERG_WR, *item);
        }
    }

    pub fn send(&mut self, data: &[u8]) {
        self.databuf.copy_from(0, data);
    }

    pub fn send_poll(&mut self) -> Result<(), CharDevDrvErr> {
        Err(CharDevDrvErr::Timeout)
    }
}
