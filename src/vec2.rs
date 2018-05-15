use std::ops::{Index, IndexMut, Div, DivAssign, Mul, MulAssign, Add, AddAssign, Sub, SubAssign};
use std::marker::Copy;
use std::convert::Into;

use traits::Sqrt;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2<T> {
	pub fn magnitude(self) -> T
		where T: Copy + Sqrt<T> + Mul<Output=T> + Add<Output=T>
	{
		(self.x * self.x + self.y * self.y).sqrt()
	}
	
	pub fn normalize(self) -> Self
		where T: Copy + Sqrt<T> + Div<Output=T> + Mul<Output=T> + Add<Output=T>
	{
		vec2(self.x / self.magnitude(), self.y / self.magnitude())
	}
	
	pub fn into_workaround<U>(self) -> Vec2<U>
		where T: Into<U>
	{
		vec2(self.x.into(), self.y.into())
	}
	
	pub fn zero() -> Self
		where T: Default
	{
		Self::default()
	}
	
	pub fn into_array(self) -> [T; 2] {
		[self.x, self.y]
	}
}

impl Vec2<f64> {
	pub fn ceil(self) -> Self {
		vec2(self.x.ceil(), self.y.ceil())
	}
}

impl Vec2<f32> {
	pub fn ceil(self) -> Self {
		vec2(self.x.ceil(), self.y.ceil())
	}
}

pub fn vec2<T>(x: T, y: T) -> Vec2<T>
{
	Vec2 { x: x, y: y }
}

pub fn dot<T>(v: Vec2<T>, u: Vec2<T>) -> T
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	v.x * u.x + v.y * u.y
}

pub fn distance<T>(v: Vec2<T>, u: Vec2<T>) -> T
	where T: Copy + Sqrt<T> + Mul<Output=T> + Add<Output=T> + Sub<Output=T>
{
	(v - u).magnitude()
}

impl<T> Div<T> for Vec2<T>
	where T: Copy + Div<Output=T>
{
	type Output = Vec2<T>;
	
	fn div(self, scalar: T) -> Vec2<T> {
		vec2(self.x / scalar, self.y / scalar)
	}
}

impl<T> DivAssign<T> for Vec2<T>
	where T: Copy + Div<Output=T>
{
	fn div_assign(&mut self, scalar: T) {
		*self = *self / scalar;
	}
}

impl<T> Div<Vec2<T>> for Vec2<T>
	where T: Div<Output=T>
{
	type Output = Vec2<T>;
	
	fn div(self, other: Vec2<T>) -> Vec2<T> {
		vec2(self.x / other.x, self.y / other.y)
	}
}

impl<T> DivAssign<Vec2<T>> for Vec2<T>
	where T: Copy + Div<Output=T>
{
	fn div_assign(&mut self, other: Vec2<T>) {
		*self = *self / other;
	}
}

impl<T> Mul<T> for Vec2<T>
	where T: Copy + Mul<Output=T>
{
	type Output = Vec2<T>;
	
	fn mul(self, scalar: T) -> Vec2<T> {
		vec2(self.x * scalar, self.y * scalar)
	}
}

impl<T> MulAssign<T> for Vec2<T>
	where T: Copy + Mul<Output=T>
{
	fn mul_assign(&mut self, scalar: T) {
		*self = *self * scalar;
	}
}

impl<T> Mul<Vec2<T>> for Vec2<T>
	where T: Mul<Output=T>
{
	type Output = Vec2<T>;
	
	fn mul(self, other: Vec2<T>) -> Vec2<T> {
		vec2(self.x * other.x, self.y * other.y)
	}
}

impl<T> MulAssign<Vec2<T>> for Vec2<T>
	where T: Copy + Mul<Output=T>
{
	fn mul_assign(&mut self, other: Vec2<T>) {
		*self = *self * other;
	}
}

impl<T> Add<T> for Vec2<T>
	where T: Copy + Add<Output=T>
{
	type Output = Vec2<T>;
	
	fn add(self, scalar: T) -> Vec2<T> {
		vec2(self.x + scalar, self.y + scalar)
	}
}

impl<T> AddAssign<T> for Vec2<T>
	where T: Copy + Add<Output=T>
{
	fn add_assign(&mut self, scalar: T) {
		*self = *self + scalar;
	}
}

impl<T> Add<Vec2<T>> for Vec2<T>
	where T: Add<Output=T>
{
	type Output = Vec2<T>;
	
	fn add(self, other: Vec2<T>) -> Vec2<T> {
		vec2(self.x + other.x, self.y + other.y)
	}
}

impl<T> AddAssign<Vec2<T>> for Vec2<T>
	where T: Copy + Add<Output=T>
{
	fn add_assign(&mut self, other: Vec2<T>) {
		*self = *self + other;
	}
}

impl<T> Sub<T> for Vec2<T>
	where T: Copy + Sub<Output=T>
{
	type Output = Vec2<T>;
	
	fn sub(self, scalar: T) -> Vec2<T> {
		vec2(self.x - scalar, self.y - scalar)
	}
}

impl<T> SubAssign<T> for Vec2<T>
	where T: Copy + Sub<Output=T>
{
	fn sub_assign(&mut self, scalar: T) {
		*self = *self - scalar;
	}
}

impl<T> Sub<Vec2<T>> for Vec2<T>
	where T: Sub<Output=T>
{
	type Output = Vec2<T>;
	
	fn sub(self, other: Vec2<T>) -> Vec2<T> {
		vec2(self.x - other.x, self.y - other.y)
	}
}

impl<T> SubAssign<Vec2<T>> for Vec2<T>
	where T: Copy + Sub<Output=T>
{
	fn sub_assign(&mut self, other: Vec2<T>) {
		*self = *self - other;
	}
}

/*impl<T, U> Into<Vec2<U>> for Vec2<T>
	where T: Into<U>
{
	fn from(v: Vec2<T>) -> Vec2<U> {
		vec2(v.x.into(), v.y.into(), v.z.into())
	}
}*/

impl<T> Default for Vec2<T>
	where T: Default
{
	fn default() -> Self {
		vec2(T::default(), T::default())
	}
}

impl<T> Index<usize> for Vec2<T> {
	type Output = T;
	
	fn index(&self, index: usize) -> &T {
		match index {
			0 => &self.x,
			1 => &self.y,
			_ => panic!("index out of bounds, index is {} but the len is 3",index),
		}
	}
}


impl<T> IndexMut<usize> for Vec2<T> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		match index {
			0 => &mut self.x,
			1 => &mut self.y,
			_ => panic!("index out of bounds, index is {} but the len is 2",index),
		}
	}
}

use std::ops::Neg;
impl<T: Neg> Neg for Vec2<T> {
	type Output = Vec2<<T as Neg>::Output>;
	fn neg(self) -> Vec2<<T as Neg>::Output> { vec2(-self.x,-self.y) }
}
