use std::cmp::PartialEq;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct VectorError;
impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VECTOR ERROR, SON!")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Row,
    Column,
}
use crate::Direction::{Column, Row};

pub trait Number:
    PartialEq + Add<Output = Self> + ToString + fmt::Display + fmt::Debug + Sized + Copy
{
}
impl<T> Number for T where
    T: PartialEq + Add<Output = T> + ToString + fmt::Display + fmt::Debug + Copy
{
}

#[derive(Debug, Clone)]
pub struct Vector<T: Number> {
    values: Option<Vec<T>>,
    direction: Option<Direction>,
    element_type: String,
    length: usize,
}

impl<T: Number> PartialEq for Vector<T> {
    fn eq(&self, other: &Vector<T>) -> bool {
        if self.element_type != other.element_type {
            return false;
        }

        if self.length != other.length {
            return false;
        }

        // direction equality?

        for (a, b) in self.values.iter().zip(other.values.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl<T: Number> Add for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &Vector<T>) -> Vector<T> {
        // handle this later
        assert_eq!(self.length, rhs.length);

        if self.length == 0 {
            return self.clone();
        }

        // default to row vector?
        let direction = self
            .direction
            .as_ref()
            .unwrap_or(rhs.direction.as_ref().unwrap_or(&Row));

        let sums: Vec<T> = self
            .values
            .as_ref()
            .expect("Non zero length")
            .iter()
            .zip(rhs.values.as_ref().expect("Non-zero length"))
            .map(|(&a, &b)| a + b)
            .collect();
        Vector::build(Vec::from(sums), *direction)
    }
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
            length: 0,
        }
    }

    pub fn build(values: Vec<T>, direction: Direction) -> Vector<T> {
        Vector {
            length: values.len(),
            values: Some(values),
            direction: Some(direction),
            element_type: String::from(std::any::type_name::<T>()),
        }
    }

    pub fn populate(&mut self, values: Vec<T>) -> Result<(), VectorError> {
        match self.values {
            None => {
                self.length = values.len();
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
