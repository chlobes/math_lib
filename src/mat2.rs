use crate::prelude::*;
use crate::vec2::*;

//TODO: generalize everything to a trait the same way as vectors?
#[repr(C)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct Mat2<T> {
	pub x: Vec2<T>,
	pub y: Vec2<T>,
}
pub fn mat2<T>(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
	Mat2 { x, y, }
}

//TODO: macro-ify all this like VecN?
impl<T: Copy + Zero + One + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Neg<Output=T>> Mat2<T>
	where Vec2<T>: VecOps<T> {
	pub fn ident() -> Self {
		mat2(
			vec2(T::one(), T::zero()),
			vec2(T::zero(), T::one()),
		)
	}
	
	pub fn det(self) -> T	{
		let Self{ x,y } = self;
		x.x * y.y - x.y * y.x
	}
	
	pub fn inv(self) -> Self {
		let Self{ x,y } = self;
		mat2(
			vec2(y.y, -y.x) / self.det(),
			vec2(-x.y, x.x) / self.det(),
		)
	}
	
	pub fn apply_to(self, v: Vec2<T>) -> Vec2<T> {
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
		where T: Trig {
		mat2(
			vec2(angle.cos(), -angle.sin()),
			vec2(angle.sin(), angle.cos()),
		)
	}
}

impl<T: Copy + Zero + One + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Neg<Output=T>> Default for Mat2<T>
	where Vec2<T>: VecOps<T> {
	fn default() -> Self {
		Mat2::ident()
	}
}

impl<T: Copy + Zero + One + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Neg<Output=T>> Mul<Mat2<T>> for Mat2<T>
	where Vec2<T>: VecOps<T> {
	type Output = Self;
	fn mul(self, other: Self) -> Self {
		let t = other.transpose();
		mat2(
			vec2(dot(self.x,t.x), dot(self.x,t.y)),
			vec2(dot(self.y,t.x), dot(self.y,t.y)),
		)
	}
}

impl<T: Copy + Zero + One + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Neg<Output=T>> Mul<Vec2<T>> for Mat2<T>
	where Vec2<T>: VecOps<T> {
	type Output = Vec2<T>;
	fn mul(self, v: Vec2<T>) -> Vec2<T> {
		self.apply_to(v)
	}
}
//no mul assign to avoid confusion as matrix multiplication is not commutative
impl<T: Copy + Zero + One + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Neg<Output=T>> Add for Mat2<T> {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		mat2(self.x + other.x, self.y + other.y)
	}
}

impl<T: Copy + Zero + One + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Neg<Output=T>> AddAssign for Mat2<T> {
	fn add_assign(&mut self, other: Self) {
		*self = *self + other;
	}
}

impl<T: Copy + Zero + One + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Neg<Output=T>> Sub for Mat2<T>	{
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		mat2(self.x - other.x, self.y - other.y)
	}
}

impl<T: Copy + Zero + One + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Neg<Output=T>> SubAssign for Mat2<T>
	where Vec2<T>: VecOps<T> {
	fn sub_assign(&mut self, other: Self) {
		*self = *self - other;
	}
}

impl<T: Neg> Neg for Mat2<T> {
	type Output = Mat2<<T as Neg>::Output>;
	fn neg(self) -> Self::Output { mat2(-self.x,-self.y) }
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
