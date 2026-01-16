use crate::collection::deque::{ Deque };

const ITEMNR: usize = 13;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct TestItem {
    id: u64,
    data: u64,
}

impl TestItem {
    fn new(data: u8) -> Self {
        Self {
            id: 0x1BAD_C0DE_0000_0000 | data as u64,
            data: 0x2BAD_C0DE_0000_0000 | data as u64,
        }
    }
}

#[test]
fn queue_push_pop() {
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    let buf: [TestItem; ITEMNR] = core::array::from_fn(
        |i| TestItem::new(i.try_into().unwrap())
    );

    let mut deque = Deque::<TestItem, ITEMNR>::default();
    assert_eq!(deque.capacity(), ITEMNR);

    //   |>
    // [ x, x, x, x, x, x, x, x, 4, 3, 2, 1, 0 ]
    //   |>
    assert_eq!(deque.len(), 0);
    assert_eq!(deque.head(), 0);
    assert_eq!(deque.tail(), 0);

    assert_eq!(deque.pop(), None);
    assert_eq!(deque.pop(), None);
    assert_eq!(deque.pop(), None);
    assert_eq!(deque.pop(), None);

    assert_eq!(deque.len(), 0);
    assert_eq!(deque.head(), 0);
    assert_eq!(deque.tail(), 0);

    for i in 0..5 {
        deque.push(buf[i]);
    }

    //                  |>
    // [ 0, 1, 2, 3, 4, x, x, x, x, x, x, x, x ]
    //   |>
    assert_eq!(deque.len(), 5);
    assert_eq!(deque.head(), 0);
    assert_eq!(deque.tail(), 5);

    for i in 0..5 {
        let item = deque.pop();
        assert_eq!(item, Some(buf[i]));
    }

    //                  |>
    // [ x, x, x, x, x, x, x, x, x, x, x, x, x ]
    //                  |>
    assert_eq!(deque.len(), 0);
    assert_eq!(deque.head(), 5);
    assert_eq!(deque.tail(), 5);

    assert_eq!(deque.pop(), None);
    assert_eq!(deque.pop(), None);
    assert_eq!(deque.pop(), None);
    assert_eq!(deque.pop(), None);

    for i in 0..ITEMNR {
        deque.push(buf[i]);
    }

    //                  |>
    // [ 8, 9, A, B, C, 0, 1, 2, 3, 4, 5, 6, 7 ]
    //                  |>
    assert_eq!(deque.len(), 13);
    assert_eq!(deque.head(), 5);
    assert_eq!(deque.tail(), 5);

    deque.push(TestItem::default());
    deque.push(TestItem::default());
    deque.push(TestItem::default());
    deque.push(TestItem::default());

    assert_eq!(deque.len(), 13);
    assert_eq!(deque.head(), 5);
    assert_eq!(deque.tail(), 5);

    for i in 0..8 {
        let item = deque.pop();
        assert_eq!(item, Some(buf[i]));
    }

    //                  |>
    // [ 8, 9, A, B, C, x, x, x, x, x, x, x, x ]
    //   |>
    assert_eq!(deque.len(), 5);
    assert_eq!(deque.head(), 0);
    assert_eq!(deque.tail(), 5);

    for i in 0..8 {
        deque.push(buf[i]);
    }

    //   |>
    // [ 8, 9, A, B, C, 0, 1, 2, 3, 4, 5, 6, 7 ]
    //   |>
    assert_eq!(deque.len(), ITEMNR);
    assert_eq!(deque.head(), 0);
    assert_eq!(deque.tail(), 0);
}

#[test]
fn deque_into_iter() {
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    let buf: [TestItem; ITEMNR] = core::array::from_fn(
        |i| TestItem::new(i.try_into().unwrap())
    );

    let mut deque = Deque::<TestItem, ITEMNR>::default();

    // empty iter
    let mut iter = deque.into_iter();
    assert_eq!(iter.head(), 0);
    assert_eq!(iter.tail(), 0);
    assert_eq!(iter.size_hint(), (0, Some(0)));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);

    deque.push(buf[0]);

    //      |>
    // [ 0, x, x, x, x, x, x, x, x, x, x, x, x ]
    //   |>
    let mut iter = deque.into_iter();
    assert_eq!(iter.head(), 0);
    assert_eq!(iter.tail(), 1);
    assert_eq!(iter.size_hint(), (1, Some(1)));

    assert_eq!(iter.next(), Some(buf[0]));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);

    //                                      <|
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, x ]
    //   |>
    for item in &buf[1..ITEMNR - 1] {
        deque.push(*item);
    }
    assert_eq!(deque.len(), ITEMNR - 1);

    let mut iter = deque.into_iter();
    assert_eq!(iter.size_hint(), (ITEMNR - 1, Some(ITEMNR - 1)));
    assert_eq!(iter.head(), 0);
    assert_eq!(iter.tail(), 12);

    //                                      <|
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, x ]
    //               |>
    for i in 0..4 {
        assert_eq!(iter.next(), Some(buf[0 + i]));
    }
    assert_eq!(iter.size_hint(), (8, Some(8)));
    assert_eq!(iter.head(), 4);
    assert_eq!(iter.tail(), 12);

    //                          <|
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, x ]
    //               |>
    for i in 0..4 {
        assert_eq!(iter.next_back(), Some(buf[ITEMNR - 2 - i]));
    }
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(iter.head(), 4);
    assert_eq!(iter.tail(), 8);

    //                          <|
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, x ]
    //                           |>
    for i in 0..4 {
        assert_eq!(iter.next(), Some(buf[4 + i]));
    }
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.head(), 8);
    assert_eq!(iter.tail(), 8);

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);

    //                                       |>
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    //   |>
    deque.push(buf[ITEMNR - 1]);
    assert_eq!(deque.len(), ITEMNR);

    //                                      <|
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    //   |>
    let mut iter = deque.into_iter();
    assert_eq!(iter.size_hint(), (13, Some(13)));
    assert_eq!(iter.head(), 0);
    assert_eq!(iter.tail(), 0);

    //                                      <|
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    //               |>
    for i in 0..4 {
        assert_eq!(iter.next(), Some(buf[0 + i]));
    }
    assert_eq!(iter.size_hint(), (9, Some(9)));
    assert_eq!(iter.head(), 4);
    assert_eq!(iter.tail(), 0);

    //                       <|
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    //               |>
    for i in 0..6 {
        assert_eq!(iter.next_back(), Some(buf[ITEMNR - 1 - i]));
    }
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.head(), 4);
    assert_eq!(iter.tail(), 7);

    //                       <|
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    //                        |>
    for i in 0..3 {
        assert_eq!(iter.next(), Some(buf[4 + i]));
    }
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.head(), 7);
    assert_eq!(iter.tail(), 7);

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);
}

/*
#[test]
fn deque_from_iter() {
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    let buf: [TestItem; ITEMNR] = core::array::from_fn(
        |i| TestItem::new(i.try_into().unwrap())
    );

    let deque: Deque<TestItem, ITEMNR> = buf
        .into_iter()
        .filter(|e| e.data % 2 == 0)
        .collect();
    assert_eq!(deque.capacity(), ITEMNR);
    assert_eq!(deque.len(), 7);
    assert_eq!(deque.head(), 0);
    assert_eq!(deque.tail(), 5);
}

#[test]
fn deque_equals() {
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    let buf: [TestItem; ITEMNR] = core::array::from_fn(|i|
        TestItem::new(i.try_into().unwrap())
    );

    let mut ldeque = Deque::<TestItem, ITEMNR>::default();
    for i in 0..4 {
        ldeque.push_front(buf[4 + i]);
    }
    for i in 0..4 {
        ldeque.push_back(buf[3 - i]);
    }
    //               |>
    // [ 4, 5, 6, 7, x, x, x, x, x, 0, 1, 2, 3 ]
    //                          <|
    assert_eq!(ldeque.len(), 8);

    let mut rdeque = Deque::<TestItem, ITEMNR>::default();
    for i in 0..ITEMNR {
        rdeque.push_front(buf[i]);
    }

    assert_ne!(ldeque, rdeque);

    for i in 0..2 {
        rdeque.pop_back();
    }
    for i in 0..ITEMNR {
        rdeque.pop_front();
    }
    for i in 0..8 {
        rdeque.push_front(buf[i]);
    }
    //                                 |>
    // [ x, x, 0, 1, 2, 3, 4, 5, 6, 7, x, x, x ]
    //     <|
    assert_eq!(rdeque.len(), 8);

    assert_eq!(ldeque, rdeque);
    assert_eq!(rdeque, ldeque);
}
*/
