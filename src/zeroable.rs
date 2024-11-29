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
