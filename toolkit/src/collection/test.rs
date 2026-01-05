use crate::collection::deque::{ Deque, DequeError };

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
fn deque_push_pop() {
    // [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C ]
    let buf: [TestItem; ITEMNR] = core::array::from_fn(
        |i| TestItem::new(i.try_into().unwrap())
    );

    let mut deque = Deque::<TestItem, ITEMNR>::default();
    assert_eq!(deque.capacity(), ITEMNR);

    assert_eq!(deque.len(), 0);
    assert_eq!(deque.get_front(), (0, true));
    assert_eq!(deque.get_back(), (12, true));

    //   |>
    // [ x, x, x, x, x, x, x, x, 4, 3, 2, 1, 0 ]
    //                       <|
    for i in 0..5 {
        let res = deque.push_back(buf[i]);
        assert_eq!(res, Ok(()));
    }

    assert_eq!(deque.len(), 5);
    assert_eq!(deque.get_front(), (0, true));
    assert_eq!(deque.get_back(), (7, true));

    //                           |>
    // [ x, x, x, x, x, x, x, x, x, x, x, x, x ]
    //                       <|
    for i in 0..5 {
        let item = deque.pop_front();
        assert_eq!(item, Some(buf[i]));
    }

    assert_eq!(deque.len(), 0);
    assert_eq!(deque.get_front(), (8, true));
    assert_eq!(deque.get_back(), (7, true));

    //                        |>
    // [ 7, 6, 5, 4, 3, 2, 1, 0, C, B, A, 9, 8 ]
    //                          <|
    for i in 0..13 {
        let res = deque.push_back(buf[i]);
        assert_eq!(res, Ok(()));
    }

    assert_eq!(deque.len(), 13);
    assert_eq!(deque.get_front(), (7, false));
    assert_eq!(deque.get_back(), (8, false));

    assert_eq!(deque.push_back(TestItem::default()), Err(DequeError::Fatal));
    assert_eq!(deque.push_back(TestItem::default()), Err(DequeError::Fatal));
    assert_eq!(deque.push_front(TestItem::default()), Err(DequeError::Fatal));
    assert_eq!(deque.push_front(TestItem::default()), Err(DequeError::Fatal));

    assert_eq!(deque.len(), 13);
    assert_eq!(deque.get_front(), (7, false));
    assert_eq!(deque.get_back(), (8, false));

    //                           |>
    // [ x, x, x, 4, 3, 2, 1, 0, x, x, x, x, x ]
    //        <|
    for i in 0..8 {
        let item = deque.pop_back();
        assert_eq!(item, Some(buf[ITEMNR - 1 - i]));
    }

    assert_eq!(deque.len(), 5);
    assert_eq!(deque.get_front(), (8, true));
    assert_eq!(deque.get_back(), (2, true));

    //         |>
    // [ 5, 6, 7, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4 ]
    //           <|
    for i in 0..8 {
        let res = deque.push_front(buf[i]);
        assert_eq!(res, Ok(()));
    }

    assert_eq!(deque.len(), 13);
    assert_eq!(deque.get_front(), (2, false));
    assert_eq!(deque.get_back(), (3, false));
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
    assert_eq!(iter.get_front(), 12);
    assert_eq!(iter.get_back(), 12);
    assert_eq!(iter.size_hint(), (0, Some(0)));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);

    //   |>
    // [ x, x, x, x, x, x, x, x, x, x, x, x, 0 ]
    //                                   <|
    let res = deque.push_back(buf[0]);

    //                                      <|
    // [ x, x, x, x, x, x, x, x, x, x, x, x, 0 ]
    //                                       |>
    let mut iter = deque.into_iter();
    assert_eq!(iter.get_front(), 12);
    assert_eq!(iter.get_back(), 12);
    assert_eq!(iter.size_hint(), (1, Some(1)));

    assert_eq!(iter.next(), Some(buf[0]));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);

    //                                      <|
    // [ x, x, x, x, x, x, x, x, x, x, x, x, 0 ]
    //                                       |>
    let mut iter = deque.into_iter();
    assert_eq!(iter.next_back(), Some(buf[0]));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);

    //   |>
    // [ x, B, A, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 ]
    //     <|
    for item in &buf[1..ITEMNR - 1] {
        let res = deque.push_back(*item);
        assert_eq!(res, Ok(()));
    }
    assert_eq!(deque.len(), ITEMNR - 1);

    //                                      <|
    // [ x, B, A, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 ]
    //      |>
    let mut iter = deque.into_iter();
    assert_eq!(iter.size_hint(), (ITEMNR - 1, Some(ITEMNR - 1)));
    assert_eq!(iter.get_front(), 12);
    assert_eq!(iter.get_back(), 1);

    //                          <|
    // [ x, B, A, 9, 8, 7, 6, 5, 4, x, x, x, x ]
    //      |>
    for i in 0..4 {
        assert_eq!(iter.next(), Some(buf[0 + i]));
    }
    assert_eq!(iter.size_hint(), (8, Some(8)));
    assert_eq!(iter.get_front(), 8);
    assert_eq!(iter.get_back(), 1);

    //                          <|
    // [ x, x, x, x, x, 7, 6, 5, 4, x, x, x, x ]
    //                  |>
    for i in 0..4 {
        assert_eq!(iter.next_back(), Some(buf[ITEMNR - 2 - i]));
    }
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(iter.get_front(), 8);
    assert_eq!(iter.get_back(), 5);

    //                 <|
    // [ x, x, x, x, x, x, x, x, x, x, x, x, x ]
    //                  |>
    for i in 0..4 {
        assert_eq!(iter.next(), Some(buf[4 + i]));
    }
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.get_front(), 5);
    assert_eq!(iter.get_back(), 5);

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);

    //   |>
    // [ C, B, A, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 ]
    //     <|
    let res = deque.push_back(buf[ITEMNR - 1]);
    assert_eq!(res, Ok(()));
    assert_eq!(deque.len(), ITEMNR);

    //                                      <|
    // [ C, B, A, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 ]
    //   |>
    let mut iter = deque.into_iter();
    assert_eq!(iter.size_hint(), (13, Some(13)));
    assert_eq!(iter.get_front(), 12);
    assert_eq!(iter.get_back(), 0);

    //                          <|
    // [ C, B, A, 9, 8, 7, 6, 5, 4, x, x, x, x ]
    //   |>
    for i in 0..4 {
        assert_eq!(iter.next(), Some(buf[0 + i]));
    }
    assert_eq!(iter.size_hint(), (9, Some(9)));
    assert_eq!(iter.get_front(), 8);
    assert_eq!(iter.get_back(), 0);

    //                          <|
    // [ x, x, x, x, x, x, 6, 5, 4, x, x, x, x ]
    //                     |>
    for i in 0..6 {
        assert_eq!(iter.next_back(), Some(buf[ITEMNR - 1 - i]));
    }
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.get_front(), 8);
    assert_eq!(iter.get_back(), 6);

    //                    <|
    // [ x, x, x, x, x, x, x, x, x, x, x, x, x ]
    //                     |>
    for i in 0..3 {
        assert_eq!(iter.next(), Some(buf[4 + i]));
    }
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.get_front(), 6);
    assert_eq!(iter.get_back(), 6);

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);
}

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
    assert_eq!(deque.get_front(), (0, true));
    assert_eq!(deque.get_back(), (5, true));
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
