use crate::prelude::*;
use crate::vec3::*;

pub use crate::prelude::{dot,distance,distance_squared,orthog_dist,angle_between,Zero,One,Abs,VecOps};

#[repr(C)]
#[derive(Debug,Default,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct Vec4<T> { pub x: T, pub y: T, pub z: T, pub w: T }

impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from((x,y,z,w): (T, T, T, T)) -> Self {
        vec4(x,y,z,w)
    }
}
impl<T> From<[T; 4]> for Vec4<T> {
    fn from([x,y,z,w]: [T; 4]) -> Self {
        vec4(x,y,z,w)
    }
}
impl<T> From<Vec4<T>> for (T, T, T, T) {
    fn from(v: Vec4<T>) -> Self {
        (v.x,v.y,v.z,v.w)
    }
}
impl<T> From<Vec4<T>> for [T; 4] {
    fn from(v: Vec4<T>) -> Self {
        [v.x,v.y,v.z,v.w]
    }
}

impl_vec!(Vec4, vec4, (x, y, z, w));

impl<T> Vec4<T> {
    pub fn downsize(self) -> Vec3<T> {
        vec3(self.x,self.y,self.z)
    }
}

macro impl_ints1($($U: ident),*) {
    $(
        impl Vec4<isize> {
            pub fn $U(self) -> Self {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
        impl Vec4<i64> {
            pub fn $U(self) -> Self {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
        impl Vec4<i32> {
            pub fn $U(self) -> Self {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
        impl Vec4<i16> {
            pub fn $U(self) -> Self {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
        impl Vec4<i8> {
            pub fn $U(self) -> Self {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
    )*
}

macro impl_ints2($($U: ident),*) {
    $(
        impl Vec4<isize> {
            pub fn $U(self) -> Vec4<bool> {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
        impl Vec4<i64> {
            pub fn $U(self) -> Vec4<bool> {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
        impl Vec4<i32> {
            pub fn $U(self) -> Vec4<bool> {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
        impl Vec4<i16> {
            pub fn $U(self) -> Vec4<bool> {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
        impl Vec4<i8> {
            pub fn $U(self) -> Vec4<bool> {
                vec4(self.x.$U(), self.y.$U(), self.z.$U(), self.w.$U())
            }
        }
    )*
}

macro impl_floats1($($U: ident),*) {
    $(
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
    )*
}

macro impl_floats2($($U: ident),*) {
    $(
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
    )*
}

//component-wise functions
//certain conversion and trig functions not implemented to avoid confusion
impl_ints1!(signum,swap_bytes/*,reverse_bits*/,to_be,to_le,wrapping_neg,wrapping_abs);
impl_ints2!(is_positive,is_negative);
impl_floats1!(floor,ceil,round,trunc,fract,signum,sqrt,exp,exp2,ln,log2,log10,cbrt,exp_m1,ln_1p);
impl_floats2!(is_infinite,is_finite,is_normal,is_sign_positive,is_sign_negative);

impl<T: fmt::Display> fmt::Display for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "({4:.*}, {5:.*}, {6:.*}, {7:.*})", p, p, p, p, self.x, self.y, self.z, self.w)
        } else {
            write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
        }
    }
}

impl<T: NiceFmt> NiceFmt for Vec4<T> {
    fn nice_fmt(&self, limit: usize, pad: bool) -> String {
        format!("({}, {}, {}, {})", self.x.nice_fmt(limit, pad), self.y.nice_fmt(limit, pad), self.z.nice_fmt(limit, pad), self.w.nice_fmt(limit, pad))
    }
}

impl<T: fmt::LowerExp> fmt::LowerExp for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "({4:.*e}, {5:.*e}, {6:.*e}, {7:.*e})", p, p, p, p, self.x, self.y, self.z, self.w)
        } else {
            write!(f, "({:e}, {:e}, {:e}, {:e})", self.x, self.y, self.z, self.w)
        }
    }
}

impl<T: FromStr> FromStr for Vec4<T> {
    type Err = <T as FromStr>::Err;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> { //TODO: use a collect-like method?
        let mut words: Vec<&str> = input.split(|c: char| c.is_whitespace() || c == ',')
            .map(|s| s.trim_matches(BRACKETS)).filter(|s| !s.is_empty()).collect();
        while words.len() < 4 { words.push(""); }
        let x = words[0].parse()?;
        let y = words[1].parse()?;
        let z = words[2].parse()?;
        let w = words[3].parse()?;
        Ok(vec4(x, y, z, w))
    }
}

#[cfg(feature = "raylib")]
impl From<Vec4<f32>> for Vector4 {
    fn from(a: Vec4<f32>) -> Self {
        Self { x: a.x, y: a.y, z: a.z, w: a.w }
    }
}
#[cfg(feature = "raylib")]
impl From<Vec4<f32>> for Vector4_ {
    fn from(a: Vec4<f32>) -> Self {
        Self { x: a.x, y: a.y, z: a.z, w: a.w }
    }
}

#[cfg(feature = "raylib")]
impl From<Vec4<u8>> for Color {
    fn from(a: Vec4<u8>) -> Self {
        Self { r: a.x, g: a.y, b: a.z, a: a.w }
    }
}
#[cfg(feature = "raylib")]
impl From<Vec4<u8>> for Color_ {
    fn from(a: Vec4<u8>) -> Self {
        Color_ { r: a.x, g: a.y, b: a.z, a: a.w }
    }
}

#[cfg(feature = "raylib")]
impl From<Vec4<f32>> for Color {
    fn from(a: Vec4<f32>) -> Self {
        (a * 255.0).min(Vec4::one() * 255.0).u8().into()
    }
}
#[cfg(feature = "raylib")]
impl From<Vec4<f32>> for Color_ {
    fn from(a: Vec4<f32>) -> Self {
        (a * 255.0).min(Vec4::one() * 255.0).u8().into()
    }
}

macro convert($T: ty, $($U: ident),*) {
    $(
        impl Vec4<$T> {
            pub fn $U(self) -> Vec4<$U> {
                vec4(self.x as $U, self.y as $U, self.z as $U, self.w as $U)
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

/*impl<T: ValConsts + Serialize> Serialize for Vec4<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Vec4", 4)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.serialize_field("w", &self.w)?;
        state.end()
    }
}*/
