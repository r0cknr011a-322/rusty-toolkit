use toolkit_unsafe::{ IPCByteBuf };

pub trait ByteBuf {
    fn addr(&self) -> usize;
    fn len(&self) -> usize;

    fn copy_to(&mut self, off: usize, buf: &mut [u8]);
    fn copy_from(&mut self, off: usize, buf: &[u8]);

    fn rd8(&mut self, off: usize) -> u8;
    fn wr8(&mut self, off: usize, value: u8);

    fn rd16(&mut self, off: usize) -> u16;
    fn wr16(&mut self, off: usize, value: u16);

    fn rd32(&mut self, off: usize) -> u32;
    fn wr32(&mut self, off: usize, value: u32);

    fn rd64(&mut self, off: usize) -> u64;
    fn wr64(&mut self, off: usize, value: u64);
}

pub trait VolatileByteBuf {
    fn rd8_volatile(&mut self, off: usize) -> u8;
    fn wr8_volatile(&mut self, off: usize, value: u8);

    fn rd16_volatile(&mut self, off: usize) -> u16;
    fn wr16_volatile(&mut self, off: usize, value: u16);

    fn rd32_volatile(&mut self, off: usize) -> u32;
    fn wr32_volatile(&mut self, off: usize, value: u32);

    fn rd64_volatile(&mut self, off: usize) -> u64;
    fn wr64_volatile(&mut self, off: usize, value: u64);
}

pub trait AtomicByteBuf {
    fn rd8_atomic(&mut self, off: usize) -> u8;
    fn wr8_atomic(&mut self, off: usize, value: u8);

    fn rd16_atomic(&mut self, off: usize) -> u16;
    fn wr16_atomic(&mut self, off: usize, value: u16);

    fn rd32_atomic(&mut self, off: usize) -> u32;
    fn wr32_atomic(&mut self, off: usize, value: u32);

    fn rd64_atomic(&mut self, off: usize) -> u64;
    fn wr64_atomic(&mut self, off: usize, value: u64);
}

pub struct MemByteBuf<'a> {
    mem: IPCByteBuf<'a>,
}

impl MemByteBuf<'_> {
    pub fn new(addr: usize, len: usize) -> Self {
        Self {
            mem: IPCByteBuf::new(addr, len),
        }
    }
}

impl ByteBuf for MemByteBuf<'_> {
    fn addr(&self) -> usize {
        self.mem.addr()
    }

    fn len(&self) -> usize {
        self.mem.len()
    }

    fn rd8(&mut self, off: usize) -> u8 {
        self.mem.rd8(off)
    }

    fn copy_to(&mut self, off: usize, buf: &mut [u8]) {
        self.mem.copy_to(off, buf);
    }

    fn copy_from(&mut self, off: usize, buf: &[u8]) {
        self.mem.copy_from(off, buf);
    }

    fn wr8(&mut self, off: usize, value: u8) {
        self.mem.wr8(off, value);
    }

    fn rd16(&mut self, off: usize) -> u16 {
        self.mem.rd16(off)
    }

    fn wr16(&mut self, off: usize, value: u16) {
        self.mem.wr16(off, value);
    }

    fn rd32(&mut self, off: usize) -> u32 {
        self.mem.rd32(off)
    }

    fn wr32(&mut self, off: usize, value: u32) {
        self.mem.wr32(off, value);
    }

    fn rd64(&mut self, off: usize) -> u64 {
        self.mem.rd64(off)
    }

    fn wr64(&mut self, off: usize, value: u64) {
        self.mem.wr64(off, value);
    }
}

impl VolatileByteBuf for MemByteBuf<'_> {
    fn rd8_volatile(&mut self, off: usize) -> u8 {
        self.mem.rd8_volatile(off)
    }

    fn wr8_volatile(&mut self, off: usize, value: u8) {
        self.mem.wr8_volatile(off, value);
    }

    fn rd16_volatile(&mut self, off: usize) -> u16 {
        self.mem.rd16_volatile(off)
    }

    fn wr16_volatile(&mut self, off: usize, value: u16) {
        self.mem.wr16_volatile(off, value);
    }

    fn rd32_volatile(&mut self, off: usize) -> u32 {
        self.mem.rd32_volatile(off)
    }

    fn wr32_volatile(&mut self, off: usize, value: u32) {
        self.mem.wr32_volatile(off, value);
    }

    fn rd64_volatile(&mut self, off: usize) -> u64 {
        self.mem.rd64_volatile(off)
    }

    fn wr64_volatile(&mut self, off: usize, value: u64) {
        self.mem.wr64_volatile(off, value);
    }
}

impl AtomicByteBuf for MemByteBuf<'_> {
    fn rd8_atomic(&mut self, off: usize) -> u8 {
        self.mem.rd8_volatile(off)
    }

    fn wr8_atomic(&mut self, off: usize, value: u8) {
        self.mem.wr8_volatile(off, value);
    }

    fn rd16_atomic(&mut self, off: usize) -> u16 {
        self.mem.rd16_volatile(off)
    }

    fn wr16_atomic(&mut self, off: usize, value: u16) {
        self.mem.wr16_volatile(off, value);
    }

    fn rd32_atomic(&mut self, off: usize) -> u32 {
        self.mem.rd32_volatile(off)
    }

    fn wr32_atomic(&mut self, off: usize, value: u32) {
        self.mem.wr32_volatile(off, value);
    }

    fn rd64_atomic(&mut self, off: usize) -> u64 {
        self.mem.rd64_volatile(off)
    }

    fn wr64_atomic(&mut self, off: usize, value: u64) {
        self.mem.wr64_volatile(off, value);
    }
}
