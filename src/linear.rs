use std::fmt;

#[derive(Debug, Clone)]
pub struct VectorError;
impl fmt::Display for VectorError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "VECTOR ERROR, SON!")
    }
}

#[derive(Debug)]
pub enum Direction{Row, Column}
use crate::Direction::{Row, Column};



#[derive(Debug)]
pub struct Vector<T> {
    values: Option<Vec<T>>,
    direction: Option<Direction>,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T>{
	Vector{
	    values: None,
	    direction: None
	}
    }
    
    pub fn build(values: Vec<T>, direction: Direction) -> Vector<T> {
	Vector {
	    values: Some(values),
	    direction: Some(direction)
	}
    }

    pub fn populate(&mut self, values: Vec<T>) -> Result<(), VectorError>{
	match self.values {
	    None => {
		self.values = Some(values);
		Ok(())
	    },
	    _ => Err(VectorError)
	}

    }

    pub fn direct(&mut self, direction: Direction) {
	self.direction = Some(direction);
    }
}

fn main(){
    let mut x = Vector::<i32>::new();
    let values = vec![-3,5,21];
    x.populate(values).unwrap();
    x.direct(Column);

    println!("{x:?}");
}
