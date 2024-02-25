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
