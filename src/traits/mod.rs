pub mod numbers;

use std::marker::Sized;

pub trait Sqrt<T> {
	fn sqrt(self) -> T;
}

impl Sqrt<f32> for f32 {
	fn sqrt(self) -> f32 { self.sqrt() }
}

impl Sqrt<f64> for f64 {
	fn sqrt(self) -> f64 { self.sqrt() }
}

pub fn dot<T: Dot<Output=O>, O>(a: T, b: T) -> O {
	a.dot(b)
}

pub trait Dot {
	type Output;
	fn dot(self, other: Self) -> Self::Output;
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

impl Trig for f32 {
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

impl Trig for f64 {
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

pub trait NiceFmt {
	fn nice_fmt(&self, limit: usize, pad: bool) -> String;
}

macro float_impl($typ: ty) {
	impl NiceFmt for $typ {
		fn nice_fmt(&self, limit: usize, pad: bool) -> String {
			let limit = limit.max(1);
			if format!("{}",self).len() <= limit {
				let mut result = format!("{}",self);
				if pad {
					while result.len() < limit {
						result.push(' ');
					}
				}
				result
			} else if (self.abs() as usize + self.is_sign_negative() as usize) < limit.pow(10) {
				//TODO: rounding
				let mut result = format!("{}",self);
				while result.len() > limit {
					result.pop().unwrap();
				}
				let c = result.pop().unwrap();
				if c == '.' {
					if pad {
						result.push(' ');
					}
				} else {
					result.push(c);
				}
				result
			} else {
				let mut result = format!("{:.*e}",limit-2,self);
				if pad {
					while result.len() < limit {
						result.push(' ');
					}
				}
				result
			}
		}
	}
}

float_impl!(f32);
float_impl!(f64);
