use prelude::*;

use mat3::*;
use vec3::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Quaternion<T> {
	pub r: T,
	pub i: T,
	pub j: T,
	pub k: T,
}

impl<T> Quaternion<T>
{
	pub fn normalise(self) -> Self
		where T: Copy + Sqrt<T> + Div<Output=T> + Mul<Output=T> + Add<Output=T>
	{
		let magnitude = (self.i * self.i + self.j * self.j + self.k * self.k + self.r * self.r).sqrt();
		Self { r: self.r / magnitude, i: self.i / magnitude, j: self.j / magnitude, k: self.k / magnitude }
	}
	
	pub fn inverse(self) -> Self
		where T: Neg<Output=T>
	{
		Self { r: -self.r, i: self.i, j: self.j, k: self.k }
	}
	
	pub fn from_axis_angle(v: Vec3<T>) -> Self
		where T: Copy + Sqrt<T> + Trig + Half + Add<Output=T> + Mul<Output=T>
	{
		let magnitude = v.magnitude();
		let (factor, r) = magnitude.half().sin_cos();
		let (i, j, k) = (factor * v.x, factor * v.y, factor * v.z);
		Self { r: r, i: i, j: j, k: k }
	}
	
	pub fn into<U>(self) -> Quaternion<U>
		where T: Into<U>
	{
		Quaternion { r: self.r.into(), i: self.i.into(), j: self.j.into(), k: self.k.into() }
	}
	
	pub fn rot_mat(self) -> Mat3<T>
		where T: Copy + Two + One + Mul<Output=T> + Add<Output=T> + Sub<Output=T>
	{
		let (r, i, j, k) = (self.r, self.i, self.j, self.k);
		mat3(
			vec3(T::one() - T::two() * (j * j + k * k), T::two() * (i * j - r * k), T::two() * (i * k + r * j)),
			vec3(T::two() * (i * j + r * k), T::one() - T::two() * (i * i + k * k), T::two() * (j * k - r * i)),
			vec3(T::two() * (i * k - r * j), T::two() * (j * k + r * i), T::one() - T::two() * (i * i + j * j)),
		)
	}
	
	pub fn convert<U>(self) -> Quaternion<U>
		where T: Into<U>
	{
		Quaternion { r: self.r.into(), i: self.i.into(), j: self.j.into(), k: self.k.into() }
	}
}

pub fn quaternion<T>(r: T, i: T, j: T, k: T) -> Quaternion<T> {
	Quaternion { r, i, j, k }
}

impl<T> Mul<Quaternion<T>> for Quaternion<T>
	where T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Copy
{
	type Output = Self;
	
	fn mul(self, other: Self) -> Self {
		Quaternion {
			r: self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k,
			i: self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j,
			j: self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i,
			k: self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r,
		}
	}
}

impl<T> ArrayTuple for Quaternion<T> {
	type Array = [T; 4];
	type Tuple = (T,T,T,T);
	fn into_array(self) -> [T; 4] { let Quaternion{r,i,j,k}=self; [r,i,j,k] }
	fn into_tuple(self) -> (T,T,T,T) { let Quaternion{r,i,j,k}=self; (r,i,j,k) }
}

impl<T> Default for Quaternion<T>
	where T: Default + One
{
	fn default() -> Self {
		Self { r: T::one(), i: T::default(), j: T::default(), k: T::default() }
	}
}

impl<T: fmt::Display> fmt::Display for Quaternion<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} + {}i + {}j + {}k)", self.r, self.i, self.j, self.k)
	}
}

macro_rules! convert {
	($T: ty, $($U: ident),+) => {$(
		impl Quaternion<$T> {
			pub fn $U(self) -> Quaternion<$U> {
				quaternion(self.r as $U, self.i as $U, self.j as $U, self.k as $U)
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
