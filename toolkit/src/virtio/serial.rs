use crate::extbytebuf::{ ExtByteBuf, ByteBuf, VolatileByteBuf };
use crate::runtime::{ Runtime };
use crate::collection::deque::{ Deque };
use crate::io::{ RecvByteChan, Poll, Error as IOErr };


const MAGIC: usize              = 0x00;
const MAGIC_VAL: u32            = 0x74726976;

const VERSION: usize            = 0x04;
const VERSION_VAL: u32          = 0x02;

const ID: usize                 = 0x08;
const ID_VAL: u32               = 0x03;

const DEV_FEAT_VAL: usize       = 0x10;
const DEV_FEAT_SEL: usize       = 0x14;
const DRV_FEAT_VAL: usize       = 0x20;
const DRV_FEAT_SEL: usize       = 0x24;
const FEAT_SIZE: u32            = 0x01;
const FEAT_MULTIPORT: u32       = 0x02;
const FEAT_EMERG_WR: u32        = 0x04;

const QUEUE_IDX: usize          = 0x30;
const QUEUE_LEN_MAX: usize      = 0x34;
const QUEUE_LEN: usize          = 0x38;
const QUEUE_READY: usize        = 0x44;
const QUEUE_NOTIFY: usize       = 0x50;

const IRQ_STATUS: usize         = 0x60;
const IRQ_ACK: usize            = 0x64;

const STATUS: usize             = 0x70;
const STATUS_ACK: u32           = 0x01;
const STATUS_DRV: u32           = 0x02;
const STATUS_DRV_OK: u32        = 0x04;
const STATUS_FEAT_OK: u32       = 0x08;
const STATUS_RESET: u32         = 0x40;
const STATUS_FAIL: u32          = 0x80;

const CMD_QUEUE_ADDR_LO: usize  = 0x80;
const CMD_QUEUE_ADDR_HI: usize  = 0x84;
const DRV_QUEUE_ADDR_LO: usize  = 0x90;
const DRV_QUEUE_ADDR_HI: usize  = 0x94;
const DEV_QUEUE_ADDR_LO: usize  = 0xA0;
const DEV_QUEUE_ADDR_HI: usize  = 0xA4;

const CFG_EMERG_WR: usize   = 0x0108;


pub struct VirtQueue<'a> {
    cmdbuf: ExtByteBuf<'a>,
    drvbuf: ExtByteBuf<'a>,
    devbuf: ExtByteBuf<'a>,
}

impl<'a> VirtQueue<'a> {
    pub fn new(cmdbuf: (usize, usize), drvbuf: (usize, usize), devbuf: (usize, usize)) -> Self {
        Self {
            cmdbuf: ExtByteBuf::new(cmdbuf.0, cmdbuf.1),
            drvbuf: ExtByteBuf::new(drvbuf.0, drvbuf.1),
            devbuf: ExtByteBuf::new(devbuf.0, devbuf.1),
        }
    }
}



pub enum Error {
    Fatal,
}

const CMD_FLAG_NEXT: u32    = 0x01;
const CMD_FLAG_WRITE: u32   = 0x02;
const CMD_FLAG_SG: u32      = 0x04;

struct BufCmd {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

pub struct SerialDevice<'a, RT> {
    rt: RT,
    regbuf: ExtByteBuf<'a>,
    rdbuf: ExtByteBuf<'a>,
    wrbuf: ExtByteBuf<'a>,
    rdque: VirtQueue<'a>,
    wrque: VirtQueue<'a>,
}

impl<'a, RT> SerialDevice<'a, RT>
where RT: Runtime {
    pub fn new(
        rt: RT, regbuf: (usize, usize),
        rdbuf: (usize, usize), wrbuf: (usize, usize),
        rdque: VirtQueue<'a>, wrque: VirtQueue<'a>
    ) -> Self {
        Self {
            rt: rt,
            regbuf: ExtByteBuf::new(regbuf.0, regbuf.1),
            rdbuf: ExtByteBuf::new(rdbuf.0, rdbuf.1), wrbuf: ExtByteBuf::new(wrbuf.0, wrbuf.1),
            rdque: rdque, wrque: wrque,
        }
    }

    pub fn emerg_wr(&mut self, data: &[u8]) {
        for item in data {
            self.regbuf.wr8_volatile(CFG_EMERG_WR, *item);
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        let magic = self.regbuf.rd32_volatile(MAGIC);
        let version = self.regbuf.rd32_volatile(VERSION);
        let id = self.regbuf.rd32_volatile(ID);

        writeln!(self.rt, "magic: {:X}; version: {}; id: {}", magic, version, id);

        if magic != MAGIC_VAL {
            return Err(Error::Fatal);
        }

        if version != VERSION_VAL {
            return Err(Error::Fatal);
        }

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

        let mut addr: u64 = 0;
        self.regbuf.wr32_volatile(QUEUE_IDX, 0);

        addr = self.rdque.cmdbuf.addr() as u64;
        self.regbuf.wr32_volatile(CMD_QUEUE_ADDR_LO, (addr & 0xFFFF_FFFF) as u32);
        self.regbuf.wr32_volatile(CMD_QUEUE_ADDR_HI, (addr >> 32) as u32);

        addr = self.rdque.drvbuf.addr() as u64;
        self.regbuf.wr32_volatile(DRV_QUEUE_ADDR_LO, (addr & 0xFFFF_FFFF) as u32);
        self.regbuf.wr32_volatile(DRV_QUEUE_ADDR_HI, (addr >> 32) as u32);

        addr = self.rdque.devbuf.addr() as u64;
        self.regbuf.wr32_volatile(DEV_QUEUE_ADDR_LO, (addr & 0xFFFF_FFFF) as u32);
        self.regbuf.wr32_volatile(DEV_QUEUE_ADDR_HI, (addr >> 32) as u32);


        self.regbuf.wr32_volatile(QUEUE_IDX, 1);

        addr = self.wrque.cmdbuf.addr() as u64;
        self.regbuf.wr32_volatile(CMD_QUEUE_ADDR_LO, (addr & 0xFFFF_FFFF) as u32);
        self.regbuf.wr32_volatile(CMD_QUEUE_ADDR_HI, (addr >> 32) as u32);

        addr = self.rdque.drvbuf.addr() as u64;
        self.regbuf.wr32_volatile(DRV_QUEUE_ADDR_LO, (addr & 0xFFFF_FFFF) as u32);
        self.regbuf.wr32_volatile(DRV_QUEUE_ADDR_HI, (addr >> 32) as u32);

        addr = self.rdque.devbuf.addr() as u64;
        self.regbuf.wr32_volatile(DEV_QUEUE_ADDR_LO, (addr & 0xFFFF_FFFF) as u32);
        self.regbuf.wr32_volatile(DEV_QUEUE_ADDR_HI, (addr >> 32) as u32);


        status |= STATUS_DRV_OK;
        self.regbuf.wr32_volatile(STATUS, status);

        Ok(())
    }

    fn exit(&mut self) {
        ()
    }
}
