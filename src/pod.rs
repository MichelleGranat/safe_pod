use crate::zeroable::Zeroable;

/// A type that can be created 
/// from and turned to a byte array
pub trait Pod: Zeroable {
    /// Size of byte representation
    const SIZE: usize;

    /// Create instance from little endian bytes
    fn from_le_bytes(buffer: &[u8]) -> Self;

    /// Create instance from big endian bytes
    fn from_be_bytes(buffer: &[u8]) -> Self;

    /// Create a little endian byte array from instance
    fn to_le_bytes(&self) -> &[u8];

    /// Create a big endian byte array from instance
    fn to_be_bytes(&self) -> &[u8];
}