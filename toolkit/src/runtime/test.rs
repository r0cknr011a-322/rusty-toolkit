use core::time::{ Duration };
use core::array::{ self };

use crate::runtime::{ Timer, Runtime, RuntimeInner, RuntimeRef };
use crate::collection::deque::{ Deque };


#[derive(Default)]
struct TestTimer { }

impl Timer for TestTimer {
    fn time(&mut self) -> Duration {
        Duration::default()
    }
}

const MSG: &str = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in
culpa qui officia deserunt mollit anim id est laborum.
"#;

struct TestItem<R> {
    rt: R,
}

impl<R> TestItem<R>
where R: Runtime {
    fn new(rt: R) -> Self {
        Self {
            rt: rt,
        }
    }

    fn log(&mut self) {
        write!(self.rt, "{}", MSG);
    }
}


const CHAN_NR: usize = 4;
const BUF_LEN: usize = 0x1000;

#[test]
fn log() {
    let rt = RuntimeInner::<TestTimer, CHAN_NR, BUF_LEN>::new(TestTimer::default());

    let Some(log0) = rt.chan(0) else {
        panic!("logger channel doesn't exist");
    };

    let Some(log1) = rt.chan(1) else {
        panic!("logger channel doesn't exist");
    };

    let Some(log2) = rt.chan(2) else {
        panic!("logger channel doesn't exist");
    };

    let Some(log3) = rt.chan(3) else {
        panic!("logger channel doesn't exist");
    };

    let log4 = rt.chan(4);
    assert!(log4.is_none());

    let mut item0 = TestItem::new(log0);
    let mut item1 = TestItem::new(log1);
    let mut item2 = TestItem::new(log2);
    let mut item3 = TestItem::new(log3);

    let mut databuf: [u8; 256] = array::from_fn(|_| 0);
    let mut read = 0;

    /* log 2 times */
    for _ in 0..2 {
        item0.log();
    }
    assert_eq!(rt.chan_len(0), Some(MSG.len() * 2));

    for _ in 0..3 {
        read = rt.rd_log(0, &mut databuf);
        assert_eq!(read, databuf.len());
    }

    read = rt.rd_log(0, &mut databuf);
    assert_eq!(read, MSG.len() * 2 % databuf.len());

    /* log 4 times */
    for _ in 0..4 {
        item1.log();
    }
    assert_eq!(rt.chan_len(1), Some(MSG.len() * 4));

    for _ in 0..6 {
        read = rt.rd_log(1, &mut databuf);
        assert_eq!(read, databuf.len());
    }

    read = rt.rd_log(1, &mut databuf);
    assert_eq!(read, MSG.len() * 4 % databuf.len());

    /* log 6 times */
    for _ in 0..6 {
        item2.log();
    }
    assert_eq!(rt.chan_len(2), Some(MSG.len() * 6));

    for _ in 0..10 {
        read = rt.rd_log(2, &mut databuf);
        assert_eq!(read, databuf.len());
    }

    read = rt.rd_log(2, &mut databuf);
    assert_eq!(read, MSG.len() * 6 % databuf.len());

    /* log 8 times */
    for _ in 0..8 {
        item3.log();
    }
    assert_eq!(rt.chan_len(3), Some(MSG.len() * 8));

    for _ in 0..13 {
        read = rt.rd_log(3, &mut databuf);
        assert_eq!(read, databuf.len());
    }

    read = rt.rd_log(3, &mut databuf);
    assert_eq!(read, MSG.len() * 8 % databuf.len());
}
