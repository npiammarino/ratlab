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
fn test_add_vector() {
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

#[test]
fn test_sub_vector() {
    let x = Vector::<i32>::build(vec![7, 6, 5], Row);
    let y = Vector::<i32>::build(vec![3, 2, 1], Column);

    let r1 = Vector::<i32>::build(vec![4, 4, 4], Row);
    let r2 = Vector::<i32>::build(vec![4, 4, 4], Column);
    let r3 = Vector::<i32>::build(vec![-4, -4, -4], Row);
    let r4 = Vector::<i32>::build(vec![-4, -4, -4], Column);

    assert_eq!(&x - &y, r1);
    assert_eq!(&x - &y, r2);
    assert_ne!(&x - &y, r3);
    assert_ne!(&x - &y, r4);

    assert_eq!(&y - &x, r3);
    assert_eq!(&y - &x, r4);
    assert_ne!(&y - &x, r1);
    assert_ne!(&y - &x, r2);
}

#[test]
fn test_inits_matrix() {
    let m = Matrix::<i32>::new();
    assert_eq!(format!("{m}"), "<i32>\n[ ]");

    let n: Matrix<i32> = Matrix::build(vec![1i32, 2, 3, 4], 2).ok().unwrap();
    assert_eq!(format!("{n}"), "<i32>\n|  1,\t2 |\n|  3,\t4 |");

    match Matrix::build(vec![1, 2, 3], 2) {
        Ok(_) => panic!("accepting bad matrix dimensions"),
        _ => {}
    }
}

#[test]
fn test_add_matrix() {
    let a = Matrix::<i32>::build(vec![1, 2, 3, 4], 2).unwrap();
    let b = Matrix::<i32>::build(vec![4, 3, 2, 1], 2).unwrap();
    let sum = Matrix::<i32>::build(vec![5, 5, 5, 5], 2).unwrap();

    assert_eq!(sum, &a + &b);
    assert_ne!(a, b);
}
