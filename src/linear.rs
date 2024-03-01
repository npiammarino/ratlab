use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct VectorError;
impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VECTOR ERROR, SON!")
    }
}

#[derive(Debug, Clone)]
pub struct MatrixError;
impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MATRIX ERROR, SON!")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Row,
    Column,
}
use crate::Direction::{Column, Row};

pub trait Number:
    PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + ToString
    + fmt::Display
    + fmt::Debug
    + Sized
    + Copy
{
}
impl<T> Number for T where
    T: PartialEq
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + ToString
        + fmt::Display
        + fmt::Debug
        + Sized
        + Copy
{
}

#[derive(Debug, Clone)]
pub struct Vector<T: Number> {
    values: Option<Vec<T>>,
    direction: Option<Direction>,
    element_type: String,
    length: usize,
}

#[derive(Debug, Clone)]
pub struct Matrix<T: Number> {
    element_type: String,
    values: Option<Vec<T>>,
    columns: usize,
}

impl<T: Number> PartialEq for Vector<T> {
    fn eq(&self, other: &Vector<T>) -> bool {
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

impl<T: Number> Sub for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &Vector<T>) -> Vector<T> {
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
            .map(|(&a, &b)| a - b)
            .collect();
        Vector::build(Vec::from(sums), *direction)
    }
}

impl<T: Number> Mul for &Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &Vector<T>) -> Matrix<T> {
        // check values here

        match (self.direction.as_ref(), rhs.direction.as_ref()) {
            (Some(dir1), Some(dir2)) => match (dir1, dir2) {
                (Column, Row) => {
                    let mut result: Vec<T> = Vec::new();
                    for &j in rhs
                        .values
                        .as_ref()
                        .expect("Should be cautght by asserts")
                        .iter()
                    {
                        for &i in self
                            .values
                            .as_ref()
                            .expect("Should be caught by asserts")
                            .iter()
                        {
                            result.push(i * j);
                        }
                    }
                    Matrix::<T>::build(
                        result,
                        self.values
                            .as_ref()
                            .expect("Should be caught by asserts")
                            .len(),
                    )
                    .expect("Constructed")
                }
                (Row, Column) => {
                    let mut products = self
                        .values
                        .as_ref()
                        .expect("Should be caught by asserts")
                        .iter()
                        .zip(
                            rhs.values
                                .as_ref()
                                .expect("Should be caught by asserts")
                                .iter(),
                        )
                        .map(|(&a, &b)| a * b);

                    let mut sum = products.next().expect("Should be caught by asserts");
                    for product in products {
                        sum = sum + product;
                    }

                    Matrix::<T>::build(vec![sum as T], 1).expect("Constructed")
                }
                _ => panic!("Improper direction for product"),
            },
            _ => panic!("Missing direction."),
        }

        // do calc
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

impl<T: Number> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix {
            element_type: String::from(std::any::type_name::<T>()),
            values: None,
            columns: 0,
        }
    }

    pub fn build(values: Vec<T>, columns: usize) -> Result<Matrix<T>, MatrixError> {
        if values.len() % columns == 0 {
            Ok(Matrix {
                element_type: String::from(std::any::type_name::<T>()),
                values: Some(values),
                columns,
            })
        } else {
            Err(MatrixError)
        }
    }
}

impl<T: Number> PartialEq for Matrix<T> {
    fn eq(&self, other: &Matrix<T>) -> bool {
        if self.columns != other.columns {
            return false;
        }

        if let (Some(v1), Some(v2)) = (self.values.as_ref(), other.values.as_ref()) {
            for (a, b) in v1.iter().zip(v2.iter()) {
                if a != b {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl<T: Number> Add for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: &Matrix<T>) -> Matrix<T> {
        match (self.values.as_ref(), rhs.values.as_ref()) {
            (None, _) => rhs.clone(),
            (_, None) => self.clone(),
            (Some(v1), Some(v2)) => {
                assert_eq!(v1.len(), v2.len());
                assert_eq!(self.columns, rhs.columns);
                let new_values: Vec<T> = v1.iter().zip(v2.iter()).map(|(&a, &b)| a + b).collect();
                Matrix::build(new_values, self.columns).expect("Error should be caught by asserts")
            }
        }
    }
}

impl<T: Number> Sub for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: &Matrix<T>) -> Matrix<T> {
        match (self.values.as_ref(), rhs.values.as_ref()) {
            (None, _) => rhs.clone(),
            (_, None) => self.clone(),
            (Some(v1), Some(v2)) => {
                assert_eq!(v1.len(), v2.len());
                assert_eq!(self.columns, rhs.columns);
                let new_values: Vec<T> = v1.iter().zip(v2.iter()).map(|(&a, &b)| a - b).collect();
                Matrix::build(new_values, self.columns).expect("Error should be caught by asserts")
            }
        }
    }
}

impl<T: Number> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.values.as_ref() {
            Some(values) => {
                let vals = format!("{:?}", values);
                let matrix: Vec<&str> = vals.split(" ").collect();
                let mut result: String = String::new();
                for (i, s) in matrix.iter().enumerate() {
                    if i == matrix.len() - 1 {
                        result.push_str(&format!("{} |", s.replace(&['[', ',', ']'], ""))[..]);
                    } else if (i + 1) % self.columns == 0 {
                        result.push_str(&format!("{} |\n|  ", s.replace(&['[', ',', ']'], ""))[..]);
                    } else {
                        result.push_str(&format!("{},\t", s.replace(&['[', ',', ']'], ""))[..]);
                    }
                }
                write!(f, "<{}>\n|  {}", self.element_type, result)
            }
            None => write!(f, "<{}>\n[ ]", self.element_type),
        }
    }
}
