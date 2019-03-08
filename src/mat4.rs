use crate::prelude::*;

use crate::vec4::*;

#[repr(C)]
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd,Eq,Ord,Hash,Serialize,Deserialize)]
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
	
	pub fn inv(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T>
	{
		unimplemented!()
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
			dot(self.x, v),
			dot(self.y, v),
			dot(self.z, v),
			dot(self.w, v),
		)
	}
	
	pub fn transpose(self) -> Self {
		mat4(
			vec4(self.x.x, self.y.x, self.z.x, self.w.x),
			vec4(self.x.y, self.y.y, self.z.y, self.w.y),
			vec4(self.x.z, self.y.z, self.z.z, self.w.z),
			vec4(self.x.w, self.y.w, self.z.w, self.w.w),
		)
	}
	
	pub fn into_array(self) -> [[T; 4]; 4] {	let Mat4{x,y,z,w} = self; [x.into_array(),y.into_array(),z.into_array(),w.into_array()] }
	pub fn into_tuple(self) -> ((T,T,T,T),(T,T,T,T),(T,T,T,T),(T,T,T,T)) { let Mat4{x,y,z,w} = self; (x.into_tuple(),y.into_tuple(),z.into_tuple(),w.into_tuple()) }
}

pub fn mat4<T>(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Mat4<T> {
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

impl<T> Mul<Mat4<T>> for Mat4<T>
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	type Output = Self;
	
	fn mul(self, other: Self) -> Self {
		let s = self;
		let t = other.transpose();
		mat4(
			vec4(dot(s.x,t.x), dot(s.x,t.y), dot(s.x,t.z), dot(s.x,t.w)),
			vec4(dot(s.y,t.x), dot(s.y,t.y), dot(s.y,t.z), dot(s.y,t.w)),
			vec4(dot(s.z,t.x), dot(s.z,t.y), dot(s.z,t.z), dot(s.z,t.w)),
			vec4(dot(s.w,t.x), dot(s.w,t.y), dot(s.w,t.z), dot(s.w,t.w)),
		)
	}
}

impl<T> Mul<Vec4<T>> for Mat4<T>
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	type Output = Vec4<T>;
	
	fn mul(self, v: Vec4<T>) -> Vec4<T> {
		self.apply_to(v)
	}
}

impl<T> ArrayTuple for Mat4<T> {
	type Array = [[T; 4]; 4];
	type Tuple = ((T,T,T,T),(T,T,T,T),(T,T,T,T),(T,T,T,T));
	fn into_array(self) -> Self::Array {	let Mat4{x,y,z,w} = self; [x.into_array(),y.into_array(),z.into_array(),w.into_array()] }
	fn into_tuple(self) -> Self::Tuple { let Mat4{x,y,z,w} = self; (x.into_tuple(),y.into_tuple(),z.into_tuple(),w.into_tuple()) }
}

macro_rules! convert {
	($T: ty, $($U: ident),+) => {$(
		impl Mat4<$T> {
			pub fn $U(self) -> Mat4<$U> {
				mat4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
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
