use std::ops::AddAssign;

pub trait SplatTarget: Sized {
    type Item: Sized;
    fn splat(item: Self::Item) -> Self;
}

pub fn splat<T: SplatTarget>(item: T::Item) -> T {
    T::splat(item)
}

macro_rules! impl_vec {
    ($name:ident, $t:ty, $n:expr) => {
        pub struct $name(pub [$t; $n]);
        
        impl SplatTarget for $name {
            type Item = $t;
            fn splat(item: Self::Item) -> Self {
                $name([item; $n])
            }
        }
        
        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                for i in 0..$n {
                    self.0[i] += rhs.0[i];
                }
            }
        }
    };
}

impl_vec!(I32x4, i32, 4);
impl_vec!(I32x2, i32, 2);
impl_vec!(I64x4, i64, 4);
impl_vec!(I64x2, i64, 2);

