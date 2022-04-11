use crate::prelude::*;
use std::marker::Sized;

mod numbers;
pub use numbers::*;

pub trait Sqrt<T> {
	fn sqrt(self) -> T;
}

pub trait Trig: Sized {
	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;
	fn asin(self) -> Self;
	fn acos(self) -> Self;
	fn atan(self) -> Self;
	fn atan2(self, other: Self) -> Self;
	fn sin_cos(self) -> (Self, Self);
	fn sinh(self) -> Self;
	fn cosh(self) -> Self;
	fn tanh(self) -> Self;
	fn asinh(self) -> Self;
	fn acosh(self) -> Self;
	fn atanh(self) -> Self;
}

pub trait IsNan: PartialOrd {
	fn is_nan(&self) -> bool;
	fn non_nan_max(self, other: Self) -> Self;
	fn non_nan_min(self, other: Self) -> Self;
}

impl<T: PartialOrd + Sized> IsNan for T {
	default fn is_nan(&self) -> bool { false }
	default fn non_nan_max(self, other: Self) -> Self { if other.is_nan() || self > other { self } else { other } }
	default fn non_nan_min(self, other: Self) -> Self { if other.is_nan() || self < other { self } else { other } }
}

pub trait NiceFmt {
	fn nice_fmt(&self, limit: usize, pad: bool) -> String;
}

macro float_impl($t: ty) {
	impl Sqrt<Self> for $t {
		fn sqrt(self) -> Self { self.sqrt() }
	}
	
	impl Trig for $t {
		fn sin(self) -> Self { self.sin() }
		fn cos(self) -> Self { self.cos() }
		fn tan(self) -> Self { self.tan() }
		fn asin(self) -> Self { self.asin() }
		fn acos(self) -> Self { self.acos()	}
		fn atan(self) -> Self { self.atan()	}
		fn atan2(self, other: Self) -> Self { self.atan2(other) }
		fn sin_cos(self) -> (Self, Self) { self.sin_cos() }
		fn sinh(self) -> Self { self.sinh() }
		fn cosh(self) -> Self { self.cosh() }
		fn tanh(self) -> Self { self.tanh() }
		fn asinh(self) -> Self { self.asinh() }
		fn acosh(self) -> Self { self.acosh() }
		fn atanh(self) -> Self { self.atanh() }
	}
	
	impl IsNan for $t {
		fn is_nan(&self) -> bool { <$t>::is_nan(*self) }
		fn non_nan_max(self, other: Self) -> Self { self.max(other) }
		fn non_nan_min(self, other: Self) -> Self { self.min(other) }
	}
	
	impl NiceFmt for $t {
		fn nice_fmt(&self, limit: usize, pad: bool) -> String {
			let limit = limit.saturating_sub(1);
			let mut result = if format!("{}",self).len() <= limit {
				format!("{}",self)
			} else if self.abs().log10() + self.is_sign_negative() as usize as $t < limit as $t {
				let l = limit.saturating_sub(self.abs().log10() as usize + self.is_sign_negative() as usize + 1);
				if l == 0 {
					format!("{}",self.round()) //this should be safe because we drop the decimal point so even if it rounds up and gains an extra digit we have a spare space to use
				} else {
					let mut result = format!("{:.*}",l,self);
					if result.contains('.') {
						while result.chars().last() == Some('0') {
							result.pop();
						}
						if result.chars().last() == Some('.') {
							result.pop();
						}
					}
					result
				}
			} else {
				let exp = if *self == 0.0 { 0 } else { self.abs().log10().floor() as i32 + 1 };
				let exp_len = if exp.abs() >= 100 {
					3
				} else if exp.abs() >= 10 {
					2
				} else {
					1
				} + exp.is_negative() as usize;
				let limit = limit.saturating_sub(exp_len + self.is_sign_negative() as usize + 2); //we cannot properly represent the number *and* guarentee that it won't have too many characters if limit < 7, because we need up to 4 for exponent, 2 for mantissa, and 1 for the letter e
				//proper representation is prioritized over obeying the character limit
				format!("{:.*e}",limit,self)
			};
			let mut i = result.chars();
			let mut r = String::new(); r.push(i.next().unwrap_or('0')); r.push(i.next().unwrap_or('0'));
			let p = i.next().unwrap_or('0');
			if p != '.' && &r == "-0" {
				result.remove(0);
			}
			if pad { while result.len() <= limit { result.push(' '); } }
			result
		}
	}
}

float_impl!(f32);
float_impl!(f64);

pub trait Dot<T>: Copy + Sub<Output=Self> + Sized {
	fn dot(&self, other: &Self) -> T;
	fn distance_squared(&self, other: &Self) -> T {
		let x = *self - *other;
		x.dot(&x)
	}
}

pub trait Distance<T> {
	fn distance(&self, other: &Self) -> T;
}

impl<T: Sqrt<T>, V: Dot<T>> Distance<T> for V {
	fn distance(&self, other: &Self) -> T {
		self.distance_squared(other).sqrt()
	}
}
