pub use array_tuple::ArrayTuple;
pub use std::ops::*;
pub use std::marker::Copy;
pub use crate::traits::*;
pub use std::fmt;
pub use serde::{Serialize,Deserialize};
pub use crate::traits::NiceFmt;
pub use std::iter::{Product,Sum};
pub use std::str::FromStr;
pub const BRACKETS: &[char] = &['(', ')', '[', ']', '{', '}'];

pub fn dot<T, V: Vector<T>>(a: V, b: V) -> T { a.dot(&b) }
pub fn distance<T, V: Vector<T>>(a: V, b: V) -> T { a.distance(&b) }
pub fn distance_squared<T, V: Vector<T>>(a: V, b: V) -> T { let x = a - b; x.dot(&x) }
