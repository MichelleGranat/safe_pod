use syn::Path;

/// A struct or Enum representation
#[non_exhaustive]
pub enum Repr {
    U8, U16, U32, U64, U128, Usize,
    I8, I16, I32, I64, I128, Isize,
    NotSupported
}

impl Repr {
    /// Get representation from a [`Path`]
    #[inline]
    pub fn from_path(path: Path) -> Self {
        if path.is_ident("u8") { return Self::U8 }
        if path.is_ident("u16") { return Self::U16 }
        if path.is_ident("u32") { return Self::U32 }
        if path.is_ident("u64") { return Self::U64 }
        if path.is_ident("u128") { return Self::U128 }
        if path.is_ident("usize") { return Self::Usize }
        if path.is_ident("i8") { return Self::I8 }
        if path.is_ident("i16") { return Self::I16 }
        if path.is_ident("i32") { return Self::I32 }
        if path.is_ident("i64") { return Self::I64 }
        if path.is_ident("i128") { return Self::I128 }
        if path.is_ident("isize") { return Self::Isize }

        Self::NotSupported
    }

    /// Returns true if representation is supported by Zeroable
    #[inline]
    pub fn is_zeroable(&self) -> bool {
        match self {
            Self::NotSupported => { false },
            _ => { true }
        }
    }

    /// Returns true if representation is supported by Pod
    #[inline]
    pub fn is_pod(&self) -> bool {
        match self {
            Self::Usize | Self::Isize | Self::NotSupported => { false },
            _ => { true }
        }
    }
}