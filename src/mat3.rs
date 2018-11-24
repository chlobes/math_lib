use prelude::*;

use vec2::vec2;
use vec3::*;
use mat4::*;
use mat2::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Mat3<T> {
	pub x: Vec3<T>,
	pub y: Vec3<T>,
	pub z: Vec3<T>,
}

impl<T> Mat3<T> {
	pub fn det(self) -> T
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T>
	{
		let Self{ x,y,z } = self;
		  x.x * y.y * z.z
		+ x.y * y.z * z.x
		+ x.z * y.x * z.y
		- x.x * y.z * z.y
		- x.y * y.x * z.z
		- x.z * y.y * z.x
	}
	
	pub fn adj(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T>
	{
		let Mat3{ x,y,z } = self;
		mat3(
			vec3(
				mat2(vec2(y.y,y.z),vec2(z.y,z.z)).det(),
				mat2(vec2(y.x,y.z),vec2(z.x,z.z)).det(),
				mat2(vec2(y.x,y.y),vec2(z.x,z.y)).det()),
			vec3(
				mat2(vec2(x.y,x.z),vec2(z.y,z.z)).det(),
				mat2(vec2(x.x,x.z),vec2(z.x,z.z)).det(),
				mat2(vec2(x.x,x.y),vec2(z.x,z.y)).det()),
			vec3(
				mat2(vec2(x.y,x.z),vec2(y.y,y.z)).det(),
				mat2(vec2(x.x,x.z),vec2(y.x,y.z)).det(),
				mat2(vec2(x.x,x.y),vec2(y.x,y.y)).det()),
		)
	}
	
	pub fn inv(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T>
	{
		let Mat3{ x,y,z } = self.adj();
		mat3(x/self.det(),y/self.det(),z/self.det())
	}
	
	pub fn ident() -> Self
		where T: One + Default
	{
		mat3(
			vec3(T::one(), T::default(), T::default()),
			vec3(T::default(), T::one(), T::default()),
			vec3(T::default(), T::default(), T::one()),
		)
	}
	
	pub fn apply_to(self, v: Vec3<T>) -> Vec3<T>
		where T: Copy + Mul<Output=T> + Add<Output=T>
	{
		vec3(
			dot(self.x, v),
			dot(self.y, v),
			dot(self.z, v),
		)
	}

	pub fn extend(self, right: Vec3<T>, bottom: Vec3<T>, corner: T) -> Mat4<T> {
		mat4(
			self.x.extend(right.x),
			self.y.extend(right.y),
			self.z.extend(right.z),
			bottom.extend(corner),
		)
	}

	pub fn transpose(self) -> Self {
		mat3(
			vec3(self.x.x, self.y.x, self.z.x),
			vec3(self.x.y, self.y.y, self.z.y),
			vec3(self.x.z, self.y.z, self.z.z),
		)
	}
}

impl Mat3<f32> {
	pub fn rotate_x(angle: f32) -> Self {
		mat3(
			vec3(1.0, 0.0, 0.0),
			vec3(0.0, angle.cos(), -angle.sin()),
			vec3(0.0, angle.sin(), angle.cos()),
		)
	}

	pub fn rotate_y(angle: f32) -> Self {
		mat3(
			vec3(angle.cos(), 0.0, angle.sin()),
			vec3(0.0, 1.0, 0.0),
			vec3(-angle.sin(), 0.0, angle.cos()),
		)
	}

	pub fn rotate_z(angle: f32) -> Self {
		mat3(
			vec3(angle.cos(), -angle.sin(), 0.0),
			vec3(angle.sin(), angle.cos(), 0.0),
			vec3(0.0, 0.0, 1.0),
		)
	}
}

impl Mat3<f64> {
	pub fn rotate_x(angle: f64) -> Self {
		mat3(
			vec3(1.0, 0.0, 0.0),
			vec3(0.0, angle.cos(), -angle.sin()),
			vec3(0.0, angle.sin(), angle.cos()),
		)
	}

	pub fn rotate_y(angle: f64) -> Self {
		mat3(
			vec3(angle.cos(), 0.0, angle.sin()),
			vec3(0.0, 1.0, 0.0),
			vec3(-angle.sin(), 0.0, angle.cos()),
		)
	}

	pub fn rotate_z(angle: f64) -> Self {
		mat3(
			vec3(angle.cos(), -angle.sin(), 0.0),
			vec3(angle.sin(), angle.cos(), 0.0),
			vec3(0.0, 0.0, 1.0),
		)
	}
}

pub fn mat3<T>(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
	Mat3 { x: x, y: y, z: z }
}

impl<T> Default for Mat3<T>
	where T: One + Default
{
	fn default() -> Self {
		Mat3::ident()
	}
}

impl<T> Mul<Mat3<T>> for Mat3<T>
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	type Output = Self;
	
	fn mul(self, other: Self) -> Self {
		let t = other.transpose();
		mat3(
			vec3(dot(self.x,t.x), dot(self.x,t.y), dot(self.x,t.z)),
			vec3(dot(self.y,t.x), dot(self.y,t.y), dot(self.y,t.z)),
			vec3(dot(self.z,t.x), dot(self.z,t.y), dot(self.z,t.z)),
		)
	}
}

impl<T> Mul<Vec3<T>> for Mat3<T>
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	type Output = Vec3<T>;
	
	fn mul(self, v: Vec3<T>) -> Vec3<T> {
		self.apply_to(v)
	}
}

impl<T> ArrayTuple for Mat3<T> {
	type Array = [[T; 3]; 3];
	type Tuple = ((T,T,T),(T,T,T),(T,T,T));
	fn into_array(self) -> Self::Array {	let Mat3{x,y,z} = self; [x.into_array(),y.into_array(),z.into_array()] }
	fn into_tuple(self) -> Self::Tuple { let Mat3{x,y,z} = self; (x.into_tuple(),y.into_tuple(),z.into_tuple()) }
}

macro_rules! convert {
	($T: ty, $($U: ident),+) => {$(
		impl Mat3<$T> {
			pub fn $U(self) -> Mat3<$U> {
				mat3(self.x.$U(), self.y.$U(), self.z.$U())
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
