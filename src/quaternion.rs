use std::ops::{Div, Mul, Add, Sub, Neg};
use std::convert::Into;

use traits::{Sqrt, Trig};
use traits::numbers::{Half, One, Two};
use mat3::*;
use vec3::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Quaternion<T> {
	pub r: T,
	pub i: T,
	pub j: T,
	pub k: T,
}

impl<T> Quaternion<T>
{
	pub fn normalise(self) -> Self
		where T: Copy + Sqrt<T> + Div<Output=T> + Mul<Output=T> + Add<Output=T>
	{
		let magnitude = (self.i * self.i + self.j * self.j + self.k * self.k + self.r * self.r).sqrt();
		Self { r: self.r / magnitude, i: self.i / magnitude, j: self.j / magnitude, k: self.k / magnitude }
	}
	
	pub fn inverse(self) -> Self
		where T: Neg<Output=T>
	{
		Self { r: -self.r, i: self.i, j: self.j, k: self.k }
	}
	
	pub fn from_axis_angle(v: Vec3<T>) -> Self
		where T: Copy + Sqrt<T> + Trig + Half + Add<Output=T> + Mul<Output=T>
	{
		let magnitude = v.magnitude();
		let (factor, r) = magnitude.half().sin_cos();
		let (i, j, k) = (factor * v.x, factor * v.y, factor * v.z);
		Self { r: r, i: i, j: j, k: k }
	}
	
	pub fn into<U>(self) -> Quaternion<U>
		where T: Into<U>
	{
		Quaternion { r: self.r.into(), i: self.i.into(), j: self.j.into(), k: self.k.into() }
	}
	
	pub fn rot_mat(self) -> Mat3<T>
		where T: Copy + Two + One + Mul<Output=T> + Add<Output=T> + Sub<Output=T>
	{
		let (r, i, j, k) = (self.r, self.i, self.j, self.k);
		mat3(
			vec3(T::one() - T::two() * (j * j + k * k), T::two() * (i * j - r * k), T::two() * (i * k + r * j)),
			vec3(T::two() * (i * j + r * k), T::one() - T::two() * (i * i + k * k), T::two() * (j * k - r * i)),
			vec3(T::two() * (i * k - r * j), T::two() * (j * k + r * i), T::one() - T::two() * (i * i + j * j)),
		)
	}
	
	pub fn convert<U>(self) -> Quaternion<U>
		where T: Into<U>
	{
		Quaternion { r: self.r.into(), i: self.i.into(), j: self.j.into(), k: self.k.into() }
	}
}

impl<T> Mul<Quaternion<T>> for Quaternion<T>
	where T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Copy
{
	type Output = Self;
	
	fn mul(self, other: Self) -> Self {
		Quaternion {
			r: self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k,
			i: self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j,
			j: self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i,
			k: self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r,
		}
	}
}

impl Default for Quaternion<f32> {
	fn default() -> Self {
		Self { r: 1.0, i: 0.0, j: 0.0, k: 0.0 }
	}
}

impl Default for Quaternion<f64> {
	fn default() -> Self {
		Self { r: 1.0, i: 0.0, j: 0.0, k: 0.0 }
	}
}
