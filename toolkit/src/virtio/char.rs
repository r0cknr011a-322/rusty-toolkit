use crate::runtime::{ Runtime };
use crate::bytebuf::{ VolatileByteBuf };

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

pub struct CharDevDrv<RT> {
    rt: RT, 
    regbufidx: usize,
}

impl<RT>
CharDevDrv<RT>
where RT: Runtime {
    pub fn new(rt: RT, idx: usize) -> Self {
        Self {
            rt: rt, regbufidx: idx,
        }
    }

    pub fn set_regbuf(&mut self, idx: usize) {
        self.regbufidx = idx;
    }

    pub fn get_regbuf(&self) -> usize {
        self.regbufidx
    }

    pub fn get_magic(&mut self) -> u32 {
        let Some(mut regbuf) = self.rt.dev(self.regbufidx) else {
            return 0;
        };
        regbuf.rd32_volatile(MAGIC)
    }

    pub fn get_version(&mut self) -> u32 {
        let Some(mut regbuf) = self.rt.dev(self.regbufidx) else {
            return 0;
        };
        regbuf.rd32_volatile(VERSION)
    }

    pub fn get_id(&mut self) -> u32 {
        let Some(mut regbuf) = self.rt.dev(self.regbufidx) else {
            return 0;
        };
        regbuf.rd32_volatile(ID)
    }

    pub fn reset(&mut self) {
        let Some(mut regbuf) = self.rt.dev(self.regbufidx) else {
            return;
        };
        let mut status = 0;
        regbuf.wr32_volatile(STATUS, status);

        status |= STATUS_ACK | STATUS_DRV;
        regbuf.wr32_volatile(STATUS, status);

        regbuf.wr32_volatile(DEV_FEAT_SEL, 0);
        let features = regbuf.rd32_volatile(DEV_FEAT_VAL);
        if features & FEAT_EMERG_WR != 0 {
            regbuf.wr32_volatile(DRV_FEAT_SEL, 0);
            regbuf.wr32_volatile(DRV_FEAT_VAL, FEAT_EMERG_WR);
        }

        status |= STATUS_FEAT_OK;
        regbuf.wr32_volatile(STATUS, status);

        let status_new = regbuf.rd32_volatile(STATUS);
        if status_new != status {
            return;
        }

        status |= STATUS_DRV_OK;
        regbuf.wr32_volatile(STATUS, status);
    }

    pub fn emerg_wr(&mut self, data: &[u8]) {
        if let Some(mut regbuf) = self.rt.dev(self.regbufidx) {
            for item in data {
                regbuf.wr8_volatile(CFG_EMERG_WR, *item);
            }
        }
    }
}
