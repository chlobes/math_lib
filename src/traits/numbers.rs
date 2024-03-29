pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
pub trait Two {
    fn two() -> Self;
}
pub trait Abs {
    fn abs(self) -> Self;
}

macro impl_ints($($t: ty),*) {
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
        impl Abs for $t {
            fn abs(self) -> Self { self.abs_diff(0) as $t }
        }
    )*
}

macro impl_floats($($t: ident),*) {
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
        impl Abs for $t {
            fn abs(self) -> Self { self.abs() }
        }
    )*
}

impl_ints!(u8,u16,u32,u64,usize,i8,i16,i32,i64,isize);
impl_floats!(f32,f64);

impl Zero for bool {
    fn zero() -> Self { false }
}
impl One for bool {
    fn one() -> Self { true }
}
