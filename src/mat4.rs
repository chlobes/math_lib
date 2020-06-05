use crate::prelude::*;

use crate::vec4::*;

#[repr(C)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct Mat4<T> {
	pub x: Vec4<T>,
	pub y: Vec4<T>,
	pub z: Vec4<T>,
	pub w: Vec4<T>,
}

impl<T> Mat4<T> {
	pub fn convert<U>(self) -> Mat4<U>
		where T: Into<U> {
		mat4(self.x.convert(), self.y.convert(), self.z.convert(), self.w.convert())
	}
	
	pub fn det(self) -> T
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> {
		let Mat4{ x,y,z,w } = self;
		  x.x * y.y * z.z * w.w
		+ x.x * y.z * z.w * w.y
		+ x.x * y.w * z.y * w.z
		
		+ x.y * y.x * z.w * w.z
		+ x.y * y.z * z.x * w.w
		+ x.y * y.w * z.z * w.x
		
		+ x.z * y.x * z.y * w.w
		+ x.z * y.y * z.w * w.x
		+ x.z * y.w * z.x * w.y
		
		+ x.w * y.x * z.z * w.y
		+ x.w * y.y * z.x * w.z
		+ x.w * y.z * z.y * w.x
		
		- x.x * y.y * z.w * w.z
		- x.x * y.z * z.y * w.w
		-	x.x * y.w * z.z * w.y
		
		- x.y * y.x * z.z * w.w
		- x.y * y.z * z.w * w.x
		- x.y * y.w * z.x * w.z
		
		- x.z * y.x * z.w * w.y
		- x.z * y.y * z.x * w.w
		- x.z * y.w * z.y * w.x
		
		- x.w * y.x * z.y * w.z
		- x.w * y.y * z.z * w.x
		- x.w * y.z * z.x * w.y
	}
	
	pub fn cofactor(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Neg<Output=T> {
		unimplemented!()
	}
	
	pub fn adjoint(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Neg<Output=T> {
		self.cofactor().transpose()
	}
	
	pub fn inv(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Neg<Output=T> {
		let Mat4{ x,y,z,w } = self;
		mat4(
			vec4(
					y.y * z.z * w.w + y.z * z.w * w.y + y.w * z.y * w.z
				- y.y * z.w * w.z - y.z * z.y * w.w - y.w * z.z * w.y,
				
					x.y * z.w * w.z + x.z * z.y * w.w + x.w * z.z * w.y
				- x.y * z.z * w.w - x.z * z.w * w.y - x.w * z.y * w.z,
				
					x.y * y.z * w.w + x.z * y.w * w.y + x.w * y.y * w.z
				- x.y * y.w * w.z - x.z * y.y * w.w - x.w * y.z * w.y,
				
					x.y * y.w * z.z + x.z * y.y * z.w + x.w * y.z * z.y
				- x.y * y.z * z.w - x.z * y.w * z.y - x.w * y.y * z.z,
			) / self.det(),
			vec4(
					y.x * z.w * w.z + y.z * z.x * w.w + y.w * z.z * w.x
				- y.x * z.z * w.w - y.z * z.w * w.x - y.w * z.x * w.z,
				
					x.x * z.z * w.w + x.z * z.w * w.x + x.w * z.x * w.z
				- x.x * z.w * w.z - x.z * z.x * w.w - x.w * z.z * w.x,
				
					x.x * y.w * w.z + x.z * y.x * w.w + x.w * y.z * w.x
				- x.x * y.z * w.w - x.z * y.w * w.x - x.w * y.x * w.z,
				
					x.x * y.z * z.w + x.z * y.w * z.x + x.w * y.x * z.z
				- x.x * y.w * z.z - x.z * y.x * z.w - x.w * y.z * z.x,
			) / self.det(),
			vec4(
					y.x * z.y * w.w + y.y * z.w * w.x + y.w * z.x * w.y
				- y.x * z.w * w.y - y.y * z.x * w.w - y.w * z.y * w.x,
				
					x.x * z.w * w.y + x.y * z.x * w.w + x.w * z.y * w.x
				- x.x * z.y * w.w - x.y * z.w * w.x - x.w * z.x * w.y,
				
					x.x * y.y * w.w + x.y * y.w * w.x + x.w * y.x * w.y
				- x.x * y.w * w.y - x.y * y.x * w.w - x.w * y.y * w.x,
				
					x.x * y.w * z.y + x.y * y.x * z.w + x.w * y.y * z.x
				- x.x * y.y * z.w - x.y * y.w * z.x - x.w * y.x * z.y,
			) / self.det(),
			vec4(
					y.x * z.z * w.y + y.y * z.x * w.z + y.z * z.y * w.x
				- y.x * z.y * w.z - y.y * z.z * w.x - y.z * z.x * w.y,
				
					x.x * z.y * w.z + x.y * z.z * w.x + x.z * z.x * w.y
				- x.x * z.z * w.y - x.y * z.x * w.z - x.z * z.y * w.x,
				
					x.x * y.z * w.y + x.y * y.x * w.z + x.z * y.y * w.x
				- x.x * y.y * w.z - x.y * y.z * w.x - x.z * y.x * w.y,
				
					x.x * y.y * z.z + x.y * y.z * z.x + x.z * y.x * z.y
				- x.x * y.z * z.y - x.y * y.x * z.z - x.z * y.y * z.x,
			) / self.det(),
		)
	}
	
	pub fn ident() -> Self
		where T: Zero + One {
		mat4(
			vec4(T::one(), T::zero(), T::zero(), T::zero()),
			vec4(T::zero(), T::one(), T::zero(), T::zero()),
			vec4(T::zero(), T::zero(), T::one(), T::zero()),
			vec4(T::zero(), T::zero(), T::zero(), T::one()),
		)
	}
	
	pub fn apply_to(self, v: Vec4<T>) -> Vec4<T>
		where T: Copy + Mul<Output=T> + Add<Output=T> {
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

impl<T> Default for Mat4<T>
	where T: Zero + One {
	fn default() -> Self {
		Mat4::ident()
	}
}

impl<T> Mul<Mat4<T>> for Mat4<T>
	where T: Copy + Mul<Output=T> + Add<Output=T> {
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
	where T: Copy + Mul<Output=T> + Add<Output=T> {
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

macro convert($T: ty, $($U: ident),+) {
	$(
		impl Mat4<$T> {
			pub fn $U(self) -> Mat4<$U> {
				mat4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
			}
		}
	)+
}

convert!(u8,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(u16,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(u32,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(u64,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(usize,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(i8,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(i16,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(i32,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(i64,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(isize,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(f32,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(f64,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(bool,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize);
