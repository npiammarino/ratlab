extern crate linear;

use linear::{Direction::*, Vector};

fn main() {
    let mut x = Vector::<i32>::new();
    let values = vec![-3, 5, 21];
    x.populate(values).unwrap();
    x.direct(Column);

    let y = Vector::build(vec![4u8, 3, 7], Row);
    let z = Vector::<f64>::new();

    let mut no_dir = Vector::<usize>::new();
    no_dir.populate(vec![3, 9, 8]).unwrap();

    let mut no_vals = Vector::<i8>::new();
    no_vals.direct(Row);

    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");

    println!("no_dir = {no_dir}");
    println!("no_vals = {no_vals}");
}
