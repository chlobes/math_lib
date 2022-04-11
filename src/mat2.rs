use crate::prelude::*;

use crate::vec2::*;

#[repr(C)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct Mat2<T> {
	pub x: Vec2<T>,
	pub y: Vec2<T>,
}

impl<T> Mat2<T> {
	pub fn det(self) -> T
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> {
		let Self{ x,y } = self;
		x.x * y.y - x.y * y.x
	}
	
	pub fn inv(self) -> Self
	where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Neg<Output=T> {
		let Self{ x,y } = self;
		mat2(
			vec2(y.y, -y.x) / self.det(),
			vec2(-x.y, x.x) / self.det(),
		)
	}
	
	pub fn ident() -> Self
		where T: Zero + One {
		mat2(
			vec2(T::one(), T::zero()),
			vec2(T::zero(), T::one()),
		)
	}
	
	pub fn apply_to(self, v: Vec2<T>) -> Vec2<T>
		where Vec2<T>: Dot<T> + Copy {
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
	
	pub fn rotate(angle: T) -> Self
		where T: Copy + Trig + Neg<Output=T> {
		mat2(
			vec2(angle.cos(), -angle.sin()),
			vec2(angle.sin(), angle.cos()),
		)
	}
}

pub fn mat2<T>(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
	Mat2 { x, y, }
}

impl<T> Default for Mat2<T>
	where T: Zero + One
{
	fn default() -> Self {
		Mat2::ident()
	}
}

impl<T> Add<Self> for Mat2<T>
	where T: Add<Output=T> {
	type Output = Self;
	
	fn add(self, other: Self) -> Self {
		mat2(self.x + other.x, self.y + other.y)
	}
}

impl<T> AddAssign<Self> for Mat2<T>
	where T: Copy + Add<Output=T> {
	fn add_assign(&mut self, other: Self) {
		*self = *self + other;
	}
}

impl<T> Sub<Self> for Mat2<T>
	where T: Sub<Output=T> {
	type Output = Self;
	
	fn sub(self, other: Self) -> Self {
		mat2(self.x - other.x, self.y - other.y)
	}
}

impl<T> SubAssign<Self> for Mat2<T>
	where T: Copy + Sub<Output=T> {
	fn sub_assign(&mut self, other: Self) {
		*self = *self - other;
	}
}

impl<T> Mul<Mat2<T>> for Mat2<T>
	where Vec2<T>: Dot<T> + Copy {
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
	where Vec2<T>: Dot<T> + Copy {
	type Output = Vec2<T>;
	
	fn mul(self, v: Vec2<T>) -> Vec2<T> {
		self.apply_to(v)
	}
}

impl<T: Neg> Neg for Mat2<T> {
	type Output = Mat2<<T as Neg>::Output>;
	fn neg(self) -> Mat2<<T as Neg>::Output> { mat2(-self.x,-self.y) }
}

impl<T> ArrayTuple for Mat2<T> {
	type Array = [[T; 2]; 2];
	type Tuple = ((T,T),(T,T));
	fn into_array(self) -> Self::Array {
		let Self{x,y} = self;
		[x.into_array(),y.into_array()]
	}
	fn into_tuple(self) -> Self::Tuple {
		let Self{x,y} = self;
		(x.into_tuple(),y.into_tuple())
	}
}

macro convert($T: ty, $($U: ident),*) {
	$(
		impl Mat2<$T> {
			pub fn $U(self) -> Mat2<$U> {
				mat2(self.x.$U(), self.y.$U())
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
