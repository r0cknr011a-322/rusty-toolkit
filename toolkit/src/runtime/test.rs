use core::time::{ Duration };
use core::array::{ self };

use crate::runtime::{ Timer, Runtime, RuntimeInner, RuntimeRef };
use crate::collection::deque::{ Deque };


#[derive(Default)]
struct TestTimer {

}

impl Timer for TestTimer {
    fn time(&mut self) -> Duration {
        Duration::default()
    }
}

const LOG: &str = r#"
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
        write!(self.rt, "{}", LOG);
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

    let item0 = TestItem::new(log0);
    let item1 = TestItem::new(log1);
    let item2 = TestItem::new(log2);
    let item3 = TestItem::new(log3);
    let mut itembuf = [
        item0, item1, item2, item3,
    ];

    for (idx, item) in itembuf.iter_mut().enumerate() {
        item.log();
        assert_eq!(rt.chan_len(idx), LOG.len());

        let mut databuf: [u8; 256] = array::from_fn(|_| 0);
        let mut total = 0;
        for _ in 0..LOG.len() / databuf.len() {
            let cnt = rt.rd_log(&mut databuf, idx);
            assert_eq!(cnt, databuf.len());
            total += cnt;
            assert_eq!(rt.chan_len(idx), LOG.len() - total);
        }

        let cnt = rt.rd_log(&mut databuf, idx);
        assert_eq!(cnt, LOG.len() % databuf.len());
    }
}
