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
pub fn distance_squared<T, V: Vector<T>>(a: V, b: V) -> T { a.distance_squared(b) }

pub use raylib::ffi::{Vector2,Vector3,Vector4,Color,Quaternion as RaylibQuaternion,Matrix};
pub use raylib::prelude::{Vector2 as Vector2_,Vector3 as Vector3_,Vector4 as Vector4_,Color as Color_,Quaternion as RaylibQuaternion_,Matrix as Matrix_};
