/// A type that can be zeroed
pub trait Zeroable: Sized {
    /// A zeroed instance
    fn zeroed() -> Self;
}