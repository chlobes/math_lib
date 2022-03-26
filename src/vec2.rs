use crate::prelude::*;

use crate::vec3::*;

#[repr(C)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2<T> {
	pub fn magnitude(self) -> T
		where T: Copy + Sqrt<T> + Mul<Output=T> + Add<Output=T> {
		(self * self).sum_elem().sqrt()
	}
	
	pub fn normalize(self) -> Self
		where T: Copy + Sqrt<T> + Div<Output=T> + Mul<Output=T> + Add<Output=T> {
		self / self.magnitude()
	}
	
	pub fn normalize_or_zero(self) -> Self
		where T: Copy + Sqrt<T> + Div<Output=T> + Mul<Output=T> + Add<Output=T> + IsNan + Zero {
		let r = self.normalize();
		if r.is_nan().or() { Self::zero() } else { r }
	}
	
	pub fn zero() -> Self
		where T: Zero {
		vec2(T::zero(), T::zero())
	}
	
	pub fn one() -> Self
		where T: One {
		vec2(T::one(), T::one())
	}
	
	pub fn max(self, other: Self) -> Self
		where T: IsNan {
		vec2(
			self.x.non_nan_max(other.x),
			self.y.non_nan_max(other.y),
		)
	}
	
	pub fn min(self, other: Self) -> Self
		where T: IsNan {
		vec2(
			self.x.non_nan_min(other.x),
			self.y.non_nan_min(other.y),
		)
	}
	
	pub fn max_elem(self) -> T
		where T: IsNan {
		self.x.non_nan_max(self.y)
	}
	
	pub fn min_elem(self) -> T
		where T: IsNan {
		self.x.non_nan_min(self.y)
	}
	
	pub fn clamp(self, min: Self, max: Self) -> Self
		where T: PartialOrd {
		self.min(max).max(min)
	}
	
	pub fn elem_max(self, m: T) -> Self
		where T: PartialOrd + Copy {
		self.max(vec2(m,m))
	}
	
	pub fn elem_min(self, m: T) -> Self
		where T: PartialOrd + Copy {
		self.min(vec2(m,m,))
	}
	
	pub fn elem_clamp(self, min: T, max: T) -> Self
		where T: PartialOrd + Copy {
		self.min(vec2(max,max)).max(vec2(min,min))
	}
	
	pub fn sum_elem(self) -> T
		where T: Add<Output=T> {
		let Vec2{x,y} = self;
		x+y
	}
	
	pub fn mul_elem(self) -> T
		where T: Mul<Output=T> {
		let Vec2{x,y} = self;
		x*y
	}
	
	pub fn extend(self, z: T) -> Vec3<T> {
		vec3(self.x, self.y, z)
	}
	
	pub fn is_nan(&self) -> Vec2<bool>
		where T: IsNan {
		vec2(self.x.is_nan(), self.y.is_nan())
	}
}

pub fn angle_between<T>(a: Vec2<T>, b: Vec2<T>) -> T
	where T: Copy + Trig + Sqrt<T> + Mul<Output=T> + Div<Output=T> + Add<Output=T> + Sub<Output=T> {
	let (a, b) = (a.normalize(), b.normalize());
	dot(a, b).acos()
}

pub use crate::prelude::{dot,distance,distance_squared};
impl<T> Vector<T> for Vec2<T>
	where T: Copy + Sqrt<T> + Mul<Output=T> + Add<Output=T> + Sub<Output=T> {
	fn dot(&self, other: &Self) -> T {
		(*self * *other).sum_elem()
	}
	fn distance(&self, other: &Self) -> T {
		(*self - *other).magnitude()
	}
}

impl<T: Mul<Output=T> + One> Product<Vec2<T>> for Vec2<T> {
	fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
		iter.fold(Self::one(), |a, b| a * b)
	}
}

impl<T: Add<Output=T> + Zero> Sum<Vec2<T>> for Vec2<T> {
	fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
		iter.fold(Self::zero(), |a, b| a + b)
	}
}

impl Vec2<bool> {
	pub fn and(self) -> bool {
		self.x && self.y
	}
	
	pub fn or(self) -> bool {
		self.x || self.y
	}
}

pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
	Vec2 { x, y, }
}

macro impl_ints1($($U: ident),*) {
	$(
		impl Vec2<isize> {
			pub fn $U(self) -> Self {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<i64> {
			pub fn $U(self) -> Self {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<i32> {
			pub fn $U(self) -> Self {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<i16> {
			pub fn $U(self) -> Self {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<i8> {
			pub fn $U(self) -> Self {
				vec2(self.x.$U(), self.y.$U())
			}
		}
	)*
}

macro impl_ints2($($U: ident),*){
	$(
		impl Vec2<isize> {
			pub fn $U(self) -> Vec2<bool> {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<i64> {
			pub fn $U(self) -> Vec2<bool> {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<i32> {
			pub fn $U(self) -> Vec2<bool> {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<i16> {
			pub fn $U(self) -> Vec2<bool> {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<i8> {
			pub fn $U(self) -> Vec2<bool> {
				vec2(self.x.$U(), self.y.$U())
			}
		}
	)*
}

macro impl_floats1($($U: ident),*) {
	$(
		impl Vec2<f64> {
			pub fn $U(self) -> Self {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<f32> {
			pub fn $U(self) -> Self {
				vec2(self.x.$U(), self.y.$U())
			}
		}
	)*
}

macro impl_floats2($($U: ident),*) {
	$(
		impl Vec2<f64> {
			pub fn $U(self) -> Vec2<bool> {
				vec2(self.x.$U(), self.y.$U())
			}
		}
		impl Vec2<f32> {
			pub fn $U(self) -> Vec2<bool> {
				vec2(self.x.$U(), self.y.$U())
			}
		}
	)*
}

//component-wise functions
//certain conversion and trig functions not implemented to avoid confusion
impl_ints1!(abs,signum,swap_bytes/*,reverse_bits*/,to_be,to_le,wrapping_neg,wrapping_abs);
impl_ints2!(is_positive,is_negative);
impl_floats1!(floor,ceil,round,trunc,fract,abs,signum,sqrt,exp,exp2,ln,log2,log10,cbrt,exp_m1,ln_1p);
impl_floats2!(is_infinite,is_finite,is_normal,is_sign_positive,is_sign_negative);

impl<T> Div<T> for Vec2<T>
	where T: Copy + Div<Output=T> {
	type Output = Vec2<T>;
	
	fn div(self, scalar: T) -> Vec2<T> {
		vec2(self.x / scalar, self.y / scalar)
	}
}

impl<T> DivAssign<T> for Vec2<T>
	where T: Copy + Div<Output=T> {
	fn div_assign(&mut self, scalar: T) {
		*self = *self / scalar;
	}
}

impl<T> Div<Vec2<T>> for Vec2<T>
	where T: Div<Output=T> {
	type Output = Vec2<T>;
	
	fn div(self, other: Vec2<T>) -> Vec2<T> {
		vec2(self.x / other.x, self.y / other.y)
	}
}

impl<T> DivAssign<Vec2<T>> for Vec2<T>
	where T: Copy + Div<Output=T> {
	fn div_assign(&mut self, other: Vec2<T>) {
		*self = *self / other;
	}
}

impl<T> Mul<T> for Vec2<T>
	where T: Copy + Mul<Output=T> {
	type Output = Vec2<T>;
	
	fn mul(self, scalar: T) -> Vec2<T> {
		vec2(self.x * scalar, self.y * scalar)
	}
}

impl<T> MulAssign<T> for Vec2<T>
	where T: Copy + Mul<Output=T> {
	fn mul_assign(&mut self, scalar: T) {
		*self = *self * scalar;
	}
}

impl<T> Mul<Vec2<T>> for Vec2<T>
	where T: Mul<Output=T> {
	type Output = Vec2<T>;
	
	fn mul(self, other: Vec2<T>) -> Vec2<T> {
		vec2(self.x * other.x, self.y * other.y)
	}
}

impl<T> MulAssign<Vec2<T>> for Vec2<T>
	where T: Copy + Mul<Output=T> {
	fn mul_assign(&mut self, other: Vec2<T>) {
		*self = *self * other;
	}
}

impl<T> Add<T> for Vec2<T>
	where T: Copy + Add<Output=T> {
	type Output = Vec2<T>;
	
	fn add(self, scalar: T) -> Vec2<T> {
		vec2(self.x + scalar, self.y + scalar)
	}
}

impl<T> AddAssign<T> for Vec2<T>
	where T: Copy + Add<Output=T> {
	fn add_assign(&mut self, scalar: T) {
		*self = *self + scalar;
	}
}

impl<T> Add<Vec2<T>> for Vec2<T>
	where T: Add<Output=T> {
	type Output = Vec2<T>;
	
	fn add(self, other: Vec2<T>) -> Vec2<T> {
		vec2(self.x + other.x, self.y + other.y)
	}
}

impl<T> AddAssign<Vec2<T>> for Vec2<T>
	where T: Copy + Add<Output=T> {
	fn add_assign(&mut self, other: Vec2<T>) {
		*self = *self + other;
	}
}

impl<T> Sub<T> for Vec2<T>
	where T: Copy + Sub<Output=T> {
	type Output = Vec2<T>;
	
	fn sub(self, scalar: T) -> Vec2<T> {
		vec2(self.x - scalar, self.y - scalar)
	}
}

impl<T> SubAssign<T> for Vec2<T>
	where T: Copy + Sub<Output=T> {
	fn sub_assign(&mut self, scalar: T) {
		*self = *self - scalar;
	}
}

impl<T> Sub<Vec2<T>> for Vec2<T>
	where T: Sub<Output=T> {
	type Output = Vec2<T>;
	
	fn sub(self, other: Vec2<T>) -> Vec2<T> {
		vec2(self.x - other.x, self.y - other.y)
	}
}

impl<T> SubAssign<Vec2<T>> for Vec2<T>
	where T: Copy + Sub<Output=T> {
	fn sub_assign(&mut self, other: Vec2<T>) {
		*self = *self - other;
	}
}

impl<T> Rem<T> for Vec2<T>
	where T: Copy + Rem<Output=T> {
	type Output = Vec2<T>;
	
	fn rem(self, scalar: T) -> Vec2<T> {
		vec2(self.x % scalar, self.y % scalar)
	}
}

impl<T> RemAssign<T> for Vec2<T>
	where T: Copy + Rem<Output=T> {
	fn rem_assign(&mut self, scalar: T) {
		*self = *self % scalar;
	}
}

/*impl<T, U> Into<Vec2<U>> for Vec2<T>
	where T: Into<U> {
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
			_ => panic!("index out of bounds, index is {} but the len is 2",index),
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

impl<T: Neg> Neg for Vec2<T> {
	type Output = Vec2<<T as Neg>::Output>;
	fn neg(self) -> Vec2<<T as Neg>::Output> { vec2(-self.x,-self.y) }
}

impl<T> ArrayTuple for Vec2<T> {
	type Array = [T; 2];
	type Tuple = (T, T);
	fn into_array(self) -> [T; 2] {	let Vec2{x,y} = self; [x,y] }
	fn into_tuple(self) -> (T, T) { self.into_array().into_tuple() }
}

impl<T> From<(T, T)> for Vec2<T> {
	fn from(t: (T, T)) -> Self {
		let (a,b) = t;
		vec2(a,b)
	}
}
impl<T> From<[T; 2]> for Vec2<T> {
	fn from(t: [T; 2]) -> Self {
		t.into_tuple().into()
	}
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(p) = f.precision() {
			write!(f, "({2:.*}, {3:.*})", p, p, self.x, self.y)
		} else {
			write!(f, "({}, {})", self.x, self.y)
		}
	}
}

impl<T: NiceFmt> NiceFmt for Vec2<T> {
	fn nice_fmt(&self, limit: usize, pad: bool) -> String {
		format!("({}, {})", self.x.nice_fmt(limit, pad), self.y.nice_fmt(limit, pad))
	}
}

impl<T: fmt::LowerExp> fmt::LowerExp for Vec2<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(p) = f.precision() {
			write!(f, "({2:.*e}, {3:.*e})", p, p, self.x, self.y)
		} else {
			write!(f, "({:e}, {:e})", self.x, self.y)
		}
	}
}

impl Into<Vector2> for Vec2<f32> {
	fn into(self) -> Vector2 {
		Vector2 { x: self.x, y: self.y }
	}
}
impl Into<Vector2_> for Vec2<f32> {
	fn into(self) -> Vector2_ {
		Vector2_ { x: self.x, y: self.y }
	}
}

macro convert($T: ty, $($U: ident),*) {
	$(
		impl Vec2<$T> {
			pub fn $U(self) -> Vec2<$U> {
				vec2(self.x as $U, self.y as $U)
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

impl<T: FromStr> FromStr for Vec2<T> {
	type Err = <T as FromStr>::Err;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut n: Vec<&str> = s.split(|c: char| c.is_whitespace() || c == ',')
			.map(|s| s.trim_matches(BRACKETS)).filter(|s| !s.is_empty()).collect();
		while n.len() < 2 { n.push(""); }
		let x = n[0].parse()?;
		let y = n[1].parse()?;
		Ok(vec2(x, y))
	}
}
