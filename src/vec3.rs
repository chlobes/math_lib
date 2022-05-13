use crate::prelude::*;
use crate::vec2::*;
use crate::vec4::*;

pub use crate::prelude::{dot,distance,distance_squared,orthog_dist,angle_between,Zero,One,Abs,VecOps};

#[repr(C)]
#[derive(Debug,Default,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct Vec3<T> { pub x: T, pub y: T, pub z: T, }

impl<T> ArrayTuple for Vec3<T> {
	type Array = [T; 3];
	type Tuple = (T, T, T);
	fn into_array(self) -> [T; 3] {	[self.x,self.y,self.z] }
	fn into_tuple(self) -> (T, T, T) { self.into_array().into_tuple() }
}
impl<T> From<(T, T, T)> for Vec3<T> {
	fn from(t: (T, T, T)) -> Self {
		let (x,y,z) = t;
		vec3(x,y,z)
	}
}
impl<T> From<[T; 3]> for Vec3<T> {
	fn from(t: [T; 3]) -> Self {
		t.into_tuple().into()
	}
}

impl_vec!(Vec3, vec3, (x, y, z));

impl<T> Vec3<T> {
	pub fn downsize(self) -> Vec2<T> {
		vec2(self.x, self.y)
	}
	
	pub fn extend(self, w: T) -> Vec4<T> {
		vec4(self.x, self.y, self.z, w)
	}
}

pub fn cross<T: Copy + Mul<Output=T> + Sub<Output=T>>(v: Vec3<T>, u: Vec3<T>) -> Vec3<T> {
	vec3(
		v.y * u.z - v.z * u.y,
		v.z * u.x - v.x * u.z,
		v.x * u.y - v.y * u.x
	)
}
//TODO: generalize these to all VecN
macro impl_ints1($($U: ident),*) {
	$(
		impl Vec3<isize> {
			pub fn $U(self) -> Self {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<i64> {
			pub fn $U(self) -> Self {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<i32> {
			pub fn $U(self) -> Self {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<i16> {
			pub fn $U(self) -> Self {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<i8> {
			pub fn $U(self) -> Self {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
	)*
}

macro impl_ints2($($U: ident),*) {
	$(
		impl Vec3<isize> {
			pub fn $U(self) -> Vec3<bool> {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<i64> {
			pub fn $U(self) -> Vec3<bool> {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<i32> {
			pub fn $U(self) -> Vec3<bool> {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<i16> {
			pub fn $U(self) -> Vec3<bool> {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
		impl Vec3<i8> {
			pub fn $U(self) -> Vec3<bool> {
				vec3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
	)*
}

macro impl_floats1($($U: ident),*) {
	$(
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
	)*
}

macro impl_floats2($($U: ident),*) {
	$(
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
	)*
}

//component-wise functions
//certain conversion and trig functions not implemented to avoid confusion
impl_ints1!(signum,swap_bytes/*,reverse_bits*/,to_be,to_le,wrapping_neg,wrapping_abs);
impl_ints2!(is_positive,is_negative);
impl_floats1!(floor,ceil,round,trunc,fract,signum,sqrt,exp,exp2,ln,log2,log10,cbrt,exp_m1,ln_1p);
impl_floats2!(is_infinite,is_finite,is_normal,is_sign_positive,is_sign_negative);

impl<T: fmt::Display> fmt::Display for Vec3<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(p) = f.precision() {
			write!(f, "({3:.*}, {4:.*}, {5:.*})", p, p, p, self.x, self.y, self.z)
		} else {
			write!(f, "({}, {}, {})", self.x, self.y, self.z)
		}
	}
}

impl<T: NiceFmt> NiceFmt for Vec3<T> {
	fn nice_fmt(&self, limit: usize, pad: bool) -> String {
		format!("({}, {}, {})", self.x.nice_fmt(limit, pad), self.y.nice_fmt(limit, pad), self.z.nice_fmt(limit, pad))
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

impl<T: FromStr> FromStr for Vec3<T> {
	type Err = <T as FromStr>::Err;
	
	fn from_str(input: &str) -> Result<Self, Self::Err> { //TODO: use a collect-like method?
		let mut words: Vec<&str> = input.split(|c: char| c.is_whitespace() || c == ',')
			.map(|s| s.trim_matches(BRACKETS)).filter(|s| !s.is_empty()).collect();
		while words.len() < 3 { words.push(""); }
		let x = words[0].parse()?;
		let y = words[1].parse()?;
		let z = words[2].parse()?;
		Ok(vec3(x, y, z))
	}
}

impl From<Vec3<f32>> for Vector3 {
	fn from(a: Vec3<f32>) -> Self {
		Self { x: a.x, y: a.y, z: a.z }
	}
}
impl From<Vec3<f32>> for Vector3_ {
	fn from(a: Vec3<f32>) -> Self {
		Self { x: a.x, y: a.y, z: a.z }
	}
}

macro convert($T: ty, $($U: ident),*) {
	$(
		impl Vec3<$T> {
			pub fn $U(self) -> Vec3<$U> {
				vec3(self.x as $U, self.y as $U, self.z as $U)
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

/*impl<T: ValConsts + Serialize> Serialize for Vec3<T> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut state = serializer.serialize_struct("Vec3", 3)?;
		state.serialize_field("x", &self.x)?;
		state.serialize_field("y", &self.y)?;
		state.serialize_field("z", &self.z)?;
		state.end()
	}
}*/
