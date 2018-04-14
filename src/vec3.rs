use std::ops::{Index, IndexMut, Div, DivAssign, Mul, MulAssign, Add, AddAssign, Sub, SubAssign};
use std::marker::Copy;
use std::convert::Into;

use traits::Sqrt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Vec3<T> {
	pub fn magnitude(self) -> T
		where T: Copy + Sqrt<T> + Mul<Output=T> + Add<Output=T>
	{
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
	
	pub fn normalize(self) -> Self
		where T: Copy + Sqrt<T> + Div<Output=T> + Mul<Output=T> + Add<Output=T>
	{
		vec3(self.x / self.magnitude(), self.y / self.magnitude(), self.z / self.magnitude())
	}
	
	pub fn into_workaround<U>(self) -> Vec3<U>
		where T: Into<U>
	{
		vec3(self.x.into(), self.y.into(), self.z.into())
	}
	
	pub fn zero() -> Self
		where T: Default
	{
		Self::default()
	}
	
	pub fn max(self, other: Self) -> Self
		where T: PartialOrd
	{
		vec3(
			if self.x < other.x { other.x } else { self.x },
			if self.y < other.y { other.y } else { self.y },
			if self.z < other.z { other.z } else { self.z },
		)
	}
	
	pub fn min(self, other: Self) -> Self
		where T: PartialOrd
	{
		vec3(
			if self.x > other.x { other.x } else { self.x },
			if self.y > other.y { other.y } else { self.y },
			if self.z > other.z { other.z } else { self.z },
		)
	}
}

impl Vec3<f64> {
	pub fn convert(self) -> Vec3<i16> {
		vec3(self.x as i16, self.y as i16, self.z as i16)
	}
}

impl Vec3<i16> {
	pub fn convert(self) -> Vec3<f64> {
		vec3(self.x as f64, self.y as f64, self.z as f64)
	}
}

impl Vec3<f64> {
	pub fn ceil(self) -> Self {
		vec3(self.x.ceil(), self.y.ceil(), self.z.ceil())
	}
}

impl Vec3<f32> {
	pub fn ceil(self) -> Self {
		vec3(self.x.ceil(), self.y.ceil(), self.z.ceil())
	}
}

pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T>
{
	Vec3 { x: x, y: y, z: z }
}

pub fn dot<T>(v: Vec3<T>, u: Vec3<T>) -> T
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	v.x * u.x + v.y * u.y + v.z * u.z
}

pub fn cross<T>(v: Vec3<T>, u: Vec3<T>) -> Vec3<T>
	where T: Copy + Mul<Output=T> + Sub<Output=T>
{
	vec3(v.y * u.z - v.z * u.y, v.z * u.x - v.x * u.z, v.x * u.y - v.y * u.x)
}

pub fn distance<T>(v: Vec3<T>, u: Vec3<T>) -> T
	where T: Copy + Sqrt<T> + Mul<Output=T> + Add<Output=T> + Sub<Output=T>
{
	(v - u).magnitude()
}

impl<T> Div<T> for Vec3<T>
	where T: Copy + Div<Output=T>
{
	type Output = Vec3<T>;
	
	fn div(self, scalar: T) -> Vec3<T> {
		vec3(self.x / scalar, self.y / scalar, self.z / scalar)
	}
}

impl<T> DivAssign<T> for Vec3<T>
	where T: Copy + Div<Output=T>
{
	fn div_assign(&mut self, scalar: T) {
		*self = *self / scalar;
	}
}

impl<T> Div<Vec3<T>> for Vec3<T>
	where T: Div<Output=T>
{
	type Output = Vec3<T>;
	
	fn div(self, other: Vec3<T>) -> Vec3<T> {
		vec3(self.x / other.x, self.y / other.y, self.z / other.z)
	}
}

impl<T> DivAssign<Vec3<T>> for Vec3<T>
	where T: Copy + Div<Output=T>
{
	fn div_assign(&mut self, other: Vec3<T>) {
		*self = *self / other;
	}
}

impl<T> Mul<T> for Vec3<T>
	where T: Copy + Mul<Output=T>
{
	type Output = Vec3<T>;
	
	fn mul(self, scalar: T) -> Vec3<T> {
		vec3(self.x * scalar, self.y * scalar, self.z * scalar)
	}
}

impl<T> MulAssign<T> for Vec3<T>
	where T: Copy + Mul<Output=T>
{
	fn mul_assign(&mut self, scalar: T) {
		*self = *self * scalar;
	}
}

impl<T> Mul<Vec3<T>> for Vec3<T>
	where T: Mul<Output=T>
{
	type Output = Vec3<T>;
	
	fn mul(self, other: Vec3<T>) -> Vec3<T> {
		vec3(self.x * other.x, self.y * other.y, self.z * other.z)
	}
}

impl<T> MulAssign<Vec3<T>> for Vec3<T>
	where T: Copy + Mul<Output=T>
{
	fn mul_assign(&mut self, other: Vec3<T>) {
		*self = *self * other;
	}
}

impl<T> Add<T> for Vec3<T>
	where T: Copy + Add<Output=T>
{
	type Output = Vec3<T>;
	
	fn add(self, scalar: T) -> Vec3<T> {
		vec3(self.x + scalar, self.y + scalar, self.z + scalar)
	}
}

impl<T> AddAssign<T> for Vec3<T>
	where T: Copy + Add<Output=T>
{
	fn add_assign(&mut self, scalar: T) {
		*self = *self + scalar;
	}
}

impl<T> Add<Vec3<T>> for Vec3<T>
	where T: Add<Output=T>
{
	type Output = Vec3<T>;
	
	fn add(self, other: Vec3<T>) -> Vec3<T> {
		vec3(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl<T> AddAssign<Vec3<T>> for Vec3<T>
	where T: Copy + Add<Output=T>
{
	fn add_assign(&mut self, other: Vec3<T>) {
		*self = *self + other;
	}
}

impl<T> Sub<T> for Vec3<T>
	where T: Copy + Sub<Output=T>
{
	type Output = Vec3<T>;
	
	fn sub(self, scalar: T) -> Vec3<T> {
		vec3(self.x - scalar, self.y - scalar, self.z - scalar)
	}
}

impl<T> SubAssign<T> for Vec3<T>
	where T: Copy + Sub<Output=T>
{
	fn sub_assign(&mut self, scalar: T) {
		*self = *self - scalar;
	}
}

impl<T> Sub<Vec3<T>> for Vec3<T>
	where T: Sub<Output=T>
{
	type Output = Vec3<T>;
	
	fn sub(self, other: Vec3<T>) -> Vec3<T> {
		vec3(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl<T> SubAssign<Vec3<T>> for Vec3<T>
	where T: Copy + Sub<Output=T>
{
	fn sub_assign(&mut self, other: Vec3<T>) {
		*self = *self - other;
	}
}

/*impl<T, U> Into<Vec3<U>> for Vec3<T>
	where T: Into<U>
{
	fn from(v: Vec3<T>) -> Vec3<U> {
		vec3(v.x.into(), v.y.into(), v.z.into())
	}
}*/

impl<T> Default for Vec3<T>
	where T: Default
{
	fn default() -> Self {
		vec3(T::default(), T::default(), T::default())
	}
}

impl<T> Index<usize> for Vec3<T> {
	type Output = T;
	
	fn index(&self, index: usize) -> &T {
		match index {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			_ => panic!("index out of bounds, index is {} but the len is 3",index),
		}
	}
}


impl<T> IndexMut<usize> for Vec3<T> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		match index {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			_ => panic!("index out of bounds, index is {} but the len is 3",index),
		}
	}
}