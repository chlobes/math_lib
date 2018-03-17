use std::ops::{Div, Mul, Add, Sub, Deref};
use std::fmt::Debug;
use std::marker::Copy;

pub trait Sqrt<T> {
	fn sqrt(self) -> T;
}

impl Sqrt<f32> for f32 {
	fn sqrt(self) -> f32 { self.sqrt() }
}

impl Sqrt<f64> for f64 {
	fn sqrt(self) -> f64 { self.sqrt() }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Vec3<T>
	where T: Copy + Debug + PartialOrd
{
	pub fn normalised(self) -> NormalisedVec3<T>
		where T: Sqrt<T> + Div<Output=T> + Mul<Output=T> + Add<Output=T>
	{
		NormalisedVec3(normalise(self))
	}
	
	pub fn magnitude(self) -> T
		where T: Sqrt<T> + Mul<Output=T> + Add<Output=T>
	{
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
}

pub fn vec<T>(x: T, y: T, z: T) -> Vec3<T>
	where T: Copy + Debug + PartialOrd
{
	Vec3 { x: x, y: y, z: z }
}

pub fn dot<T: Mul<Output=T> + Add<Output=T>>(v: Vec3<T>, u: Vec3<T>) -> T
	where T: Copy + Debug + PartialOrd
{
	v.x * u.x + v.y * u.y + v.z * u.z
}

pub fn cross<T: Mul<Output=T> + Sub<Output=T>>(v: Vec3<T>, u: Vec3<T>) -> Vec3<T>
	where T: Copy + Debug + PartialOrd
{
	vec(v.y * u.z - v.z * u.y, v.z * u.x - v.x * u.z, v.x * u.y - v.y * u.x)
}

pub fn normalise<T: Div<Output=T> + Mul<Output=T> + Add<Output=T> + Sqrt<T>>(v: Vec3<T>) -> Vec3<T>
	where T: Copy + Debug + PartialOrd
{
	vec(v.x / v.magnitude(), v.y / v.magnitude(), v.z / v.magnitude())
}

pub fn distance<T: Sub<Output=T> + Mul<Output=T> + Add<Output=T> + Sqrt<T>>(v: Vec3<T>, u: Vec3<T>) -> T
	where T: Copy + Debug + PartialOrd
{
	(v - u).magnitude()
}

impl<T> Div<T> for Vec3<T>
	where T: Copy + Debug + PartialOrd + Div<Output=T>
{
	type Output = Vec3<T>;
	
	fn div(self, other: T) -> Vec3<T> {
		vec(self.x / other, self.y / other, self.z / other)
	}
}

impl<T> Div<Vec3<T>> for Vec3<T>
	where T: Copy + Debug + PartialOrd + Div<Output=T>
{
	type Output = Vec3<T>;
	
	fn div(self, other: Vec3<T>) -> Vec3<T> {
		vec(self.x / other.x, self.y / other.y, self.z / other.z)
	}
}

impl<T> Mul<T> for Vec3<T>
	where T: Copy + Debug + PartialOrd + Mul<Output=T>
{
	type Output = Vec3<T>;
	
	fn mul(self, other: T) -> Vec3<T> {
		vec(self.x * other, self.y * other, self.z * other)
	}
}

impl<T> Mul<Vec3<T>> for Vec3<T>
	where T: Copy + Debug + PartialOrd + Mul<Output=T>
{
	type Output = Vec3<T>;
	
	fn mul(self, other: Vec3<T>) -> Vec3<T> {
		vec(self.x * other.x, self.y * other.y, self.z * other.z)
	}
}

impl<T> Add<T> for Vec3<T>
	where T: Copy + Debug + PartialOrd + Add<Output=T>
{
	type Output = Vec3<T>;
	
	fn add(self, other: T) -> Vec3<T> {
		vec(self.x + other, self.y + other, self.z + other)
	}
}

impl<T> Add<Vec3<T>> for Vec3<T>
	where T: Copy + Debug + PartialOrd + Add<Output=T>
{
	type Output = Vec3<T>;
	
	fn add(self, other: Vec3<T>) -> Vec3<T> {
		vec(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl<T> Sub<T> for Vec3<T>
	where T: Copy + Debug + PartialOrd + Sub<Output=T>
{
	type Output = Vec3<T>;
	
	fn sub(self, other: T) -> Vec3<T> {
		vec(self.x - other, self.y - other, self.z - other)
	}
}

impl<T> Sub<Vec3<T>> for Vec3<T>
	where T: Copy + Debug + PartialOrd + Sub<Output=T>
{
	type Output = Vec3<T>;
	
	fn sub(self, other: Vec3<T>) -> Vec3<T> {
		vec(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NormalisedVec3<T>(Vec3<T>);

impl<T> Deref for NormalisedVec3<T> {
	type Target = Vec3<T>;
	
	fn deref(&self) -> &Vec3<T> {
		&self.0
	}
}