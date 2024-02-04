use crate::prelude::*;

use crate::mat3::*;
use crate::vec3::*;

#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize,Deserialize))]
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct Quaternion<T> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}
pub fn quaternion<T>(r: T, i: T, j: T, k: T) -> Quaternion<T> {
    Quaternion { r, i, j, k }
}
//TODO: integer quaternions?
impl<T: Copy + Zero + One + Two + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Trig + Sqrt<T> + Neg<Output=T> + Abs> Quaternion<T> {
    pub fn ident() -> Self {
        quaternion(T::one(), T::zero(), T::zero(), T::zero())
    }
    
    pub fn magnitude(self) -> T {
        (self.i * self.i + self.j * self.j + self.k * self.k + self.r * self.r).sqrt()
    }
    
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self { r: self.r / mag, i: self.i / mag, j: self.j / mag, k: self.k / mag }
    }
    
    pub fn inv(self) -> Self {
        Self { r: -self.r, i: self.i, j: self.j, k: self.k }
    }
    
    pub fn from_euler_angles(input: Vec3<T>) -> Self {
        let (factor, r) = (input.magnitude() / T::two()).sin_cos();
        let (i, j, k) = (factor * input.x, factor * input.y, factor * input.z);
        quaternion(r, i, j, k)
    }
    
    pub fn rot_mat(self) -> Mat3<T> {
        let (r, i, j, k) = (self.r, self.i, self.j, self.k);
        mat3(
            vec3(T::one() - T::two() * (j * j + k * k), T::two() * (i * j - r * k), T::two() * (i * k + r * j)),
            vec3(T::two() * (i * j + r * k), T::one() - T::two() * (i * i + k * k), T::two() * (j * k - r * i)),
            vec3(T::two() * (i * k - r * j), T::two() * (j * k + r * i), T::one() - T::two() * (i * i + j * j)),
        )
    }
}

impl<T: Copy + Zero + One + Two + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Trig + Sqrt<T> + Neg<Output=T> + Abs> Default for Quaternion<T> {
    fn default() -> Self {
        Quaternion::ident()
    }
}

impl<T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T>> Mul<Quaternion<T>> for Quaternion<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k,
            i: self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j,
            j: self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i,
            k: self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r,
        }
    }
}

impl<T: Copy + Zero + One + Two + Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sub<Output=T> + Trig + Sqrt<T> + Neg<Output=T> + Abs> Product<Quaternion<T>> for Quaternion<T> {
    fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::ident(), |a, b| a * b)
    }
}

impl<T> Index<usize> for Quaternion<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.r,
            1 => &self.i,
            2 => &self.j,
            3 => &self.k,
            _ => panic!("index out of bounds, index is {} but the len is 4",index),
        }
    }
}

impl<T> IndexMut<usize> for Quaternion<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.r,
            1 => &mut self.i,
            2 => &mut self.j,
            3 => &mut self.k,
            _ => panic!("index out of bounds, index is {} but the len is 4",index),
        }
    }
}


impl<T: fmt::Display> fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "{4:.*} + {5:.*}i + {6:.*}j + {7:.*}k", p, p, p, p, self.r, self.i, self.j, self.k)
        } else {
            write!(f, "{} + {}i + {}j + {}k", self.r, self.i, self.j, self.k)
        }
    }
}

impl<T: NiceFmt> NiceFmt for Quaternion<T> {
    fn nice_fmt(&self, limit: usize, pad: bool) -> String {
        format!("({} + {}i + {}j + {}k)", self.r.nice_fmt(limit, pad), self.i.nice_fmt(limit, pad), self.j.nice_fmt(limit, pad), self.k.nice_fmt(limit, pad))
    }
}

impl<T: fmt::LowerExp> fmt::LowerExp for Quaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "{4:.*e} + {5:.*e}i + {6:.*e}j + {7:.*e}k", p, p, p, p, self.r, self.i, self.j, self.k)
        } else {
            write!(f, "{:e} + {:e}i + {:e}j + {:e}k", self.r, self.i, self.j, self.k)
        }
    }
}

impl<T: FromStr> FromStr for Quaternion<T> {
    type Err = <T as FromStr>::Err;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> { //TODO: use a collect-like method?
        let mut words: Vec<&str> = input.split(|c: char| c.is_whitespace() || c == ',')
            .map(|s| s.trim_matches(BRACKETS)).filter(|s| !s.is_empty()).collect();
        while words.len() < 4 { words.push(""); }
        Ok(quaternion(words[0].parse()?, words[1].parse()?, words[2].parse()?, words[3].parse()?))
    }
}

#[cfg(feature = "raylib")]
impl From<Quaternion<f32>> for RaylibQuaternion {
    fn from(a: Quaternion<f32>) -> Self {
        Self { x: a.i, y: a.j, z: a.k, w: a.r }
    }
}
#[cfg(feature = "raylib")]
impl From<Quaternion<f32>> for RaylibQuaternion_ {
    fn from(a: Quaternion<f32>) -> Self {
        Self { x: a.i, y: a.j, z: a.k, w: a.r }
    }
}

macro convert($T: ty, $($U: ident),*) {
    $(
        impl Quaternion<$T> {
            pub fn $U(self) -> Quaternion<$U> {
                quaternion(self.r as $U, self.i as $U, self.j as $U, self.k as $U)
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
