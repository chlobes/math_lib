use crate::prelude::*;
use crate::vec3::*;

pub use crate::prelude::{dot,distance,distance_squared,orthog_dist,angle_between,Zero,One,Abs,VecOps};

#[repr(C)]
#[derive(Debug,Default,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct Vec2<T> { pub x: T, pub y: T }

impl<T> ArrayTuple for Vec2<T> {
	type Array = [T; 2];
	type Tuple = (T, T);
	fn into_array(self) -> [T; 2] {	[self.x,self.y] }
	fn into_tuple(self) -> (T, T) { self.into_array().into_tuple() }
}
impl<T> From<(T, T)> for Vec2<T> {
	fn from(t: (T, T)) -> Self {
		let (x,y) = t;
		vec2(x,y)
	}
}
impl<T> From<[T; 2]> for Vec2<T> {
	fn from(t: [T; 2]) -> Self {
		t.into_tuple().into()
	}
}

impl_vec!(Vec2, vec2, (x, y));

impl<T> Vec2<T> {
	pub fn extend(self, z: T) -> Vec3<T> {
		vec3(self.x, self.y, z)
	}
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

macro impl_ints2($($U: ident),*) {
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
impl_ints1!(signum,swap_bytes/*,reverse_bits*/,to_be,to_le,wrapping_neg,wrapping_abs);
impl_ints2!(is_positive,is_negative);
impl_floats1!(floor,ceil,round,trunc,fract,signum,sqrt,exp,exp2,ln,log2,log10,cbrt,exp_m1,ln_1p);
impl_floats2!(is_infinite,is_finite,is_normal,is_sign_positive,is_sign_negative);

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

impl From<Vec2<f32>> for Vector2 {
	fn from(a: Vec2<f32>) -> Self {
		Self { x: a.x, y: a.y }
	}
}
impl From<Vec2<f32>> for Vector2_ {
	fn from(a: Vec2<f32>) -> Self {
		Self { x: a.x, y: a.y }
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
