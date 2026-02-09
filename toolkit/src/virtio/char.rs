use core::fmt::{ Write };
use crate::runtime::{ Runtime };
use crate::bytebuf::{ RawByteBuf, ByteBuf, VolatileByteBuf };
use crate::collection::deque::{ Deque };
use crate::task::{ Session, Queue, Poll, GenRsp, GenErr };

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

pub struct CharDevDrv<'a, RT> {
    rt: RT, 
    regbuf: RawByteBuf<'a>,
    descbuf: RawByteBuf<'a>,
    drvbuf: RawByteBuf<'a>,
    devbuf: RawByteBuf<'a>,
    databuf: Deque<Option<&'a RawByteBuf<'a>>, 4>,
}

impl<'a, RT>
CharDevDrv<'a, RT>
where RT: Runtime {
    pub fn new(rt: RT, regbuf: RawByteBuf<'a>, // databuf: RawByteBuf<'a>,
        descbuf: RawByteBuf<'a>, drvbuf: RawByteBuf<'a>, devbuf: RawByteBuf<'a>) -> Self {
        Self {
            rt: rt, regbuf: regbuf,
            descbuf: descbuf, drvbuf: drvbuf, devbuf: devbuf,
            databuf: Deque::default(),
        }
    }

    pub fn emerg_wr(&mut self, data: &[u8]) {
        for item in data {
            self.regbuf.wr8_volatile(CFG_EMERG_WR, *item);
        }
    }
}

impl<'a, RT>
Session for CharDevDrv<'a, RT>
where RT: Runtime {
    type Arg = ();
    type Error = GenErr;

    fn init(&mut self, arg: ()) -> Poll<Result<(), GenErr>> {
        let magic = self.regbuf.rd32_volatile(MAGIC);
        if magic != MAGIC_VAL {
            return Poll::Ready(Err(GenErr::Fatal));
        }

        let id = self.regbuf.rd32_volatile(ID);
        if id != ID_VAL {
            return Poll::Ready(Err(GenErr::Fatal));
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
            return Poll::Ready(Err(GenErr::Fatal));
        }

        status |= STATUS_DRV_OK;
        self.regbuf.wr32_volatile(STATUS, status);

        Poll::Ready(Ok(()))
    }

    fn exit(&mut self) -> Poll<()> {
        Poll::Ready(())
    }
}

impl<'a, RT>
Queue for CharDevDrv<'a, RT>
where RT: Runtime {
    type Request = &'a RawByteBuf<'a>;
    type Response = GenRsp;
    type Error = GenErr;

    fn push(&mut self, req: &'a RawByteBuf<'a>) -> Poll<Result<(), GenErr>> {
        if self.databuf.is_full() {
            return Poll::Pending;
        }
        let Some(mut logger) = self.rt.log(1) else {
            return Poll::Ready(Err(GenErr::Fatal));
        };

        writeln!(logger, "push: new buf: {:?} {:?}", req.addr(), req.len());
        self.databuf.push(Some(req));
        Poll::Ready(Ok(()))
    }

    fn pop(&mut self) -> Poll<Result<GenRsp, GenErr>> {
        Poll::Ready(Ok(GenRsp::Ok))
    }
}
