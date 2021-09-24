#![allow(incomplete_features)]
#![feature(decl_macro,specialization)]
#![allow(clippy::type_complexity)]

extern crate serde;
extern crate array_tuple;

pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod quaternion;
pub mod traits;

mod prelude;
