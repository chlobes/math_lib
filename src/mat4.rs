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
		where Vec4<T>: Dot<T> + Copy {
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
}

pub fn mat4<T>(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Mat4<T> {
	Mat4 { x, y, z, w, }
}

impl<T> Default for Mat4<T>
	where T: Zero + One {
	fn default() -> Self {
		Mat4::ident()
	}
}

impl<T> Add<Self> for Mat4<T>
	where T: Add<Output=T> {
	type Output = Self;
	
	fn add(self, other: Self) -> Self {
		mat4(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
	}
}

impl<T> AddAssign<Self> for Mat4<T>
	where T: Copy + Add<Output=T> {
	fn add_assign(&mut self, other: Self) {
		*self = *self + other;
	}
}

impl<T> Sub<Self> for Mat4<T>
	where T: Sub<Output=T> {
	type Output = Self;
	
	fn sub(self, other: Self) -> Self {
		mat4(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
	}
}

impl<T> SubAssign<Self> for Mat4<T>
	where T: Copy + Sub<Output=T> {
	fn sub_assign(&mut self, other: Self) {
		*self = *self - other;
	}
}

impl<T> Mul<Mat4<T>> for Mat4<T>
	where Vec4<T>: Dot<T> + Copy {
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
	where Vec4<T>: Dot<T> + Copy {
	type Output = Vec4<T>;
	
	fn mul(self, v: Vec4<T>) -> Vec4<T> {
		self.apply_to(v)
	}
}

impl<T: Neg> Neg for Mat4<T> {
	type Output = Mat4<<T as Neg>::Output>;
	fn neg(self) -> Mat4<<T as Neg>::Output> { mat4(-self.x,-self.y,-self.z,-self.w) }
}

type CT<T> = (T,T,T,T); //clippy complaints
impl<T> ArrayTuple for Mat4<T> {
	type Array = [[T; 4]; 4];
	type Tuple = (CT<T>,CT<T>,CT<T>,CT<T>);
	fn into_array(self) -> Self::Array {
		let Mat4{x,y,z,w} = self;
		[x.into_array(),y.into_array(),z.into_array(),w.into_array()]
	}
	fn into_tuple(self) -> Self::Tuple {
		let Mat4{x,y,z,w} = self;
		(x.into_tuple(),y.into_tuple(),z.into_tuple(),w.into_tuple())
	}
}

impl Into<Matrix> for Mat4<f32> {
	fn into(self) -> Matrix {
		let a = self.transpose(); //since we use row major while they use column major
		Matrix {
			m0: a.x.x, m1: a.x.y, m2: a.x.z, m3: a.x.w,
			m4: a.y.x, m5: a.y.y, m6: a.y.z, m7: a.y.w,
			m8: a.z.x, m9: a.z.y, m10: a.z.z,m11: a.z.w,
			m12: a.w.x,m13: a.w.y,m14: a.w.z,m15: a.w.w,
		}
	}
}

macro convert($T: ty, $($U: ident),*) {
	$(
		impl Mat4<$T> {
			pub fn $U(self) -> Mat4<$U> {
				mat4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
			}
		}
	)*
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
