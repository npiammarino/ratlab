extern crate linear;

use linear::{Direction::*, *};

#[test]
fn test_inits_vector() {
    let u = Vector::<u8>::build(vec![1, 2, 3], Column);
    assert_eq!(format!("{u}"), "<u8>\n[  1,\n   2,\n   3  ]");

    let mut v = Vector::<i32>::new();
    assert_eq!(format!("{v}"), "<i32> Undirected []");
    v.direct(Row);
    assert_eq!(format!("{v}"), "<i32> Row []");

    let mut w = Vector::<usize>::new();
    w.populate(vec![1, 2, 3]).unwrap();
    assert_eq!(format!("{w}"), "<usize> Undirected [1, 2, 3]");
}

#[test]
fn test_eq() {
    let t1 = Vector::<i32>::build(vec![4, 4, 4], Row);

    let r1 = Vector::<i32>::build(vec![4, 4, 4], Row);
    let r2 = Vector::<i32>::build(vec![4, 4, 4], Column);

    let bad1 = Vector::<i32>::build(vec![3, 3, 3], Row);
    let bad2 = Vector::<i32>::build(vec![4, 4], Row);

    assert_eq!(t1, r1);
    assert_eq!(t1, r2);

    assert_ne!(t1, bad1);
    assert_ne!(t1, bad2);
}

#[test]
fn test_add() {
    let x = Vector::<i32>::build(vec![1, 2, 3], Row);
    let y = Vector::<i32>::build(vec![3, 2, 1], Column);

    let r1 = Vector::<i32>::build(vec![4, 4, 4], Row);
    let r2 = Vector::<i32>::build(vec![4, 4, 4], Column);

    let bad1 = Vector::<i32>::build(vec![3, 3, 3], Row);
    let bad2 = Vector::<i32>::build(vec![4, 4], Row);

    assert_eq!(&x + &y, r1);
    assert_eq!(&x + &y, r2);

    assert_ne!(&x + &y, bad1);
    assert_ne!(&x + &y, bad2);
}
