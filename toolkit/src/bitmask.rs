pub struct BitMask32 {
    mask: u32,
}

impl BitMask32 {
    pub const fn mask(high: u32, low: u32) -> Self {
        Self {
            mask: ((1 << (high - low + 1)) - 1) << low,
        }
    }

    pub(crate) const fn value(&self) -> u32 {
        self.mask
    }

    pub const fn get(&self, value: u32) -> u32 {
        (value & self.mask) >> self.mask.trailing_zeros()
    }

    pub const fn set(&self, mut value: u32, mut set: u32) -> u32 {
        value &= !self.mask;
        set &= self.mask >> self.mask.trailing_zeros();
        value | (set << self.mask.trailing_zeros())
    }
}
