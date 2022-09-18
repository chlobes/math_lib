pub use std::ops::*;
pub use std::marker::Copy;
pub use crate::traits::*;
pub use std::fmt;
pub use serde::{Serialize,Deserialize,Serializer,Deserializer,ser::SerializeStruct};
pub use crate::traits::NiceFmt;
pub use std::iter::{Product,Sum};
pub use std::str::FromStr;
pub const BRACKETS: &[char] = &['(', ')', '[', ']', '{', '}'];

pub use crate::vector::*;
pub use crate::*;

#[cfg(feature = "raylib")]
pub use raylib::ffi::{Vector2,Vector3,Vector4,Color,Quaternion as RaylibQuaternion,Matrix};
#[cfg(feature = "raylib")]
pub use raylib::prelude::{Vector2 as Vector2_,Vector3 as Vector3_,Vector4 as Vector4_,Color as Color_,Quaternion as RaylibQuaternion_,Matrix as Matrix_};
