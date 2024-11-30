use crate::zeroable::Zeroable;

/// Errors for converting byte arrays
/// to [`Pod`] type instance
#[derive(Debug, PartialEq)]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u8::from_le_bytes([buffer[0]]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u8::from_be_bytes([buffer[0]]))
    }

    #[inline]
    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        buffer[0] = u8::to_le_bytes(*self)[0];
        Ok(1)
    }

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u16::from_le_bytes([buffer[0], buffer[1]]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u16::from_be_bytes([buffer[0], buffer[1]]))
    }

    #[inline]
    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = u16::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];

        Ok(2)
    }

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u32::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3]
        ]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u32::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3]
        ]))
    }

    #[inline]
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

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u64::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(u64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i8::from_le_bytes([buffer[0]]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i8::from_be_bytes([buffer[0]]))
    }

    #[inline]
    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        buffer[0] = i8::to_le_bytes(*self)[0];
        Ok(1)
    }

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i16::from_le_bytes([buffer[0], buffer[1]]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i16::from_be_bytes([buffer[0], buffer[1]]))
    }

    #[inline]
    fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        let bytes = i16::to_le_bytes(*self);

        buffer[0] = bytes[0];
        buffer[1] = bytes[1];

        Ok(2)
    }

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i32::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3]
        ]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i32::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3]
        ]))
    }

    #[inline]
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

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i64::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(i64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(f32::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
        ]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(f32::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
        ]))
    }

    #[inline]
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

    #[inline]
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

    #[inline]
    fn from_le_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(f64::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
        ]))
    }

    #[inline]
    fn from_be_bytes(buffer: &[u8]) -> Result<Self, PodError> {
        if buffer.len() < Self::SIZE {
            return Err(PodError::NotEnoughSpace);
        }

        Ok(f64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
        ]))
    }

    #[inline]
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

    #[inline]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pod_bool() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [0u8; <bool as Pod>::SIZE * 2];
            buffer_read[<bool as Pod>::SIZE] = 1u8;
            buffer_read
        };
        
        assert_eq!(bool::from_le_bytes(&buffer_read), Ok(false));
        assert_eq!(bool::from_be_bytes(&buffer_read), Ok(false));
        assert_eq!(bool::from_le_bytes(&buffer_read[<bool as Pod>::SIZE..]), Ok(true));
        assert_eq!(bool::from_be_bytes(&buffer_read[<bool as Pod>::SIZE..]), Ok(true));

        // Read fail
        let buffer_read_fail1 = [0u8; <bool as Pod>::SIZE - 1];
        let buffer_read_fail2: [u8; 1] = [3u8];
        
        assert_eq!(bool::from_le_bytes(&buffer_read_fail1), Err(PodError::NotEnoughSpace));
        assert_eq!(bool::from_be_bytes(&buffer_read_fail1), Err(PodError::NotEnoughSpace));
        assert_eq!(bool::from_be_bytes(&buffer_read_fail2), Err(PodError::NotInRange));
        assert_eq!(bool::from_be_bytes(&buffer_read_fail2), Err(PodError::NotInRange));

        // Write success
        let mut buffer_write1 = [0u8; <bool as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <bool as Pod>::SIZE * 2];
        
        assert_eq!(
            true.to_le_bytes(&mut buffer_write1), 
            Ok(<bool as Pod>::SIZE)
        );
        assert_eq!(
            true.to_be_bytes(&mut buffer_write1[<bool as Pod>::SIZE..]), 
            Ok(<bool as Pod>::SIZE)
        );
        assert_eq!(
            false.to_le_bytes(&mut buffer_write2), 
            Ok(<bool as Pod>::SIZE)
        );
        assert_eq!(
            false.to_be_bytes(&mut buffer_write2[<bool as Pod>::SIZE..]), 
            Ok(<bool as Pod>::SIZE)
        );
        
        assert_eq!(&buffer_write1[0..<bool as Pod>::SIZE], &[1]);
        assert_eq!(&buffer_write1[<bool as Pod>::SIZE..], &[1]);
        assert_eq!(&buffer_write2[0..<bool as Pod>::SIZE], &[0]);
        assert_eq!(&buffer_write2[<bool as Pod>::SIZE..], &[0]);

        // Write fail
        let mut buffer_write_fail = [0u8; <bool as Pod>::SIZE - 1];

        assert_eq!(true.to_le_bytes(&mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(true.to_be_bytes(&mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_u8() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [0u8; <u8 as Pod>::SIZE * 2];
            buffer_read[<u8 as Pod>::SIZE] = 1u8;
            buffer_read
        };
        
        assert_eq!(<u8 as Pod>::from_le_bytes(&buffer_read), Ok(0u8));
        assert_eq!(<u8 as Pod>::from_be_bytes(&buffer_read), Ok(0u8));
        assert_eq!(<u8 as Pod>::from_le_bytes(&buffer_read[<u8 as Pod>::SIZE..]), Ok(1u8));
        assert_eq!(<u8 as Pod>::from_be_bytes(&buffer_read[<u8 as Pod>::SIZE..]), Ok(1u8));

        // Read fail
        let buffer_read_fail = [0u8; <u8 as Pod>::SIZE - 1];
        
        assert_eq!(<u8 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u8 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <u8 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <u8 as Pod>::SIZE * 2];
        
        assert_eq!(
            <u8 as Pod>::to_le_bytes(&1, &mut buffer_write1), 
            Ok(<u8 as Pod>::SIZE)
        );
        assert_eq!(
            <u8 as Pod>::to_be_bytes(&1, &mut buffer_write1[<u8 as Pod>::SIZE..]), 
            Ok(<u8 as Pod>::SIZE)
        );
        assert_eq!(
            <u8 as Pod>::to_le_bytes(&0, &mut buffer_write2), 
            Ok(<u8 as Pod>::SIZE)
        );
        assert_eq!(
            <u8 as Pod>::to_be_bytes(&0, &mut buffer_write2[<u8 as Pod>::SIZE..]), 
            Ok(<u8 as Pod>::SIZE)
        );
        
        assert_eq!(&buffer_write1[0..<u8 as Pod>::SIZE], &[1]);
        assert_eq!(&buffer_write1[<u8 as Pod>::SIZE..], &[1]);
        assert_eq!(&buffer_write2[0..<u8 as Pod>::SIZE], &[0]);
        assert_eq!(&buffer_write2[<u8 as Pod>::SIZE..], &[0]);

        // Write fail
        let mut buffer_write_fail= [0u8; <u8 as Pod>::SIZE - 1];

        assert_eq!(<u8 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u8 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_u16() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [0u8; <u16 as Pod>::SIZE * 2];
            buffer_read[<u16 as Pod>::SIZE - 1] = 1u8;
            buffer_read[<u16 as Pod>::SIZE] = 1u8;
            buffer_read
        };
        
        assert_eq!(<u16 as Pod>::from_le_bytes(&buffer_read), Ok(256u16));
        assert_eq!(<u16 as Pod>::from_be_bytes(&buffer_read), Ok(1u16));
        assert_eq!(<u16 as Pod>::from_le_bytes(&buffer_read[<u16 as Pod>::SIZE..]), Ok(1u16));
        assert_eq!(<u16 as Pod>::from_be_bytes(&buffer_read[<u16 as Pod>::SIZE..]), Ok(256u16));

        // Read fail
        let buffer_read_fail = [0u8; <u16 as Pod>::SIZE - 1];
        
        assert_eq!(<u16 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u16 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <u16 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <u16 as Pod>::SIZE * 2];
        
        assert_eq!(
            <u16 as Pod>::to_le_bytes(&256, &mut buffer_write1), 
            Ok(<u16 as Pod>::SIZE)
        );
        assert_eq!(
            <u16 as Pod>::to_be_bytes(&1, &mut buffer_write1[<u16 as Pod>::SIZE..]), 
            Ok(<u16 as Pod>::SIZE)
        );
        assert_eq!(
            <u16 as Pod>::to_le_bytes(&1, &mut buffer_write2), 
            Ok(<u16 as Pod>::SIZE)
        );
        assert_eq!(
            <u16 as Pod>::to_be_bytes(&256, &mut buffer_write2[<u16 as Pod>::SIZE..]), 
            Ok(<u16 as Pod>::SIZE)
        );

        assert_eq!(&buffer_write1[0..<u16 as Pod>::SIZE], &buffer_read[0..<u16 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<u16 as Pod>::SIZE..], &buffer_read[0..<u16 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<u16 as Pod>::SIZE], &buffer_read[<u16 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<u16 as Pod>::SIZE..], &buffer_read[<u16 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail= [0u8; <u16 as Pod>::SIZE - 1];

        assert_eq!(<u16 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u16 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_u32() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [0u8; <u32 as Pod>::SIZE * 2];
            buffer_read[<u32 as Pod>::SIZE - 1] = 1u8;
            buffer_read[<u32 as Pod>::SIZE] = 1u8;
            buffer_read
        };
        
        assert_eq!(<u32 as Pod>::from_le_bytes(&buffer_read), Ok(16777216u32));
        assert_eq!(<u32 as Pod>::from_be_bytes(&buffer_read), Ok(1u32));
        assert_eq!(<u32 as Pod>::from_le_bytes(&buffer_read[<u32 as Pod>::SIZE..]), Ok(1u32));
        assert_eq!(<u32 as Pod>::from_be_bytes(&buffer_read[<u32 as Pod>::SIZE..]), Ok(16777216u32));

        // Read fail
        let buffer_read_fail = [0u8; <u32 as Pod>::SIZE - 1];
        
        assert_eq!(<u32 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u32 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <u32 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <u32 as Pod>::SIZE * 2];
        
        assert_eq!(
            <u32 as Pod>::to_le_bytes(&16777216, &mut buffer_write1), 
            Ok(<u32 as Pod>::SIZE)
        );
        assert_eq!(
            <u32 as Pod>::to_be_bytes(&1, &mut buffer_write1[<u32 as Pod>::SIZE..]), 
            Ok(<u32 as Pod>::SIZE)
        );
        assert_eq!(
            <u32 as Pod>::to_le_bytes(&1, &mut buffer_write2), 
            Ok(<u32 as Pod>::SIZE)
        );
        assert_eq!(
            <u32 as Pod>::to_be_bytes(&16777216, &mut buffer_write2[<u32 as Pod>::SIZE..]), 
            Ok(<u32 as Pod>::SIZE)
        );
        
        assert_eq!(&buffer_write1[0..<u32 as Pod>::SIZE], &buffer_read[0..<u32 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<u32 as Pod>::SIZE..], &buffer_read[0..<u32 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<u32 as Pod>::SIZE], &buffer_read[<u32 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<u32 as Pod>::SIZE..], &buffer_read[<u32 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <u32 as Pod>::SIZE - 1];

        assert_eq!(<u32 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u32 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_u64() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [0u8; <u64 as Pod>::SIZE * 2];
            buffer_read[<u64 as Pod>::SIZE - 1] = 1u8;
            buffer_read[<u64 as Pod>::SIZE] = 1u8;
            buffer_read
        };
        
        assert_eq!(<u64 as Pod>::from_le_bytes(&buffer_read), Ok(72057594037927936u64));
        assert_eq!(<u64 as Pod>::from_be_bytes(&buffer_read), Ok(1u64));
        assert_eq!(<u64 as Pod>::from_le_bytes(&buffer_read[<u64 as Pod>::SIZE..]), Ok(1u64));
        assert_eq!(<u64 as Pod>::from_be_bytes(&buffer_read[<u64 as Pod>::SIZE..]), Ok(72057594037927936u64));

        // Read fail
        let buffer_read_fail = [0u8; <u64 as Pod>::SIZE - 1];
        
        assert_eq!(<u64 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u64 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <u64 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <u64 as Pod>::SIZE * 2];
        
        assert_eq!(
            <u64 as Pod>::to_le_bytes(&72057594037927936, &mut buffer_write1), 
            Ok(<u64 as Pod>::SIZE)
        );
        assert_eq!(
            <u64 as Pod>::to_be_bytes(&1, &mut buffer_write1[8..]), 
            Ok(<u64 as Pod>::SIZE)
        );
        assert_eq!(
            <u64 as Pod>::to_le_bytes(&1, &mut buffer_write2), 
            Ok(<u64 as Pod>::SIZE)
        );
        assert_eq!(
            <u64 as Pod>::to_be_bytes(&72057594037927936, &mut buffer_write2[8..]), 
            Ok(<u64 as Pod>::SIZE)
        );
        
        assert_eq!(&buffer_write1[0..<u64 as Pod>::SIZE], &buffer_read[0..<u64 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<u64 as Pod>::SIZE..], &buffer_read[0..<u64 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<u64 as Pod>::SIZE], &buffer_read[<u64 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<u64 as Pod>::SIZE..], &buffer_read[<u64 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <u64 as Pod>::SIZE - 1];

        assert_eq!(<u64 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u64 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_u128() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [0u8; <u128 as Pod>::SIZE * 2];
            buffer_read[<u128 as Pod>::SIZE - 1] = 1u8;
            buffer_read[<u128 as Pod>::SIZE] = 1u8;
            buffer_read
        };
        
        assert_eq!(
            <u128 as Pod>::from_le_bytes(&buffer_read), 
            Ok(1329227995784915872903807060280344576u128)
        );
        assert_eq!(
            <u128 as Pod>::from_be_bytes(&buffer_read), 
            Ok(1u128)
        );
        assert_eq!(
            <u128 as Pod>::from_le_bytes(&buffer_read[<u128 as Pod>::SIZE..]), 
            Ok(1u128)
        );
        assert_eq!(
            <u128 as Pod>::from_be_bytes(&buffer_read[<u128 as Pod>::SIZE..]), 
            Ok(1329227995784915872903807060280344576u128)
        );

        // Read fail
        let buffer_read_fail = [0u8; <u128 as Pod>::SIZE - 1];
        
        assert_eq!(<u128 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u128 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <u128 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <u128 as Pod>::SIZE * 2];
        
        assert_eq!(
            <u128 as Pod>::to_le_bytes(&1329227995784915872903807060280344576, &mut buffer_write1), 
            Ok(<u128 as Pod>::SIZE)
        );
        assert_eq!(
            <u128 as Pod>::to_be_bytes(&1, &mut buffer_write1[<u128 as Pod>::SIZE..]), 
            Ok(<u128 as Pod>::SIZE)
        );
        assert_eq!(
            <u128 as Pod>::to_le_bytes(&1, &mut buffer_write2), 
            Ok(<u128 as Pod>::SIZE)
        );
        assert_eq!(
            <u128 as Pod>::to_be_bytes(&1329227995784915872903807060280344576, &mut buffer_write2[<u128 as Pod>::SIZE..]), 
            Ok(<u128 as Pod>::SIZE)
        );
        
        assert_eq!(&buffer_write1[0..<u128 as Pod>::SIZE], &buffer_read[0..<u128 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<u128 as Pod>::SIZE..], &buffer_read[0..<u128 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<u128 as Pod>::SIZE], &buffer_read[<u128 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<u128 as Pod>::SIZE..], &buffer_read[<u128 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <u128 as Pod>::SIZE - 1];

        assert_eq!(<u128 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<u128 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_i8() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [255u8; <i8 as Pod>::SIZE * 2];
            buffer_read[<i8 as Pod>::SIZE] = 1u8;
            buffer_read
        };
        
        assert_eq!(<i8 as Pod>::from_le_bytes(&buffer_read), Ok(-1i8));
        assert_eq!(<i8 as Pod>::from_be_bytes(&buffer_read), Ok(-1i8));
        assert_eq!(<i8 as Pod>::from_le_bytes(&buffer_read[<u8 as Pod>::SIZE..]), Ok(1i8));
        assert_eq!(<i8 as Pod>::from_be_bytes(&buffer_read[<u8 as Pod>::SIZE..]), Ok(1i8));

        // Read fail
        let buffer_read_fail = [0u8; <i8 as Pod>::SIZE - 1];
        
        assert_eq!(<i8 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i8 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <i8 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <i8 as Pod>::SIZE * 2];
        
        assert_eq!(
            <i8 as Pod>::to_le_bytes(&-1, &mut buffer_write1), 
            Ok(<i8 as Pod>::SIZE)
        );
        assert_eq!(
            <i8 as Pod>::to_be_bytes(&-1, &mut buffer_write1[<i8 as Pod>::SIZE..]), 
            Ok(<i8 as Pod>::SIZE)
        );
        assert_eq!(
            <i8 as Pod>::to_le_bytes(&1, &mut buffer_write2), 
            Ok(<i8 as Pod>::SIZE)
        );
        assert_eq!(
            <i8 as Pod>::to_be_bytes(&1, &mut buffer_write2[<i8 as Pod>::SIZE..]), 
            Ok(<i8 as Pod>::SIZE)
        );

        assert_eq!(&buffer_write1[0..<u8 as Pod>::SIZE], &[255]);
        assert_eq!(&buffer_write1[<u8 as Pod>::SIZE..], &[255]);
        assert_eq!(&buffer_write2[0..<u8 as Pod>::SIZE], &[1]);
        assert_eq!(&buffer_write2[<u8 as Pod>::SIZE..], &[1]);

        // Write fail
        let mut buffer_write_fail = [0u8; <i8 as Pod>::SIZE - 1];

        assert_eq!(<i8 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i8 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_i16() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [255u8; <i16 as Pod>::SIZE * 2];
            buffer_read[<i16 as Pod>::SIZE..].clone_from_slice(
                &[0u8; <i16 as Pod>::SIZE]
            );
            buffer_read[(<i16 as Pod>::SIZE * 2) - 1] = 1u8;
            buffer_read
        };
        
        assert_eq!(<i16 as Pod>::from_le_bytes(&buffer_read), Ok(-1i16));
        assert_eq!(<i16 as Pod>::from_be_bytes(&buffer_read), Ok(-1i16));
        assert_eq!(<i16 as Pod>::from_le_bytes(&buffer_read[<i16 as Pod>::SIZE..]), Ok(256i16));
        assert_eq!(<i16 as Pod>::from_be_bytes(&buffer_read[<i16 as Pod>::SIZE..]), Ok(1i16));

        // Read fail
        let buffer_read_fail = [0u8; <i16 as Pod>::SIZE - 1];
        
        assert_eq!(<i16 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i16 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <i16 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <i16 as Pod>::SIZE * 2];
        
        assert_eq!(
            <i16 as Pod>::to_le_bytes(&-1, &mut buffer_write1), 
            Ok(<i16 as Pod>::SIZE)
        );
        assert_eq!(
            <i16 as Pod>::to_be_bytes(&-1, &mut buffer_write1[<i16 as Pod>::SIZE..]), 
            Ok(<i16 as Pod>::SIZE)
        );
        assert_eq!(
            <i16 as Pod>::to_le_bytes(&256, &mut buffer_write2), 
            Ok(<i16 as Pod>::SIZE)
        );
        assert_eq!(
            <i16 as Pod>::to_be_bytes(&1, &mut buffer_write2[<i16 as Pod>::SIZE..]),
            Ok(<i16 as Pod>::SIZE)
        );

        assert_eq!(&buffer_write1[0..<i16 as Pod>::SIZE], &buffer_read[0..<i16 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<i16 as Pod>::SIZE..], &buffer_read[0..<i16 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<i16 as Pod>::SIZE], &buffer_read[<i16 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<i16 as Pod>::SIZE..], &buffer_read[<i16 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <i16 as Pod>::SIZE - 1];

        assert_eq!(<i16 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i16 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_i32() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [255u8; <i32 as Pod>::SIZE * 2];
            buffer_read[<i32 as Pod>::SIZE..].clone_from_slice(
                &[0u8; <i32 as Pod>::SIZE]
            );
            buffer_read[(<i32 as Pod>::SIZE * 2) - 1] = 1u8;
            buffer_read
        };
        
        assert_eq!(<i32 as Pod>::from_le_bytes(&buffer_read), Ok(-1i32));
        assert_eq!(<i32 as Pod>::from_be_bytes(&buffer_read), Ok(-1i32));
        assert_eq!(<i32 as Pod>::from_le_bytes(&buffer_read[<i32 as Pod>::SIZE..]), Ok(16777216i32));
        assert_eq!(<i32 as Pod>::from_be_bytes(&buffer_read[<i32 as Pod>::SIZE..]), Ok(1i32));

        // Read fail
        let buffer_read_fail = [0u8; <i32 as Pod>::SIZE - 1];
        
        assert_eq!(<i32 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i32 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <i32 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <i32 as Pod>::SIZE * 2];
        
        assert_eq!(
            <i32 as Pod>::to_le_bytes(&-1, &mut buffer_write1), 
            Ok(<i32 as Pod>::SIZE)
        );
        assert_eq!(
            <i32 as Pod>::to_be_bytes(&-1, &mut buffer_write1[<i32 as Pod>::SIZE..]), 
            Ok(<i32 as Pod>::SIZE)
        );
        assert_eq!(
            <i32 as Pod>::to_le_bytes(&16777216, &mut buffer_write2), 
            Ok(<i32 as Pod>::SIZE)
        );
        assert_eq!(
            <i32 as Pod>::to_be_bytes(&1, &mut buffer_write2[<i32 as Pod>::SIZE..]), 
            Ok(<i32 as Pod>::SIZE)
        );

        assert_eq!(&buffer_write1[0..<i32 as Pod>::SIZE], &buffer_read[0..<i32 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<i32 as Pod>::SIZE..], &buffer_read[0..<i32 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<i32 as Pod>::SIZE], &buffer_read[<i32 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<i32 as Pod>::SIZE..], &buffer_read[<i32 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <i32 as Pod>::SIZE - 1];

        assert_eq!(<i32 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i32 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_i64() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [255u8; <i64 as Pod>::SIZE * 2];
            buffer_read[<i64 as Pod>::SIZE..].clone_from_slice(
                &[0u8; <i64 as Pod>::SIZE]
            );
            buffer_read[(<i64 as Pod>::SIZE * 2) - 1] = 1u8;
            buffer_read
        };
        
        assert_eq!(<i64 as Pod>::from_le_bytes(&buffer_read), Ok(-1i64));
        assert_eq!(<i64 as Pod>::from_be_bytes(&buffer_read), Ok(-1i64));
        assert_eq!(<i64 as Pod>::from_le_bytes(&buffer_read[<i64 as Pod>::SIZE..]), Ok(72057594037927936i64));
        assert_eq!(<i64 as Pod>::from_be_bytes(&buffer_read[<i64 as Pod>::SIZE..]), Ok(1i64));

        // Read fail
        let buffer_read_fail = [0u8; <i64 as Pod>::SIZE - 1];
        
        assert_eq!(<i64 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i64 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <i64 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <i64 as Pod>::SIZE * 2];
        
        assert_eq!(
            <i64 as Pod>::to_le_bytes(&-1, &mut buffer_write1), 
            Ok(<i64 as Pod>::SIZE)
        );
        assert_eq!(
            <i64 as Pod>::to_be_bytes(&-1, &mut buffer_write1[<i64 as Pod>::SIZE..]), 
            Ok(<i64 as Pod>::SIZE)
        );
        assert_eq!(
            <i64 as Pod>::to_le_bytes(&72057594037927936, &mut buffer_write2), 
            Ok(<i64 as Pod>::SIZE)
        );
        assert_eq!(
            <i64 as Pod>::to_be_bytes(&1, &mut buffer_write2[<i64 as Pod>::SIZE..]), 
            Ok(<i64 as Pod>::SIZE)
        );

        assert_eq!(&buffer_write1[0..<i64 as Pod>::SIZE], &buffer_read[0..<i64 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<i64 as Pod>::SIZE..], &buffer_read[0..<i64 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<i64 as Pod>::SIZE], &buffer_read[<i64 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<i64 as Pod>::SIZE..], &buffer_read[<i64 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <i64 as Pod>::SIZE - 1];

        assert_eq!(<i64 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i64 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_i128() {
        // Read success
        let buffer_read = {
            let mut buffer_read = [255u8; <i128 as Pod>::SIZE * 2];
            buffer_read[<i128 as Pod>::SIZE..].clone_from_slice(
                &[0u8; <i128 as Pod>::SIZE]
            );
            buffer_read[(<i128 as Pod>::SIZE * 2) - 1] = 1u8;
            buffer_read
        };
        
        assert_eq!(
            <i128 as Pod>::from_le_bytes(&buffer_read), 
            Ok(-1i128)
        );
        assert_eq!(
            <i128 as Pod>::from_be_bytes(&buffer_read), 
            Ok(-1i128)
        );
        assert_eq!(
            <i128 as Pod>::from_le_bytes(&buffer_read[<i128 as Pod>::SIZE..]), 
            Ok(1329227995784915872903807060280344576i128)
        );
        assert_eq!(
            <i128 as Pod>::from_be_bytes(&buffer_read[<i128 as Pod>::SIZE..]), 
            Ok(1i128)
        );

        // Read fail
        let buffer_read_fail = [0u8; <i128 as Pod>::SIZE - 1];
        
        assert_eq!(<i128 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i128 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <i128 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <i128 as Pod>::SIZE * 2];
        
        assert_eq!(
            <i128 as Pod>::to_le_bytes(&-1, &mut buffer_write1), 
            Ok(<i128 as Pod>::SIZE)
        );
        assert_eq!(
            <i128 as Pod>::to_be_bytes(&-1, &mut buffer_write1[<i128 as Pod>::SIZE..]), 
            Ok(<i128 as Pod>::SIZE)
        );
        assert_eq!(
            <i128 as Pod>::to_le_bytes(&1329227995784915872903807060280344576, &mut buffer_write2), 
            Ok(<i128 as Pod>::SIZE)
        );
        assert_eq!(
            <i128 as Pod>::to_be_bytes(&1, &mut buffer_write2[<i128 as Pod>::SIZE..]), 
            Ok(<i128 as Pod>::SIZE)
        );

        assert_eq!(&buffer_write1[0..<i128 as Pod>::SIZE], &buffer_read[0..<i128 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<i128 as Pod>::SIZE..], &buffer_read[0..<i128 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<i128 as Pod>::SIZE], &buffer_read[<i128 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<i128 as Pod>::SIZE..], &buffer_read[<i128 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <i128 as Pod>::SIZE - 1];

        assert_eq!(<i128 as Pod>::to_le_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<i128 as Pod>::to_be_bytes(&1, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_f32() {
        // Read success
        let buffer_read = [0u8, 0u8, 128u8, 191u8, 0u8, 0u8, 192u8, 63u8];
        
        assert_eq!(<f32 as Pod>::from_le_bytes(&buffer_read), Ok(-1f32));
        assert_eq!(<f32 as Pod>::from_be_bytes(&buffer_read), Ok(4.6185e-41f32));
        assert_eq!(<f32 as Pod>::from_le_bytes(&buffer_read[<f32 as Pod>::SIZE..]), Ok(1.5f32));
        assert_eq!(<f32 as Pod>::from_be_bytes(&buffer_read[<f32 as Pod>::SIZE..]), Ok(6.8965e-41f32));

        // Read fail
        let buffer_read_fail = [0u8; <f32 as Pod>::SIZE - 1];
        
        assert_eq!(<f32 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<f32 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <f32 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <f32 as Pod>::SIZE * 2];
        
        assert_eq!(
            <f32 as Pod>::to_le_bytes(&-1f32, &mut buffer_write1), 
            Ok(<f32 as Pod>::SIZE)
        );
        assert_eq!(
            <f32 as Pod>::to_be_bytes(&4.6185e-41f32, &mut buffer_write1[<f32 as Pod>::SIZE..]), 
            Ok(<f32 as Pod>::SIZE)
        );
        assert_eq!(
            <f32 as Pod>::to_le_bytes(&1.5f32, &mut buffer_write2), 
            Ok(<f32 as Pod>::SIZE)
        );
        assert_eq!(
            <f32 as Pod>::to_be_bytes(&6.8965e-41f32, &mut buffer_write2[<f32 as Pod>::SIZE..]), 
            Ok(<f32 as Pod>::SIZE)
        );

        assert_eq!(&buffer_write1[0..<f32 as Pod>::SIZE], &buffer_read[0..<f32 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<f32 as Pod>::SIZE..], &buffer_read[0..<f32 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<f32 as Pod>::SIZE], &buffer_read[<f32 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<f32 as Pod>::SIZE..], &buffer_read[<f32 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <f32 as Pod>::SIZE - 1];

        assert_eq!(<f32 as Pod>::to_le_bytes(&1f32, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<f32 as Pod>::to_be_bytes(&1f32, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }

    #[test]
    fn pod_f64() {
        // Read success
        let buffer_read = [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 191u8, 
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 248u8, 63u8
        ];
        
        assert_eq!(<f64 as Pod>::from_le_bytes(&buffer_read), Ok(-1f64));
        assert_eq!(<f64 as Pod>::from_be_bytes(&buffer_read), Ok(3.045e-319f64));
        assert_eq!(<f64 as Pod>::from_le_bytes(&buffer_read[<f64 as Pod>::SIZE..]), Ok(1.5f64));
        assert_eq!(<f64 as Pod>::from_be_bytes(&buffer_read[<f64 as Pod>::SIZE..]), Ok(3.13984e-319f64));

        // Read fail
        let buffer_read_fail = [0u8; <f64 as Pod>::SIZE - 1];
        
        assert_eq!(<f64 as Pod>::from_le_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<f64 as Pod>::from_be_bytes(&buffer_read_fail), Err(PodError::NotEnoughSpace));

        // Write success
        let mut buffer_write1 = [0u8; <f64 as Pod>::SIZE * 2];
        let mut buffer_write2 = [1u8; <f64 as Pod>::SIZE * 2];
        
        assert_eq!(
            <f64 as Pod>::to_le_bytes(&-1f64, &mut buffer_write1), 
            Ok(<f64 as Pod>::SIZE)
        );
        assert_eq!(
            <f64 as Pod>::to_be_bytes(&3.045e-319f64, &mut buffer_write1[<f64 as Pod>::SIZE..]), 
            Ok(<f64 as Pod>::SIZE)
        );
        assert_eq!(
            <f64 as Pod>::to_le_bytes(&1.5f64, &mut buffer_write2), 
            Ok(<f64 as Pod>::SIZE)
        );
        assert_eq!(
            <f64 as Pod>::to_be_bytes(&3.13984e-319f64, &mut buffer_write2[<f64 as Pod>::SIZE..]), 
            Ok(<f64 as Pod>::SIZE)
        );

        assert_eq!(&buffer_write1[0..<f64 as Pod>::SIZE], &buffer_read[0..<f64 as Pod>::SIZE]);
        assert_eq!(&buffer_write1[<f64 as Pod>::SIZE..], &buffer_read[0..<f64 as Pod>::SIZE]);
        assert_eq!(&buffer_write2[0..<f64 as Pod>::SIZE], &buffer_read[<f64 as Pod>::SIZE..]);
        assert_eq!(&buffer_write2[<f64 as Pod>::SIZE..], &buffer_read[<f64 as Pod>::SIZE..]);

        // Write fail
        let mut buffer_write_fail = [0u8; <f64 as Pod>::SIZE - 1];

        assert_eq!(<f64 as Pod>::to_le_bytes(&1f64, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
        assert_eq!(<f64 as Pod>::to_be_bytes(&1f64, &mut buffer_write_fail), Err(PodError::NotEnoughSpace));
    }
}