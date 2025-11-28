use core::ptr;
use core::fmt;
use core::slice;

pub mod muart;
pub mod mgeth;
pub mod basis_timer;

struct NetTrxWrite<'a> {
	buf: &'a [u8],
}

struct NetTrxRead<'a> {
	buf: &'a [u8],
}

struct NetDriverCaps {
	align: usize,
}

enum NetTrx {
	NetTrxWrite,
	NetTrxRead,
}

enum NetRes {
	NetResOk,
}

enum NetErr {
	InvalidTrx(&'static str),
	Timeout(&'static str),
}

pub trait NetDriver {
    fn getcaps(&self) -> NetDriverCaps;
    fn resume(&mut self, trxq: &[NetTrx]);
    fn poll(&mut self, resq: &mut [NetRes]);
}

pub struct DmaBuf {
    base: *const u8,
    len: usize,
    pos: usize,
}

impl DmaBuf {
    pub fn new(base: usize, len: usize, _align: usize) -> Self {
        DmaBuf {
            base: ptr::with_exposed_provenance(base),
            len: len,
            pos: 0,
        }
    }
}

pub struct RawDataBuf<'a> {
    pub buf: &'a mut [u8],
    pos: usize,
}

impl RawDataBuf<'_> {
    pub fn new(start: usize, len: usize, _align: usize) -> Self {
        let startptr: *const u8 = ptr::with_exposed_provenance(start);
        let buf;
        unsafe { buf = slice::from_raw_parts_mut(startptr.cast_mut(), len); }
        Self { buf: buf, pos: 0 }
    }
}

impl fmt::Write for RawDataBuf<'_> {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        self.buf[self.pos..self.pos + data.len()].copy_from_slice(data.as_bytes());
        Ok(())
    }
}
