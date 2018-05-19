use std::ops::{Mul, Add, Sub};
use std::marker::Copy;
use std::convert::Into;

use traits::numbers::One;
use vec4::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Mat4<T> {
	pub x: Vec4<T>,
	pub y: Vec4<T>,
	pub z: Vec4<T>,
	pub w: Vec4<T>,
}

impl<T> Mat4<T> {
	pub fn convert<U>(self) -> Mat4<U>
		where T: Into<U>
	{
		mat4(self.x.convert(), self.y.convert(), self.z.convert(), self.w.convert())
	}
	
	pub fn det(self) -> T
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T>
	{
			self.x.x * self.y.y * self.z.z * self.w.w
		+ 	self.x.x * self.y.z * self.z.w * self.w.y
		+ 	self.x.x * self.y.w * self.z.y * self.w.z

		+ 	self.x.y * self.y.x * self.z.w * self.w.z
		+ 	self.x.y * self.y.z * self.z.x * self.w.w
		+ 	self.x.y * self.y.w * self.z.z * self.w.x

		+ 	self.x.z * self.y.x * self.z.y * self.w.w
		+ 	self.x.z * self.y.y * self.z.w * self.w.x
		+ 	self.x.z * self.y.w * self.z.x * self.w.y

		+ 	self.x.w * self.y.x * self.z.z * self.w.y
		+ 	self.x.w * self.y.y * self.z.x * self.w.z
		+ 	self.x.w * self.y.z * self.z.y * self.w.x

		- 	self.x.x * self.y.y * self.z.w * self.w.z
		- 	self.x.x * self.y.z * self.z.y * self.w.w
		-	self.x.x * self.y.w * self.z.z * self.w.y

		- 	self.x.y * self.y.x * self.z.z * self.w.w
		- 	self.x.y * self.y.z * self.z.w * self.w.x
		- 	self.x.y * self.y.w * self.z.x * self.w.z

		- 	self.x.z * self.y.x * self.z.w * self.w.y
		- 	self.x.z * self.y.y * self.z.x * self.w.w
		- 	self.x.z * self.y.w * self.z.y * self.w.x

		- 	self.x.w * self.y.x * self.z.y * self.w.z
		- 	self.x.w * self.y.y * self.z.z * self.w.x
		- 	self.x.w * self.y.z * self.z.x * self.w.y
	}
	
	pub fn ident() -> Self
		where T: One + Default
	{
		mat4(
			vec4(T::one(), T::default(), T::default(), T::default()),
			vec4(T::default(), T::one(), T::default(), T::default()),
			vec4(T::default(), T::default(), T::one(), T::default()),
			vec4(T::default(), T::default(), T::default(), T::one()),
		)
	}
	
	pub fn apply_to(self, v: Vec4<T>) -> Vec4<T>
		where T: Copy + Mul<Output=T> + Add<Output=T>
	{
		vec4(
			self.x.x * v.x + self.x.y * v.y + self.x.z * v.z + self.x.w * v.w,
			self.y.x * v.x + self.y.y * v.y + self.y.z * v.z + self.y.w * v.w,
			self.z.x * v.x + self.z.y * v.y + self.z.z * v.z + self.z.w * v.w,
			self.w.x * v.x + self.w.y * v.y + self.w.z * v.z + self.w.w * v.w,
		)
	}
	
	pub fn into_array(self) -> [[T; 4]; 4] {
		[self.x.into_array(), self.y.into_array(), self.z.into_array(), self.w.into_array()]
	}

	pub fn transpose(self) -> Self {
		mat4(
			vec4(self.x.x, self.y.x, self.z.x, self.w.x),
			vec4(self.x.y, self.y.y, self.z.y, self.w.y),
			vec4(self.x.z, self.y.z, self.z.z, self.w.z),
			vec4(self.x.w, self.y.w, self.z.w, self.w.w),
		)
	}
}

pub fn mat4<T>(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Mat4<T>
{
	Mat4 { x: x, y: y, z: z, w: w }
}

/*impl<T, U> Into<Vec3<U>> for Vec3<T>
	where T: Into<U>
{
	fn from(v: Vec3<T>) -> Vec3<U> {
		vec3(v.x.into(), v.y.into(), v.z.into())
	}
}*/

impl<T> Default for Mat4<T>
	where T: One + Default
{
	fn default() -> Self {
		Mat4::ident()
	}
}
