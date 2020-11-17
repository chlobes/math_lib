pub trait Zero {
	fn zero() -> Self;
}

pub trait One {
	fn one() -> Self;
}

pub trait Two {
	fn two() -> Self;
}

macro impl_ints($($t: ty),+) {
	$(
		impl Zero for $t {
			fn zero() -> Self { 0 }
		}
		
		impl One for $t {
			fn one() -> Self { 1 }
		}
		
		impl Two for $t {
			fn two() -> Self { 2 }
		}
	)+
}

macro impl_floats($($t: ident),+) {
	$(
		impl Zero for $t {
			fn zero() -> Self { 0.0 }
		}
		
		impl One for $t {
			fn one() -> Self { 1.0 }
		}
		
		impl Two for $t {
			fn two() -> Self { 2.0 }
		}
	)+
}

impl_ints!(u8,u16,u32,u64,usize,i8,i16,i32,i64,isize);
impl_floats!(f32,f64);
