use core::cmp;
use core::cell::RefCell;
use core::fmt::{ self, Write };

pub struct LogBuf {
    buf: [u8; 32],
    pos: usize,
}

impl LogBuf {
    pub fn new() -> Self {
        Self { buf: [0; 32], pos: 0 }
    }

    pub fn get_data(&mut self) -> &[u8] {
        self.pos = 0;
        &self.buf
    }

    pub fn is_ready(&self) -> bool {
        self.buf.len() - self.pos == 0
    }
}

impl fmt::Write for LogBuf {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        let cnt = cmp::min(self.buf.len() - self.pos, data.len());
        let dst = &mut self.buf[self.pos..self.pos + cnt];
        let src = &data.as_bytes()[0..cnt];
        dst.copy_from_slice(src);
        self.pos += cnt;
        Ok(())
    }
}

pub struct LogCell<'a, Logger>
where Logger: fmt::Write {
    logcell: &'a RefCell<Logger>,
}

impl<'a, Logger> LogCell<'a, Logger>
where Logger: fmt::Write {
    pub fn new(logcell: &'a RefCell<Logger>) -> Self {
        Self { logcell }
    }
}

impl<Logger> Clone for LogCell<'_, Logger>
where Logger: fmt::Write {
    fn clone(&self) -> Self {
        Self { logcell: self.logcell }
    }
}

impl<Logger> Copy for LogCell<'_, Logger>
where Logger: fmt::Write {
    
}

impl<Logger> fmt::Write for LogCell<'_, Logger>
where Logger: fmt::Write {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        let mut logger = self.logcell.borrow_mut();
        logger.write_str(data)
    }
}
