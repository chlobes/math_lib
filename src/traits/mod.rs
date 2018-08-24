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
