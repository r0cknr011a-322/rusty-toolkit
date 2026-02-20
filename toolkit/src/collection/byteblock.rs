use core::array::{ self };


#[derive(Clone, Copy)]
pub struct ByteBlock<const L: usize> {
    buf: [u8; L],
    len: usize,
}

impl<const L: usize> Default for ByteBlock<L> {
    fn default() -> Self {
        Self {
            buf: array::from_fn(|_| { 0 }),
            len: 0,
        }
    }
}

impl<const L: usize> ByteBlock<L> {
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn free(&self) -> usize {
        self.buf.len() - self.len
    }

    pub fn rd_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn wr_slice(&mut self, data: &[u8]) {
        let slice = &mut self.buf[self.len..];
        if data.len() < slice.len() {
            let (dst, _) = slice.split_at_mut(data.len());
            dst.copy_from_slice(data);
            self.len += data.len();
            return;
        }
        if data.len() > slice.len() {
            let (src, _) = data.split_at(slice.len());
            slice.copy_from_slice(src);
            self.len += slice.len();
            return;
        }
        slice.copy_from_slice(data);
        self.len += data.len();
    }
}
