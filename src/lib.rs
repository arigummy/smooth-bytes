//! Convenience bit operations for byte slices.
//!
//! `smooth-bytes` adds extension methods for checking, setting, clearing,
//! listing, and masking bits in existing `[u8]` buffers.
//!
//! It is meant for small byte-buffer tasks where using a full bit-vector
//! abstraction would be unnecessary.
//!
//! # Bit order
//!
//! Bit indexes are counted from the least significant bit of the last byte.
//!
//! In `[0x03, 0xe9]`, bit `0` is the lowest bit of `0xe9`, and bit `8` is the
//! lowest bit of `0x03`.
//!
//! # Example
//!
//! ```
//! use smooth_bytes::SmoothBytes;
//!
//! let mut bytes = [0x03, 0xe9];
//!
//! assert!(bytes.get_bit(8));
//! assert!(!bytes.get_bit(2));
//!
//! bytes.set_bit(2);
//! assert!(bytes.get_bit(2));
//!
//! bytes.reset_bit(3);
//! bytes.or_mask(0x0414u16);
//!
//! let set_bits = bytes.get_set_bits();
//! assert!(set_bits.contains(&2));
//! ```

pub trait SmoothBytes {
    fn get_bit(&self, bit: usize) -> bool;
    fn get_set_bits(&self) -> Vec<usize>;
    fn get_signs_bits(&self) -> Vec<u8>;
    fn set_bit(&mut self, bit: usize);
    fn reset_bit(&mut self, bit: usize);
    fn or_mask<T: Into<u128>>(&mut self, mask: T);
    fn and_mask<T: Into<u128>>(&mut self, mask: T);
    fn is_zero(&self) -> bool;
}

impl SmoothBytes for [u8] {
    fn get_bit(&self, bit: usize) -> bool {
        if bit >= self.len() * 8 {
            return false;
        }
        self[self.len() - 1 - bit / 8] >> (bit % 8) & 1 == 1
    }
    fn get_set_bits(&self) -> Vec<usize> {
        let mut res = vec![];
        for i in 0..self.len() * 8 {
            if self.get_bit(i) {
                res.push(i);
            }
        }
        res
    }

    fn get_signs_bits(&self) -> Vec<u8> {
        self.get_set_bits()
            .into_iter()
            .map(|bit| u8::try_from(bit).expect("bit index does not fit into u8"))
            .collect()
    }
    fn set_bit(&mut self, bit: usize) {
        if bit >= self.len() * 8 {
            return;
        }
        self[self.len() - 1 - bit / 8] |= 1u8 << (bit % 8);
    }
    fn reset_bit(&mut self, bit: usize) {
        if bit >= self.len() * 8 {
            return;
        }
        self[self.len() - 1 - bit / 8] &= !(1u8 << (bit % 8));
    }
    fn or_mask<T: Into<u128>>(&mut self, mask: T) {
        let mut mask = mask.into();
        for byte in self.iter_mut().rev() {
            *byte |= (mask & 0xff) as u8;
            mask >>= 8;
        }
    }

    fn and_mask<T: Into<u128>>(&mut self, mask: T) {
        let mut mask = mask.into();
        for byte in self.iter_mut().rev() {
            *byte &= (mask & 0xff) as u8;
            mask >>= 8;
        }
    }
    fn is_zero(&self) -> bool {
        self.iter().all(|&b| b == 0u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod get_bit {
        use super::*;
        #[test]
        fn test_get_bit_ar_1() {
            assert_eq!([3, 233].get_bit(2), false);
        }
        #[test]
        fn test_get_bit_ar_2() {
            assert_eq!([3, 233].get_bit(16), false)
        }
        #[test]
        fn test_get_bit_ar_3() {
            assert_eq!([3, 233].get_bit(20), false)
        }
        #[test]
        fn test_get_bit_ar_4() {
            assert_eq!([3, 233].get_bit(8), true)
        }
        #[test]
        fn test_get_bit_vec_1() {
            assert_eq!(vec![3, 233].get_bit(2), false)
        }
        #[test]
        fn test_get_bit_vec_2() {
            assert_eq!(vec![3, 233].get_bit(16), false)
        }
        #[test]
        fn test_get_bit_vec_3() {
            assert_eq!(vec![3, 233].get_bit(20), false)
        }
        #[test]
        fn test_get_bit_vec_4() {
            assert_eq!(vec![3, 233].get_bit(8), true)
        }
    }
    mod set_bit {
        use super::*;
        #[test]
        fn test_set_bit_ar_1() {
            let mut x = [3, 233];
            x.set_bit(2);
            assert_eq!(x.get_bit(2), true);
            assert_eq!(x[1], 237);
            assert_eq!(x, [3, 237]);
        }
        #[test]
        fn test_set_bit_ar_2() {
            let mut x = [3, 233];
            x.set_bit(16);
            assert_eq!(x.get_bit(16), false);
            assert_eq!(x[1], 233);
            assert_eq!(x, [3, 233]);
        }
        #[test]
        fn test_set_bit_ar_3() {
            let mut x = [3, 233];
            x.set_bit(3);
            assert_eq!(x.get_bit(3), true);
            assert_eq!(x[1], 233);
            assert_eq!(x, [3, 233]);
        }
        #[test]
        fn test_set_bit_ar_4() {
            let mut x = [3, 233];
            x.set_bit(10);
            assert_eq!(x.get_bit(10), true);
            assert_eq!(x[0], 7);
            assert_eq!(x, [7, 233]);
        }
        #[test]
        fn test_set_bit_vec_1() {
            let mut x = vec![3, 233];
            x.set_bit(2);
            assert_eq!(x.get_bit(2), true);
            assert_eq!(x[1], 237);
            assert_eq!(x, vec![3, 237]);
        }
        #[test]
        fn test_set_bit_vec_2() {
            let mut x = vec![3, 233];
            x.set_bit(16);
            assert_eq!(x.get_bit(16), false);
            assert_eq!(x[1], 233);
            assert_eq!(x, vec![3, 233]);
        }
        #[test]
        fn test_set_bit_vec_3() {
            let mut x = vec![3, 233];
            x.set_bit(3);
            assert_eq!(x.get_bit(3), true);
            assert_eq!(x[1], 233);
            assert_eq!(x, vec![3, 233]);
        }
        #[test]
        fn test_set_bit_vec_4() {
            let mut x = vec![3, 233];
            x.set_bit(10);
            assert_eq!(x.get_bit(10), true);
            assert_eq!(x[0], 7);
            assert_eq!(x, vec![7, 233]);
        }
    }
    mod tes_reset_bit {
        use super::*;
        #[test]
        fn test_reset_bit_ar_1() {
            let mut x = [3, 233];
            x.reset_bit(3);
            assert_eq!(x.get_bit(3), false);
            assert_eq!(x[1], 225);
            assert_eq!(x, [3, 225]);
        }
        #[test]
        fn test_reset_bit_ar_2() {
            let mut x = [3, 233];
            x.reset_bit(16);
            assert_eq!(x.get_bit(16), false);
            assert_eq!(x[1], 233);
            assert_eq!(x, [3, 233]);
        }
        #[test]
        fn test_reset_bit_ar_3() {
            let mut x = [3, 233];
            x.reset_bit(2);
            assert_eq!(x.get_bit(2), false);
            assert_eq!(x[1], 233);
            assert_eq!(x, [3, 233]);
        }
        #[test]
        fn test_reset_bit_ar_4() {
            let mut x = [3, 233];
            x.reset_bit(9);
            assert_eq!(x.get_bit(9), false);
            assert_eq!(x[0], 1);
            assert_eq!(x, [1, 233]);
        }
        #[test]
        fn test_reset_bit_vec_1() {
            let mut x = vec![3, 233];
            x.reset_bit(3);
            assert_eq!(x.get_bit(3), false);
            assert_eq!(x[1], 225);
            assert_eq!(x, vec![3, 225]);
        }
        #[test]
        fn test_reset_bit_vec_2() {
            let mut x = vec![3, 233];
            x.reset_bit(16);
            assert_eq!(x.get_bit(16), false);
            assert_eq!(x[1], 233);
            assert_eq!(x, vec![3, 233]);
        }
        #[test]
        fn test_reset_bit_vec_3() {
            let mut x = vec![3, 233];
            x.reset_bit(2);
            assert_eq!(x.get_bit(2), false);
            assert_eq!(x[1], 233);
            assert_eq!(x, vec![3, 233]);
        }
        #[test]
        fn test_reset_bit_vec_4() {
            let mut x = vec![3, 233];
            x.reset_bit(9);
            assert_eq!(x.get_bit(9), false);
            assert_eq!(x[0], 1);
            assert_eq!(x, vec![1, 233]);
        }
    }

    mod get_set_bits {
        use super::*;

        #[test]
        fn test_get_set_bits() {
            assert_eq!([3, 233].get_set_bits(), vec![0, 3, 5, 6, 7, 8, 9]);
        }

        #[test]
        fn test_get_signs_bits_alias() {
            assert_eq!([3, 233].get_signs_bits(), vec![0, 3, 5, 6, 7, 8, 9]);
        }
    }

    mod masks {
        use super::*;

        #[test]
        fn test_or_mask() {
            let mut x = [0x03, 0xe9];
            x.or_mask(0x0414u16);
            assert_eq!(x, [0x07, 0xfd]);
        }

        #[test]
        fn test_and_mask() {
            let mut x = [0x03, 0xe9];
            x.and_mask(0x01f0u16);
            assert_eq!(x, [0x01, 0xe0]);
        }

        #[test]
        fn test_and_mask_clears_bytes_outside_mask() {
            let mut x = [0xff, 0xff, 0xff];
            x.and_mask(0x00ffu16);
            assert_eq!(x, [0x00, 0x00, 0xff]);
        }

        #[test]
        fn test_is_zero() {
            assert!([0, 0].is_zero());
            assert!(![0, 1].is_zero());
        }
    }
}
