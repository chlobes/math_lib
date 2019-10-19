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

const NICE_FMT_LIMIT: usize = 8;

pub trait NiceFmt {
	fn nice_fmt(&self) -> String;
}

impl NiceFmt for f32 {
	fn nice_fmt(&self) -> String {
		if format!("{}",self).len() <= NICE_FMT_LIMIT {
			format!("{}",self)
		} else if (self.abs() as usize + self.is_sign_negative() as usize) < NICE_FMT_LIMIT.pow(10) {
			//TODO: rounding
			let mut result = format!("{}",self);
			while result.len() > NICE_FMT_LIMIT {
				result.pop().unwrap();
			}
			let c = result.pop().unwrap();
			if c == '.' {
				result.push(' ');
			} else {
				result.push(c);
			}
			result
		} else {
			format!("{:.*e}",NICE_FMT_LIMIT-2,self)
		}
	}
}

impl NiceFmt for f64 {
	fn nice_fmt(&self) -> String {
		if format!("{}",self).len() <= NICE_FMT_LIMIT {
			format!("{}",self)
		} else if (self.abs() as usize + self.is_sign_negative() as usize) < NICE_FMT_LIMIT.pow(10) {
			//TODO: rounding
			let mut result = format!("{}",self);
			while result.len() > NICE_FMT_LIMIT {
				result.pop().unwrap();
			}
			let c = result.pop().unwrap();
			if c == '.' {
				result.push(' ');
			} else {
				result.push(c);
			}
			result
		} else {
			format!("{:.*e}",NICE_FMT_LIMIT-2,self)
		}
	}
}
