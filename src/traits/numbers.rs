pub trait Half {
	fn half(self) -> Self;
}

impl Half for f32 {
	fn half(self) -> Self { self * 0.5 }
}

impl Half for f64 {
	fn half(self) -> Self { self * 0.5 }
}

impl Half for i8 {
	fn half(self) -> Self { self / 2 }
}

impl Half for i16 {
	fn half(self) -> Self { self / 2 }
}

impl Half for i32 {
	fn half(self) -> Self { self / 2 }
}

impl Half for i64 {
	fn half(self) -> Self { self / 2 }
}

impl Half for u8 {
	fn half(self) -> Self { self / 2 }
}

impl Half for u16 {
	fn half(self) -> Self { self / 2 }
}

impl Half for u32 {
	fn half(self) -> Self { self / 2 }
}

impl Half for u64 {
	fn half(self) -> Self { self / 2 }
}

pub trait One {
	fn one() -> Self;
}

impl One for f32 {
	fn one() -> Self { 1.0 }
}

impl One for f64 {
	fn one() -> Self { 1.0 }
}

impl One for i8 {
	fn one() -> Self { 1 }
}

impl One for i16 {
	fn one() -> Self { 1 }
}

impl One for i32 {
	fn one() -> Self { 1 }
}

impl One for i64 {
	fn one() -> Self { 1 }
}

impl One for u8 {
	fn one() -> Self { 1 }
}

impl One for u16 {
	fn one() -> Self { 1 }
}

impl One for u32 {
	fn one() -> Self { 1 }
}

impl One for u64 {
	fn one() -> Self { 1 }
}

pub trait Two {
	fn two() -> Self;
}

impl Two for f32 {
	fn two() -> Self { 2.0 }
}

impl Two for f64 {
	fn two() -> Self { 2.0 }
}

impl Two for i8 {
	fn two() -> Self { 2 }
}

impl Two for i16 {
	fn two() -> Self { 2 }
}

impl Two for i32 {
	fn two() -> Self { 2 }
}

impl Two for i64 {
	fn two() -> Self { 2 }
}

impl Two for u8 {
	fn two() -> Self { 2 }
}

impl Two for u16 {
	fn two() -> Self { 2 }
}

impl Two for u32 {
	fn two() -> Self { 2 }
}

impl Two for u64 {
	fn two() -> Self { 2 }
}