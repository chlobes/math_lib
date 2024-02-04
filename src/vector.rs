use crate::prelude::*;

pub trait VecOps<T>: Copy + Sized + Mul<Output=Self> + Sub<Output=Self> + Abs {
    fn sum_elem(self) -> T;
    fn dot(self, rhs: Self) -> T {
        (self * rhs).sum_elem()
    }
    fn distance_squared(self, rhs: Self) -> T {
        (self - rhs).dot(self - rhs)
    }
    fn orthog_dist(self, rhs: Self) -> T {
        (self - rhs).abs().sum_elem()
    }
}

pub fn dot<V: VecOps<T>, T>(a: V, b: V) -> T {
    a.dot(b)
}
pub fn distance_squared<V: VecOps<T>, T>(a: V, b: V) -> T {
    a.distance_squared(b)
}
pub fn distance<V: VecOps<T>, T: Sqrt<T>>(a: V, b: V) -> T {
    a.distance_squared(b).sqrt()
}
pub fn orthog_dist<V: VecOps<T>, T>(a: V, b: V) -> T {
    a.orthog_dist(b)
}
pub fn normalize<V: VecOps<T> + Div<T,Output=V>, T: Sqrt<T>>(v: V) -> V {
    v / dot(v, v).sqrt()
}
pub fn angle_between<V: VecOps<T> + Div<T,Output=V>, T: Trig + Sqrt<T>>(a: V, b: V) -> T {
    let (a, b) = (normalize(a), normalize(b));
    dot(a, b).acos()
}

//macros 2.0 are apparently cabbage and don't let you use functions that are defined inside them -_-
#[macro_export]
macro_rules! impl_vec {
($type: ident, $vec: ident, ($($field: ident),*)) => {
    pub const fn $vec<T>($($field: T,)*) -> $type<T> {
        $type { $($field,)* }
    }
    
    impl<T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Abs> VecOps<T> for $type<T> {
        fn sum_elem(self) -> T {
            let arr: [T; _] = self.into();
            let mut iter = arr.iter();
            let mut r = *iter.next().unwrap();
            while let Some(&x) = iter.next() {
                r = r + x;
            }
            r
        }
    }
    
    impl<T: Copy> $type<T> {
        pub fn ident() -> Self
            where T: Zero + One {
            let mut r = $type { $($field: T::zero(),)* };
            r.x = T::one();
            r
        }
        
        pub fn mul_elem(self) -> T
            where T: One + Mul<Output=T> {
            $(self.$field * )* T::one()
        }
        
        pub fn min(self, rhs: Self) -> Self
            where T: IsNan {
            $vec($(self.$field.non_nan_min(rhs.$field),)*)
        }
        
        pub fn max(self, rhs: Self) -> Self
            where T: IsNan {
            $vec($(self.$field.non_nan_max(rhs.$field),)*)
        }
        
        pub fn clamp(self, min: Self, max: Self) -> Self
            where T: IsNan {
            self.min(max).max(min)
        }
        
        pub fn min_elem(self) -> T
            where T: IsNan {
            let arr: [T; _] = self.into();
            let mut iter = arr.iter();
            let mut r = *iter.next().unwrap();
            while let Some(&x) = iter.next() {
                r = r.non_nan_min(x);
            }
            r
        }
        
        pub fn max_elem(self) -> T
            where T: IsNan {
            let arr: [T; _] = self.into();
            let mut iter = arr.iter();
            let mut r = *iter.next().unwrap();
            while let Some(&x) = iter.next() {
                r = r.non_nan_max(x);
            }
            r
        }
        
        pub fn is_nan(self) -> $type<bool>
            where T: IsNan {
            $vec($(self.$field.is_nan(),)*)
        }
        
        pub fn magnitude(self) -> T
            where T: Sqrt<T>, Self: VecOps<T> {
            self.dot(self).sqrt()
        }
        
        pub fn normalize(self) -> Self
            where T: Div<Output=T> + Sqrt<T>, Self: VecOps<T> {
            normalize(self)
        }
        
        pub fn normalize_or_zero(self) -> Self
            where T: Zero + Div<Output=T> + Sqrt<T> + IsNan, Self: VecOps<T> {
            let r = self.normalize();
            if r.is_nan().or() { Self::zero() } else { r }
        }
    }
    
    impl $type<bool> {
        pub fn or(self) -> bool {
            $(self.$field ||)* false
        }
        
        pub fn and(self) -> bool {
            $(self.$field &&)* true
        }
    }
    
    impl<T: Mul<Output=T>> Mul for $type<T> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            $vec($(self.$field * rhs.$field,)*)
        }
    }
    
    impl<T: MulAssign> MulAssign for $type<T> {
        fn mul_assign(&mut self, rhs: Self) {
            $(self.$field *= rhs.$field;)*
        }
    }
    
    impl<T: Div<Output=T>> Div for $type<T> {
        type Output = Self;
        fn div(self, rhs: Self) -> Self {
            $vec($(self.$field / rhs.$field,)*)
        }
    }
    
    impl<T: DivAssign> DivAssign for $type<T> {
        fn div_assign(&mut self, rhs: Self) {
            $(self.$field /= rhs.$field;)*
        }
    }
    
    impl<T: Add<Output=T>> Add for $type<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            $vec($(self.$field + rhs.$field,)*)
        }
    }
    
    impl<T: AddAssign> AddAssign for $type<T> {
        fn add_assign(&mut self, rhs: Self) {
            $(self.$field += rhs.$field;)*
        }
    }
    
    impl<T: Sub<Output=T>> Sub for $type<T> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            $vec($(self.$field - rhs.$field,)*)
        }
    }
    
    impl<T: SubAssign> SubAssign for $type<T> {
        fn sub_assign(&mut self, rhs: Self) {
            $(self.$field -= rhs.$field;)*
        }
    }
    
    impl<T: Rem<Output=T>> Rem for $type<T> {
        type Output = Self;
        fn rem(self, rhs: Self) -> Self {
            $vec($(self.$field % rhs.$field,)*)
        }
    }
    
    impl<T: RemAssign> RemAssign for $type<T> {
        fn rem_assign(&mut self, rhs: Self) {
            $(self.$field %= rhs.$field;)*
        }
    }
    
    impl<T: Copy + Mul<Output=T>> Mul<T> for $type<T> {
        type Output = Self;
        fn mul(self, rhs: T) -> Self {
            $vec($(self.$field * rhs,)*)
        }
    }
    
    impl<T: Copy + MulAssign> MulAssign<T> for $type<T> {
        fn mul_assign(&mut self, rhs: T) {
            $(self.$field *= rhs;)*
        }
    }
    
    impl<T: Copy + Div<Output=T>> Div<T> for $type<T> {
        type Output = Self;
        fn div(self, rhs: T) -> Self {
            $vec($(self.$field / rhs,)*)
        }
    }
    
    impl<T: Copy + DivAssign> DivAssign<T> for $type<T> {
        fn div_assign(&mut self, rhs: T) {
            $(self.$field /= rhs;)*
        }
    }
    
    impl<T: Copy + Add<Output=T>> Add<T> for $type<T> {
        type Output = Self;
        fn add(self, rhs: T) -> Self {
            $vec($(self.$field + rhs,)*)
        }
    }
    
    impl<T: Copy + AddAssign> AddAssign<T> for $type<T> {
        fn add_assign(&mut self, rhs: T) {
            $(self.$field += rhs;)*
        }
    }
    
    impl<T: Copy + Sub<Output=T>> Sub<T> for $type<T> {
        type Output = Self;
        fn sub(self, rhs: T) -> Self {
            $vec($(self.$field - rhs,)*)
        }
    }
    
    impl<T: Copy + SubAssign> SubAssign<T> for $type<T> {
        fn sub_assign(&mut self, rhs: T) {
            $(self.$field -= rhs;)*
        }
    }
    
    impl<T: Copy + Rem<Output=T>> Rem<T> for $type<T> {
        type Output = Self;
        fn rem(self, rhs: T) -> Self {
            $vec($(self.$field % rhs,)*)
        }
    }
    
    impl<T: Copy + RemAssign> RemAssign<T> for $type<T> {
        fn rem_assign(&mut self, rhs: T) {
            $(self.$field %= rhs;)*
        }
    }
    
    impl<T: Neg> Neg for $type<T> {
        type Output = $type<<T as Neg>::Output>;
        fn neg(self) -> Self::Output {
            $vec($(-self.$field,)*)
        }
    }
    
    impl<T: Abs> Abs for $type<T> {
        fn abs(self) -> Self {
            $vec($(self.$field.abs(),)*)
        }
    }
    
    impl<T: Zero> Zero for $type<T> {
        fn zero() -> Self {
            $type { $($field: T::zero(), )* }
        }
    }
    
    impl<T: One> One for $type<T> {
        fn one() -> Self {
            $type { $($field: T::one(), )* }
        }
    }
    
    impl<T: One + Mul<Output=T>> Product for $type<T> {
        fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
            iter.fold(Self::one(), |a, b| a * b)
        }
    }
    
    impl<T: Zero + Add<Output=T>> Sum for $type<T> {
        fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }
    
    impl<T> Index<usize> for $type<T> {
        type Output = T;
        #[allow(unused_assignments)]
        fn index(&self, index: usize) -> &T {
            let mut i = 0;
            $(
                if i == index {
                    return &self.$field
                }
                i += 1;
            )*
            panic!("index out of bounds, index is {} but the len is {}",index,i)
        }
    }
    
    impl<T> IndexMut<usize> for $type<T> {
        #[allow(unused_assignments)]
        fn index_mut(&mut self, index: usize) -> &mut T {
            let mut i = 0;
            $(
                if i == index {
                    return &mut self.$field
                }
                i += 1;
            )*
            panic!("index out of bounds, index is {} but the len is {}",index,i)
        }
    }
}}
