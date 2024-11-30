use crate::zeroable::Zeroable;

/// Errors for converting byte arrays
/// to [`Pod`] type instance
#[non_exhaustive]
pub enum PodError {
    /// Not enough bytes in buffer
    /// to construct type
    NotEnoughSpace,
    /// The bytes in the buffer are not in 
    /// the permitted range for the type
    NotInRange,
}

/// A type that can be created 
/// from and turned to a byte array
pub trait Pod: Zeroable {
    /// Size of byte representation
    const SIZE: usize;

    /// Create instance from little endian bytes
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError>;

    /// Create instance from big endian bytes
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError>;

    /// Create a little endian byte array from instance
    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError>;

    /// Create a big endian byte array from instance
    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError>;

    // TODO: add Vec<u8> support
}

impl Pod for bool {
    const SIZE: usize = 1;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        if buffer[0] == 0 {
            Ok(false)
        } else if buffer[0] == 1 {
            Ok(true)
        } else {
            Err(PodError::NotInRange)
        }
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        if buffer[0] == 0 {
            Ok(false)
        } else if buffer[0] == 1 {
            Ok(true)
        } else {
            Err(PodError::NotInRange)
        }
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        if *self {
            buffer[0] = u8::to_le_bytes(1)[0];
            Ok(1)
        } else {
            buffer[0] = u8::to_le_bytes(0)[0];
            Ok(1)
        }
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        if *self {
            buffer[0] = u8::to_be_bytes(1)[0];
            Ok(1)
        } else {
            buffer[0] = u8::to_be_bytes(0)[0];
            Ok(1)
        }
    }
}

impl Pod for u8 {
    const SIZE: usize = 1;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u8::from_le_bytes([buffer[0]]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u8::from_be_bytes([buffer[0]]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        buffer[0] = u8::to_le_bytes(*self)[0];
        Ok(1)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        buffer[0] = u8::to_be_bytes(*self)[0];
        Ok(1)
    }
}

impl Pod for u16 {
    const SIZE: usize = 2;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u16::from_le_bytes([buffer[0], buffer[1]]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u16::from_be_bytes([buffer[0], buffer[1]]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u16::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];

        Ok(2)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u16::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        
        Ok(2)
    }
}

impl Pod for u32 {
    const SIZE: usize = 4;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u32::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3]
        ]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u32::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3]
        ]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u32::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        
        Ok(4)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u32::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        
        Ok(4)
    }
}

impl Pod for u64 {
    const SIZE: usize = 8;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u64::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u64::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        
        Ok(8)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u64::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        
        Ok(8)
    }
}

impl Pod for u128 {
    const SIZE: usize = 16;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u128::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
            buffer[8], buffer[9], buffer[10], buffer[11],
            buffer[12], buffer[13], buffer[14], buffer[15],
        ]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u128::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
            buffer[8], buffer[9], buffer[10], buffer[11],
            buffer[12], buffer[13], buffer[14], buffer[15],
        ]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u128::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        buffer[8] = bytes[8];
        buffer[9] = bytes[9];
        buffer[10] = bytes[10];
        buffer[11] = bytes[11];
        buffer[12] = bytes[12];
        buffer[13] = bytes[13];
        buffer[14] = bytes[14];
        buffer[15] = bytes[15];
        
        Ok(16)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u128::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        buffer[8] = bytes[8];
        buffer[9] = bytes[9];
        buffer[10] = bytes[10];
        buffer[11] = bytes[11];
        buffer[12] = bytes[12];
        buffer[13] = bytes[13];
        buffer[14] = bytes[14];
        buffer[15] = bytes[15];
        
        Ok(16)
    }
}

impl Pod for i8 {
    const SIZE: usize = 1;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i8::from_le_bytes([buffer[0]]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i8::from_be_bytes([buffer[0]]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        buffer[0] = i8::to_le_bytes(*self)[0];
        Ok(1)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        buffer[0] = i8::to_be_bytes(*self)[0];
        Ok(1)
    }
}

impl Pod for i16 {
    const SIZE: usize = 2;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i16::from_le_bytes([buffer[0], buffer[1]]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i16::from_be_bytes([buffer[0], buffer[1]]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = i16::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];

        Ok(2)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }
        
        let bytes = i16::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];

        Ok(2)
    }
}

impl Pod for i32 {
    const SIZE: usize = 4;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i32::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3]
        ]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i32::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3]
        ]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = i32::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        
        Ok(4)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = i32::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        
        Ok(4)
    }
}

impl Pod for i64 {
    const SIZE: usize = 8;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i64::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = i64::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        
        Ok(8)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = i64::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        
        Ok(8)
    }
}

impl Pod for i128 {
    const SIZE: usize = 16;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i128::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
            buffer[8], buffer[9], buffer[10], buffer[11],
            buffer[12], buffer[13], buffer[14], buffer[15],
        ]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i128::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
            buffer[8], buffer[9], buffer[10], buffer[11],
            buffer[12], buffer[13], buffer[14], buffer[15],
        ]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = i128::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        buffer[8] = bytes[8];
        buffer[9] = bytes[9];
        buffer[10] = bytes[10];
        buffer[11] = bytes[11];
        buffer[12] = bytes[12];
        buffer[13] = bytes[13];
        buffer[14] = bytes[14];
        buffer[15] = bytes[15];
        
        Ok(16)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = i128::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        buffer[8] = bytes[8];
        buffer[9] = bytes[9];
        buffer[10] = bytes[10];
        buffer[11] = bytes[11];
        buffer[12] = bytes[12];
        buffer[13] = bytes[13];
        buffer[14] = bytes[14];
        buffer[15] = bytes[15];
        
        Ok(16)
    }
}

impl Pod for f32 {
    const SIZE: usize = 4;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(f32::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
        ]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(f32::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
        ]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = f32::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        
        Ok(4)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = f32::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        
        Ok(4)
    }
}

impl Pod for f64 {
    const SIZE: usize = 8;

    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(f64::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
        ]))
    }

    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(f64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
        ]))
    }

    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = f64::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        
        Ok(8)
    }

    fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = f64::to_be_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];
        buffer[2] = bytes[2];
        buffer[3] = bytes[3];
        buffer[4] = bytes[4];
        buffer[5] = bytes[5];
        buffer[6] = bytes[6];
        buffer[7] = bytes[7];
        
        Ok(8)
    }
}

// impl<T: Pod + Copy, const N: usize> Pod for [T; N] {
//     const SIZE: usize = N * T::SIZE;

//     fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
//         if buffer.len() < Self::SIZE {
//             return Err(PodError::NotEnoughSpace);
//         }

//         todo!()
//     }

//     fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
//         if buffer.len() < Self::SIZE {
//             return Err(PodError::NotEnoughSpace);
//         }

//         todo!()
//     }

//     fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
//         if buffer.len() < Self::SIZE {
//             return Err(PodError::NotEnoughSpace);
//         }

//         todo!()
//     }

//     fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
//         if buffer.len() < Self::SIZE {
//             return Err(PodError::NotEnoughSpace);
//         }

//         todo!()
//     }
// }