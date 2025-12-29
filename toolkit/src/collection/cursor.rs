#[derive(Clone, Copy)]
pub struct Cursor {
    pos: usize,
    max: usize,
    inc: bool,
}

impl Cursor {
    pub fn new(pos: usize, len: usize, inc: bool) -> Self {
        Self {
            pos: pos, max: len - 1, inc: inc,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn next(&mut self) {
        self.pos = match self.inc {
            true => match self.pos == self.max {
                true => 0,
                false => self.pos + 1,
            },
            false => match self.pos == 0 {
                true => self.max,
                false => self.pos - 1,
            },
        };
    }

    pub fn prev(&mut self) {
        self.pos = match self.inc {
            true => match self.pos == 0 {
                true => self.max,
                false => self.pos - 1,
            },
            false => match self.pos == self.max {
                true => 0,
                false => self.pos + 1,
            },
        };
    }
}
