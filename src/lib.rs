//! A small crate for pointer arithmetic
use std::convert::TryInto;
use std::ptr::NonNull;

/// Utility trait for calculating the offset from various pointer types, in bytes
pub trait ByteOffset {
    // Workaround raw pointers not retaining their *mut or *const-ness with Self
    type Output;
    /// # Safety
    /// See [`std::primitive::pointer::offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset)
    unsafe fn byteoffset<T: TryInto<isize>>(&self, offset: T) -> Result<Self::Output, T::Error>;
}

impl<U: ?Sized> ByteOffset for *mut U {
    type Output = *mut u8;
    unsafe fn byteoffset<T: TryInto<isize>>(&self, offset: T) -> Result<Self::Output, T::Error> {
        Ok(self.cast::<u8>().offset(offset.try_into()?))
    }
}

impl<U: ?Sized> ByteOffset for *const U {
    type Output = *const u8;
    unsafe fn byteoffset<T: TryInto<isize>>(&self, offset: T) -> Result<Self::Output, T::Error> {
        Ok(self.cast::<u8>().offset(offset.try_into()?))
    }
}

impl<U: ?Sized> ByteOffset for NonNull<U> {
    type Output = NonNull<u8>;
    unsafe fn byteoffset<T: TryInto<isize>>(&self, offset: T) -> Result<Self::Output, T::Error> {
        Ok(NonNull::new_unchecked(
            // Offset of a NonNull is also NonNull
            self.cast::<u8>().as_ptr().offset(offset.try_into()?),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    #[test]
    fn test_offset() {
        let v = vec![0u8, 1, 2, 3];
        let v0 = &v[0];
        let v1 = &v[1];
        let v2 = &v[2];
        let v3 = &v[3];

        // Increment
        let v0_addr = v0 as *const u8;
        let v1_addr = unsafe { v0_addr.byteoffset(1).unwrap() };
        assert!(ptr::eq(v1, v1_addr));

        // Decrement
        let v3_addr = v3 as *const u8;
        let v2_addr = unsafe { v3_addr.byteoffset(-1).unwrap() };
        assert!(ptr::eq(v2, v2_addr));
    }
}
