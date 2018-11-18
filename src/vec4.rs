use prelude::*;

use vec3::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Vec4<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}

impl<T> Vec4<T> {
	pub fn magnitude(self) -> T
		where T: Copy + Sqrt<T> + Mul<Output=T> + Add<Output=T>
	{
		(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
	}
	
	pub fn normalise(self) -> Self
		where T: Copy + Sqrt<T> + Div<Output=T> + Mul<Output=T> + Add<Output=T>
	{
		vec4(self.x / self.magnitude(), self.y / self.magnitude(), self.z / self.magnitude(), self.w / self.magnitude())
	}
	
	pub fn convert<U>(self) -> Vec4<U>
		where T: Into<U>
	{
		vec4(self.x.into(), self.y.into(), self.z.into(), self.w.into())
	}
	
	pub fn from_vec3(v: Vec3<T>) -> Self
		where T: One,
	{
		Self { x: v.x, y: v.y, z: v.z, w: T::one() }
	}
	
	pub fn max(self, other: Self) -> Self
		where T: PartialOrd
	{
		vec4(
			if self.x < other.x { other.x } else { self.x },
			if self.y < other.y { other.y } else { self.y },
			if self.z < other.z { other.z } else { self.z },
			if self.w < other.w { other.w } else { self.w },
		)
	}
	
	pub fn min(self, other: Self) -> Self
		where T: PartialOrd
	{
		vec4(
			if self.x > other.x { other.x } else { self.x },
			if self.y > other.y { other.y } else { self.y },
			if self.z > other.z { other.z } else { self.z },
			if self.w > other.w { other.w } else { self.w },
		)
	}
	
	pub fn max_elem(self) -> T
		where T: PartialOrd
	{
		let Vec4{x,y,z,w} = self;
		let (a,b) = (if x > y { x } else { y }, if z > w { z } else { w });
		if a > b { a } else { b }
	}
	
	pub fn min_elem(self) -> T
		where T: PartialOrd
	{
		let Vec4{x,y,z,w} = self;
		let (a,b) = (if x < y { x } else { y }, if z < w { z } else { w });
		if a < b { a } else { b }
	}
	
	
	pub fn zero() -> Self
		where T: Default
	{
		vec4(T::default(), T::default(), T::default(), T::default())
	}
}

pub fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T>
{
	Vec4 { x: x, y: y, z: z, w: w }
}

impl Vec4<bool> {
	pub fn and(self) -> bool {
		self.x && self.y && self.z && self.w
	}
	
	pub fn or(self) -> bool {
		self.x || self.y || self.z || self.w
	}
}

macro_rules! impl_floats1 {
	($($U: ident),+) => {$(
		impl Vec4<f64> {
			pub fn $U(self) -> Self {
				vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
			}
		}
		impl Vec4<f32> {
			pub fn $U(self) -> Self {
				vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
			}
		}
	)+}
}

macro_rules! impl_floats2 {
	($($U: ident),+) => {$(
		impl Vec4<f64> {
			pub fn $U(self) -> Vec4<bool> {
				vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
			}
		}
		impl Vec4<f32> {
			pub fn $U(self) -> Vec4<bool> {
				vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
			}
		}
	)+}
}

//component-wise functions
//certain conversion and trig functions not implemented to avoid confusion
impl_floats1!(floor,ceil,round,trunc,fract,abs,signum,sqrt,exp,exp2,ln,log2,log10,cbrt,exp_m1,ln_1p);
impl_floats2!(is_nan,is_infinite,is_finite,is_normal,is_sign_positive,is_sign_negative);

pub fn dotvec4<T>(v: Vec4<T>, u: Vec4<T>) -> T
	where T: Copy + Mul<Output=T> + Add<Output=T>
{
	v.x * u.x + v.y * u.y + v.z * u.z + v.w * u.w
}

impl Vec4<u16> {
	pub fn into_workaround(self) -> Vec4<f64> {
		vec4(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
	}
}

impl Vec4<f64> {
	pub fn into_workaround(self) -> Vec4<u16> {
		vec4(self.x as u16, self.y as u16, self.z as u16, self.w as u16)
	}
}

impl<T> Div<T> for Vec4<T>
	where T: Copy + Div<Output=T>
{
	type Output = Vec4<T>;
	
	fn div(self, scalar: T) -> Vec4<T> {
		vec4(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
	}
}

impl<T> DivAssign<T> for Vec4<T>
	where T: Copy + Div<Output=T>
{
	fn div_assign(&mut self, scalar: T) {
		*self = *self / scalar;
	}
}

impl<T> Div<Vec4<T>> for Vec4<T>
	where T: Div<Output=T>
{
	type Output = Vec4<T>;
	
	fn div(self, other: Vec4<T>) -> Vec4<T> {
		vec4(self.x / other.x, self.y / other.y, self.z / other.z, self.w / other.w)
	}
}

impl<T> DivAssign<Vec4<T>> for Vec4<T>
	where T: Copy + Div<Output=T>
{
	fn div_assign(&mut self, other: Vec4<T>) {
		*self = *self / other;
	}
}

impl<T> Mul<T> for Vec4<T>
	where T: Copy + Mul<Output=T>
{
	type Output = Vec4<T>;
	
	fn mul(self, scalar: T) -> Vec4<T> {
		vec4(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
	}
}

impl<T> MulAssign<T> for Vec4<T>
	where T: Copy + Mul<Output=T>
{
	fn mul_assign(&mut self, scalar: T) {
		*self = *self * scalar;
	}
}

impl<T> Mul<Vec4<T>> for Vec4<T>
	where T: Mul<Output=T>
{
	type Output = Vec4<T>;
	
	fn mul(self, other: Vec4<T>) -> Vec4<T> {
		vec4(self.x * other.x, self.y * other.y, self.z * other.z, self.w * other.w)
	}
}

impl<T> MulAssign<Vec4<T>> for Vec4<T>
	where T: Copy + Mul<Output=T>
{
	fn mul_assign(&mut self, other: Vec4<T>) {
		*self = *self * other;
	}
}

impl<T> Add<T> for Vec4<T>
	where T: Copy + Add<Output=T>
{
	type Output = Vec4<T>;
	
	fn add(self, scalar: T) -> Vec4<T> {
		vec4(self.x + scalar, self.y + scalar, self.z + scalar, self.w + scalar)
	}
}

impl<T> AddAssign<T> for Vec4<T>
	where T: Copy + Add<Output=T>
{
	fn add_assign(&mut self, scalar: T) {
		*self = *self + scalar;
	}
}

impl<T> Add<Vec4<T>> for Vec4<T>
	where T: Add<Output=T>
{
	type Output = Vec4<T>;
	
	fn add(self, other: Vec4<T>) -> Vec4<T> {
		vec4(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
	}
}

impl<T> AddAssign<Vec4<T>> for Vec4<T>
	where T: Copy + Add<Output=T>
{
	fn add_assign(&mut self, other: Vec4<T>) {
		*self = *self + other;
	}
}

impl<T> Sub<T> for Vec4<T>
	where T: Copy + Sub<Output=T>
{
	type Output = Vec4<T>;
	
	fn sub(self, scalar: T) -> Vec4<T> {
		vec4(self.x - scalar, self.y - scalar, self.z - scalar, self.w - scalar)
	}
}

impl<T> SubAssign<T> for Vec4<T>
	where T: Copy + Sub<Output=T>
{
	fn sub_assign(&mut self, scalar: T) {
		*self = *self - scalar;
	}
}

impl<T> Sub<Vec4<T>> for Vec4<T>
	where T: Sub<Output=T>
{
	type Output = Vec4<T>;
	
	fn sub(self, other: Vec4<T>) -> Vec4<T> {
		vec4(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
	}
}

impl<T> SubAssign<Vec4<T>> for Vec4<T>
	where T: Copy + Sub<Output=T>
{
	fn sub_assign(&mut self, other: Vec4<T>) {
		*self = *self - other;
	}
}

/*impl<T, U> Into<Vec4<U>> for Vec4<T>
	where T: Into<U>
{
	fn from(v: Vec4<T>) -> Vec4<U> {
		vec4(v.x.into(), v.y.into(), v.z.into())
	}
}*/

impl<T> Default for Vec4<T>
	where T: Default
{
	fn default() -> Self {
		vec4(T::default(), T::default(), T::default(), T::default())
	}
}

impl<T> Index<usize> for Vec4<T> {
	type Output = T;
	
	fn index(&self, index: usize) -> &T {
		match index {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			3 => &self.w,
			_ => panic!("index out of bounds, index is {} but the len is 4",index),
		}
	}
}


impl<T> IndexMut<usize> for Vec4<T> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		match index {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			3 => &mut self.w,
			_ => panic!("index out of bounds, index is {} but the len is 4",index),
		}
	}
}

impl<T: Neg> Neg for Vec4<T> {
	type Output = Vec4<<T as Neg>::Output>;
	fn neg(self) -> Vec4<<T as Neg>::Output> { vec4(-self.x,-self.y,-self.z,-self.w) }
}

impl<T> ArrayTuple for Vec4<T> {
	type Array = [T; 4];
	type Tuple = (T, T, T, T);
	fn into_array(self) -> [T; 4] {	let Vec4{x,y,z,w} = self; [x,y,z,w] }
	fn into_tuple(self) -> (T, T, T, T) { self.into_array().into_tuple() }
}

impl<T> From<(T, T, T, T)> for Vec4<T> {
	fn from(t: (T, T, T, T)) -> Self {
		let (a,b,c,d) = t;
		vec4(a,b,c,d)
	}
}
impl<T> From<[T; 4]> for Vec4<T> {
	fn from(t: [T; 4]) -> Self {
		t.into_tuple().into()
	}
}

impl<T: fmt::Display> fmt::Display for Vec4<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(p) = f.precision() {
			write!(f, "({4:.*}, {5:.*}, {6:.*}, {7:.*})", p, p, p, p, self.x, self.y, self.z, self.w)
		} else {
			write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
		}
	}
}

macro_rules! convert {
    ($T: ty, $($U: ident),+) => {$(
        impl Vec4<$T> {
            pub fn $U(self) -> Vec4<$U> {
                vec4(self.x as $U, self.y as $U, self.z as $U, self.w as $U)
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
