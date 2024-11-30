/// A type that can be zeroed
pub trait Zeroable: Sized {
    /// A zeroed instance
    fn zeroed() -> Self;
}

impl Zeroable for bool {
    fn zeroed() -> Self {
        false
    }
}

impl Zeroable for u8 {
    fn zeroed() -> Self {
        0u8
    }
}

impl Zeroable for u16 {
    fn zeroed() -> Self {
        0u16
    }
}

impl Zeroable for u32 {
    fn zeroed() -> Self {
        0u32
    }
}

impl Zeroable for u64 {
    fn zeroed() -> Self {
        0u64
    }
}

impl Zeroable for u128 {
    fn zeroed() -> Self {
        0u128
    }
}

impl Zeroable for usize {
    fn zeroed() -> Self {
        0usize
    }
}

impl Zeroable for i8 {
    fn zeroed() -> Self {
        0i8
    }
}

impl Zeroable for i16 {
    fn zeroed() -> Self {
        0i16
    }
}

impl Zeroable for i32 {
    fn zeroed() -> Self {
        0i32
    }
}

impl Zeroable for i64 {
    fn zeroed() -> Self {
        0i64
    }
}

impl Zeroable for i128 {
    fn zeroed() -> Self {
        0i128
    }
}

impl Zeroable for isize {
    fn zeroed() -> Self {
        0isize
    }
}

impl Zeroable for f32 {
    fn zeroed() -> Self {
        0f32
    }
}

impl Zeroable for f64 {
    fn zeroed() -> Self {
        0f64
    }
}

impl<T: Zeroable + Copy, const N: usize> Zeroable for [T; N] {
    fn zeroed() -> Self {
        [T::zeroed(); N]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroed_bool() {
        assert_eq!(bool::zeroed(), false);
    }

    #[test]
    fn zeroed_u8() {
        assert_eq!(u8::zeroed(), 0u8);
    }

    #[test]
    fn zeroed_u16() {
        assert_eq!(u16::zeroed(), 0u16);
    }

    #[test]
    fn zeroed_u32() {
        assert_eq!(u32::zeroed(), 0u32);
    }

    #[test]
    fn zeroed_u64() {
        assert_eq!(u64::zeroed(), 0u64);
    }

    #[test]
    fn zeroed_u128() {
        assert_eq!(u128::zeroed(), 0u128);
    }

    #[test]
    fn zeroed_usize() {
        assert_eq!(usize::zeroed(), 0usize);
    }

    #[test]
    fn zeroed_i8() {
        assert_eq!(i8::zeroed(), 0i8);
    }

    #[test]
    fn zeroed_i16() {
        assert_eq!(i16::zeroed(), 0i16);
    }

    #[test]
    fn zeroed_i32() {
        assert_eq!(i32::zeroed(), 0i32);
    }

    #[test]
    fn zeroed_i64() {
        assert_eq!(i64::zeroed(), 0i64);
    }

    #[test]
    fn zeroed_i128() {
        assert_eq!(i128::zeroed(), 0i128);
    }

    #[test]
    fn zeroed_isize() {
        assert_eq!(isize::zeroed(), 0isize);
    }

    #[test]
    fn zeroed_f32() {
        assert_eq!(f32::zeroed(), 0f32);
    }

    #[test]
    fn zeroed_f64() {
        assert_eq!(f64::zeroed(), 0f64);
    }

    #[test]
    fn zeroed_arrays() {
        assert_eq!(<[bool; 3]>::zeroed(), [false, false, false]);
        assert_eq!(<[u8; 1]>::zeroed(), [0u8]);
        assert_eq!(<[i8; 5]>::zeroed(), [0i8, 0i8, 0i8, 0i8, 0i8]);
        assert_eq!(<[f32; 2]>::zeroed(), [0f32, 0f32]);
        assert_eq!(<[f64; 10]>::zeroed(), [
            0f64, 0f64, 0f64,
            0f64, 0f64, 0f64,
            0f64, 0f64, 0f64,
            0f64
        ]);
    }
}