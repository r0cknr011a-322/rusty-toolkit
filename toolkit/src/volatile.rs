use toolkit_unsafe::{ IPCByteBuf };

pub trait VolatileByteBuf {
    fn rd8(&mut self, off: usize) -> u8;
    fn wr8(&mut self, off: usize, value: u8);

    fn rd16(&mut self, off: usize) -> u16;
    fn wr16(&mut self, off: usize, value: u16);

    fn rd32(&mut self, off: usize) -> u32;
    fn wr32(&mut self, off: usize, value: u32);

    fn rd64(&mut self, off: usize) -> u64;
    fn wr64(&mut self, off: usize, value: u64);
}

pub struct MemVolatileByteBuf {
    mem: IPCByteBuf,
}

impl VolatileByteBuf for MemVolatileByteBuf {
    fn rd8(&mut self, off: usize) -> u8 {
        self.mem.rd8_volatile(off)
    }

    fn wr8(&mut self, off: usize, value: u8) {
        self.mem.wr8_volatile(off, value);
    }

    fn rd16(&mut self, off: usize) -> u16 {
        self.mem.rd16_volatile(off)
    }

    fn wr16(&mut self, off: usize, value: u16) {
        self.mem.wr16_volatile(off, value);
    }

    fn rd32(&mut self, off: usize) -> u32 {
        self.mem.rd32_volatile(off)
    }

    fn wr32(&mut self, off: usize, value: u32) {
        self.mem.wr32_volatile(off, value);
    }

    fn rd64(&mut self, off: usize) -> u64 {
        self.mem.rd64_volatile(off)
    }

    fn wr64(&mut self, off: usize, value: u64) {
        self.mem.wr64_volatile(off, value);
    }
}
