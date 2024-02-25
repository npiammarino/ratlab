use std::fmt;

#[derive(Debug, Clone)]
pub struct VectorError;
impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VECTOR ERROR, SON!")
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Row,
    Column,
}
use crate::Direction::{Column, Row};

pub trait Number: ToString + fmt::Display + fmt::Debug {}
impl<T> Number for T where T: ToString + fmt::Display + fmt::Debug {}

#[derive(Debug, Clone)]
pub struct Vector<T: Number> {
    values: Option<Vec<T>>,
    direction: Option<Direction>,
    element_type: String,
}

impl<T: Number> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.direction, &self.values) {
            (Some(dir), Some(_values)) => match dir {
                // these need adjustments for length
                Column => {
                    let values = format!("{:?}", self.values.as_ref().unwrap());
                    let column: String = values
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .join("\n   ")
                        .replace(&['[', ']'], "");

                    write!(f, "<{}>\n[  {}  ]", self.element_type, column)
                }
                Row => {
                    write!(
                        f,
                        "<{}> {:?}",
                        self.element_type,
                        self.values.as_ref().unwrap()
                    )
                }
            },
            _ => {
                if let Some(dir) = &self.direction {
                    write!(f, "<{}> {dir:?} []", self.element_type)
                } else if let Some(vals) = &self.values {
                    write!(f, "<{}> Undirected {vals:?}", self.element_type)
                } else {
                    write!(f, "<{}> Undirected []", self.element_type)
                }
            }
        }
    }
}

impl<T: Number> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            values: None,
            direction: None,
            element_type: String::from(std::any::type_name::<T>()),
        }
    }

    pub fn build(values: Vec<T>, direction: Direction) -> Vector<T> {
        Vector {
            values: Some(values),
            direction: Some(direction),
            element_type: String::from(std::any::type_name::<T>()),
        }
    }

    pub fn populate(&mut self, values: Vec<T>) -> Result<(), VectorError> {
        match self.values {
            None => {
                self.values = Some(values);
                Ok(())
            }
            _ => Err(VectorError),
        }
    }

    pub fn direct(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }
}
