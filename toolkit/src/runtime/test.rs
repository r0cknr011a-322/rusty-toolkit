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


const CHAN_NR: usize = 5;
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

    let Some(log4) = rt.chan(4) else {
        panic!("logger channel doesn't exist");
    };

    let log5 = rt.chan(5);
    assert!(log5.is_none());
    let log6 = rt.chan(6);
    assert!(log6.is_none());

    let mut item0 = TestItem::new(log0);
    let mut item1 = TestItem::new(log1);
    let mut item2 = TestItem::new(log2);
    let mut item3 = TestItem::new(log3);
    let mut item4 = TestItem::new(log4);

    let mut databuf: [u8; 256] = array::from_fn(|_| 0);
    let mut head = 0;
    let mut tail = 0;

    /* log 2 times */
    for _ in 0..2 {
        item0.log();
    }
    assert_eq!(rt.chan_len(0), Some(MSG.len() * 2));

    let refbuf: [u8; MSG.len() * 2] = array::from_fn(|i| MSG.as_bytes()[i % MSG.len()]);
    head = 0;
    tail = 0;
    for _ in 0..3 {
        tail += rt.rd_log(0, &mut databuf);
        assert_eq!(tail - head, databuf.len());
        assert_eq!(&databuf, &refbuf[head..tail]);
        head = tail;
    }

    tail += rt.rd_log(0, &mut databuf);
    assert_eq!(rt.chan_len(0), Some(0));
    assert_eq!(tail, MSG.len() * 2);
    assert_eq!(&databuf[..tail - head], &refbuf[head..tail]);

    /* log 4 times */
    for _ in 0..4 {
        item1.log();
    }
    assert_eq!(rt.chan_len(1), Some(MSG.len() * 4));

    let refbuf: [u8; MSG.len() * 4] = array::from_fn(|i| MSG.as_bytes()[i % MSG.len()]);
    head = 0;
    tail = 0;
    for _ in 0..6 {
        tail += rt.rd_log(1, &mut databuf);
        assert_eq!(tail - head, databuf.len());
        assert_eq!(&databuf, &refbuf[head..tail]);
        head = tail;
    }

    tail += rt.rd_log(1, &mut databuf);
    assert_eq!(rt.chan_len(1), Some(0));
    assert_eq!(tail, MSG.len() * 4);
    assert_eq!(&databuf[..tail - head], &refbuf[head..tail]);

    /* log 6 times */
    for _ in 0..6 {
        item2.log();
    }
    assert_eq!(rt.chan_len(2), Some(MSG.len() * 6));

    let refbuf: [u8; MSG.len() * 6] = array::from_fn(|i| MSG.as_bytes()[i % MSG.len()]);
    head = 0;
    tail = 0;
    for _ in 0..10 {
        tail += rt.rd_log(2, &mut databuf);
        assert_eq!(tail - head, databuf.len());
        assert_eq!(&databuf, &refbuf[head..tail]);
        head = tail;
    }

    tail += rt.rd_log(2, &mut databuf);
    assert_eq!(rt.chan_len(2), Some(0));
    assert_eq!(tail, MSG.len() * 6);
    assert_eq!(&databuf[..tail - head], &refbuf[head..tail]);


    /* log 8 times */
    for _ in 0..8 {
        item3.log();
    }
    assert_eq!(rt.chan_len(3), Some(MSG.len() * 8));

    let refbuf: [u8; MSG.len() * 8] = array::from_fn(|i| MSG.as_bytes()[i % MSG.len()]);
    head = 0;
    tail = 0;
    for _ in 0..13 {
        tail += rt.rd_log(3, &mut databuf);
        assert_eq!(tail - head, databuf.len());
        assert_eq!(&databuf, &refbuf[head..tail]);
        head = tail;
    }

    tail += rt.rd_log(3, &mut databuf);
    assert_eq!(rt.chan_len(3), Some(0));
    assert_eq!(tail, MSG.len() * 8);
    assert_eq!(&databuf[..tail - head], &refbuf[head..tail]);

    /* log 10 times */
    for _ in 0..10 {
        item4.log();
    }
    assert_eq!(rt.chan_len(4), rt.chan_capacity(4));

    let refbuf: [u8; MSG.len() * 10] = array::from_fn(|i| MSG.as_bytes()[i % MSG.len()]);
    head = 0;
    tail = 0;
    for _ in 0..16 {
        tail += rt.rd_log(4, &mut databuf);
        assert_eq!(tail - head, databuf.len());
        assert_eq!(&databuf, &refbuf[head..tail]);
        head = tail;
    }

    tail += rt.rd_log(4, &mut databuf);
    assert_eq!(rt.chan_len(4), Some(0));
}
