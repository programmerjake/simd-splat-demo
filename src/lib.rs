use simd_splat::{splat, I32x4};

pub fn test(lhs: &mut I32x4, rhs: i32) {
    *lhs += splat(rhs);
    *lhs += splat(5);
}
