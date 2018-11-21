use prelude::*;

use vec4::*;

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
	
	pub fn clamp(self, min: Self, max: Self) -> Self
		where T: PartialOrd
	{
		self.min(max).max(min)
	}
	
	pub fn elem_max(self, m: T) -> Self
		where T: PartialOrd + Copy
	{
		self.max(vec3(m,m,m))
	}
	
	pub fn elem_min(self, m: T) -> Self
		where T: PartialOrd + Copy
	{
		self.min(vec3(m,m,m))
	}
	
	pub fn elem_clamp(self, min: T, max: T) -> Self
		where T: PartialOrd + Copy
	{
		self.min(vec3(max,max,max)).max(vec3(min,min,min))
	}
	
	pub fn max_elem(self) -> T
		where T: PartialOrd
	{
		let Vec3{x,y,z} = self;
		if x > y { if x > z { x } else { z } } else if y > z { y } else { z }
	}
	
	pub fn min_elem(self) -> T
		where T: PartialOrd
	{
		let Vec3{x,y,z} = self;
		if x < y { if x < z { x } else { z } } else if y < z { y } else { z }
	}
	
	pub fn extend(self, w: T) -> Vec4<T> {
		vec4(self.x, self.y, self.z, w)
	}
}

impl Vec3<bool> {
	pub fn and(self) -> bool {
		self.x && self.y && self.z
	}
	
	pub fn or(self) -> bool {
		self.x || self.y || self.z
	}
}

macro_rules! impl_floats1 {
	($($U: ident),+) => {$(
		impl Vec3<f64> {
			pub fn $U(self) -> Self {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<f32> {
			pub fn $U(self) -> Self {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
	)+}
}

macro_rules! impl_floats2 {
	($($U: ident),+) => {$(
		impl Vec3<f64> {
			pub fn $U(self) -> Vec3<bool> {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<f32> {
			pub fn $U(self) -> Vec3<bool> {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
	)+}
}

//component-wise functions
//certain conversion and trig functions not implemented to avoid confusion
impl_floats1!(floor,ceil,round,trunc,fract,abs,signum,sqrt,exp,exp2,ln,log2,log10,cbrt,exp_m1,ln_1p);
impl_floats2!(is_nan,is_infinite,is_finite,is_normal,is_sign_positive,is_sign_negative);

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

impl<T: Neg> Neg for Vec3<T> {
	type Output = Vec3<<T as Neg>::Output>;
	fn neg(self) -> Vec3<<T as Neg>::Output> { vec3(-self.x,-self.y,-self.z) }
}

impl<T> ArrayTuple for Vec3<T> {
	type Array = [T; 3];
	type Tuple = (T, T, T);
	fn into_array(self) -> [T; 3] {	let Vec3{x,y,z} = self; [x,y,z] }
	fn into_tuple(self) -> (T, T, T) { self.into_array().into_tuple() }
}

impl<T> From<(T, T, T)> for Vec3<T> {
	fn from(t: (T, T, T)) -> Self {
		let (a,b,c) = t;
		vec3(a,b,c)
	}
}
impl<T> From<[T; 3]> for Vec3<T> {
	fn from(t: [T; 3]) -> Self {
		t.into_tuple().into()
	}
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(p) = f.precision() {
			write!(f, "({3:.*}, {4:.*}, {5:.*})", p, p, p, self.x, self.y, self.z)
		} else {
			write!(f, "({}, {}, {})", self.x, self.y, self.z)
		}
	}
}

impl<T: fmt::LowerExp> fmt::LowerExp for Vec3<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(p) = f.precision() {
			write!(f, "({3:.*e}, {4:.*e}, {5:.*e})", p, p, p, self.x, self.y, self.z)
		} else {
			write!(f, "({:e}, {:e}, {:e})", self.x, self.y, self.z)
		}
	}
}

macro_rules! convert {
	($T: ty, $($U: ident),+) => {$(
		impl Vec3<$T> {
			pub fn $U(self) -> Vec3<$U> {
				vec3(self.x as $U, self.y as $U, self.z as $U)
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
