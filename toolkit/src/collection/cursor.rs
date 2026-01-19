#[derive(Clone, Copy)]
enum Dir {
    Inc,
    Dec,
}

#[derive(Clone, Copy, Debug)]
pub struct Cursor<const MAX: usize, const INC: bool> {
    pos: usize,
}

impl<const MAX: usize, const INC: bool> Cursor<MAX, INC> {
    pub fn new(mut pos: usize) -> Self {
        if pos >= MAX {
            pos = MAX - 1;
        }
        Self {
            pos: pos,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    fn at_edge(&self, movedir: Dir) -> bool {
        match INC {
            true => match movedir {
                Dir::Inc => self.pos == MAX - 1,
                Dir::Dec => self.pos == 0,
            },
            false => match movedir {
                Dir::Inc => self.pos == 0,
                Dir::Dec => self.pos == MAX - 1,
            }
        }
    }

    fn overflow(&self, movedir: Dir) -> usize {
        match INC {
            true => match movedir {
                Dir::Inc => 0,
                Dir::Dec => MAX - 1,
            },
            false => match movedir {
                Dir::Inc => MAX - 1,
                Dir::Dec => 0,
            }
        }
    }

    fn forward(&self, movedir: Dir) -> usize {
        match INC {
            true => match movedir {
                Dir::Inc => self.pos + 1,
                Dir::Dec => self.pos - 1,
            },
            false => match movedir {
                Dir::Inc => self.pos - 1,
                Dir::Dec => self.pos + 1,
            }
        }
    }

    fn do_move(&mut self, dir: Dir) {
        self.pos = match self.at_edge(dir) {
            true => self.overflow(dir),
            false => self.forward(dir),
        }
    }

    pub fn next(&mut self) {
        self.do_move(Dir::Inc);
    }

    pub fn prev(&mut self) {
        self.do_move(Dir::Dec);
    }
}
