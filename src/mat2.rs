use crate::prelude::*;

use crate::vec2::*;

#[repr(C)]
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Eq,Ord,Hash,Serialize,Deserialize)]
pub struct Mat2<T> {
	pub x: Vec2<T>,
	pub y: Vec2<T>,
}

impl<T> Mat2<T> {
	pub fn det(self) -> T
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T>
	{
		let Self{ x,y } = self;
		x.x * y.y - x.y * y.x
	}
	
	pub fn inv(self) -> Self
	where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Neg<Output=T>
	{
		let Self{ x,y } = self;
		mat2(
			vec2(y.y, -y.x) / self.det(),
			vec2(-x.y, x.x) / self.det(),
		)
	}
	
	pub fn ident() -> Self
		where T: One + Default
	{
		mat2(
			vec2(T::one(), T::default()),
			vec2(T::default(), T::one()),
		)
	}
	
	pub fn apply_to(self, v: Vec2<T>) -> Vec2<T>
		where T: Copy + Mul<Output=T> + Add<Output=T>
	{
		vec2(
			dot(self.x, v),
			dot(self.y, v),
		)
	}
	
	pub fn transpose(self) -> Self {
		mat2(
			vec2(self.x.x, self.y.x),
			vec2(self.x.y, self.y.y),
		)
	}
}

impl Mat2<f32> {
	pub fn rotate(angle: f32) -> Self {
		mat2(
			vec2(angle.cos(), -angle.sin()),
			vec2(angle.sin(), angle.cos()),
		)
	}
}

impl Mat2<f64> {
	pub fn rotate(angle: f64) -> Self {
		mat2(
			vec2(angle.cos(), -angle.sin()),
			vec2(angle.sin(), angle.cos()),
		)
	}
}

pub fn mat2<T>(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
	Mat2 { x: x, y: y }
}

impl<T> Default for Mat2<T>
	where T: One + Default
{
	fn default() -> Self {
		Mat2::ident()
	}
}

impl<T> Mul<Mat2<T>> for Mat2<T>
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	type Output = Self;
	
	fn mul(self, other: Self) -> Self {
		let t = other.transpose();
		mat2(
			vec2(dot(self.x,t.x), dot(self.x,t.y)),
			vec2(dot(self.y,t.x), dot(self.y,t.y)),
		)
	}
}

impl<T> Mul<Vec2<T>> for Mat2<T>
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	type Output = Vec2<T>;
	
	fn mul(self, v: Vec2<T>) -> Vec2<T> {
		self.apply_to(v)
	}
}

impl<T> ArrayTuple for Mat2<T> {
	type Array = [[T; 2]; 2];
	type Tuple = ((T,T),(T,T));
	fn into_array(self) -> Self::Array {	let Self{x,y} = self; [x.into_array(),y.into_array()] }
	fn into_tuple(self) -> Self::Tuple { let Self{x,y} = self; (x.into_tuple(),y.into_tuple()) }
}

macro_rules! convert {
	($T: ty, $($U: ident),+) => {$(
		impl Mat2<$T> {
			pub fn $U(self) -> Mat2<$U> {
				mat2(self.x.$U(), self.y.$U())
			}
		}
	)+}
}

convert!(u8, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(u16, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(u32, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(u64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(usize, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(i8, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(i16, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(i32, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(i64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(isize, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(f32, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
convert!(f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
