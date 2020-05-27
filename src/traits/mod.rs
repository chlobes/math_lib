pub mod numbers;

use std::marker::Sized;

pub fn dot<T: Dot<Output=O>, O>(a: T, b: T) -> O {
	a.dot(b)
}

pub trait Dot {
	type Output;
	fn dot(self, other: Self) -> Self::Output;
} //intentionally not implemented for numeric types to prevent confusion, if I want to multiply I'll type a '*', if I type dot(float_a, float_b) that's a mistake it should not compile

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
	
	impl NiceFmt for $t {
		fn nice_fmt(&self, limit: usize, pad: bool) -> String {
			let limit = limit.max(1);
			let mut result = if format!("{}",self).len() <= limit {
				format!("{}",self)
			} else if (self.abs() + self.is_sign_negative() as usize as $t).log10() < limit as $t {
				let l = limit.saturating_sub(self.abs() as usize + self.is_sign_negative() as usize + 1);
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
			if pad { while result.len() < limit { result.push(' '); } }
			result
		}
	}
}

float_impl!(f32);
float_impl!(f64);
