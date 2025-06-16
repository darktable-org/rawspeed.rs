pub mod endianness {

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Endianness {
        Little,
        Big,
    }

    #[inline]
    #[cfg(target_endian = "little")]
    pub fn get_host_endianness() -> Endianness {
        Endianness::Little
    }

    #[inline]
    #[cfg(target_endian = "big")]
    pub fn get_host_endianness() -> Endianness {
        Endianness::Big
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum EndiannessDetectionError {
            Unknown,
        }

        pub fn get_host_endianness_runtime() -> Result<Endianness, EndiannessDetectionError> {
            match u16::from_ne_bytes([1, 0]) {
                1 => Ok(Endianness::Little),
                0 => Ok(Endianness::Big),
                _ => Err(EndiannessDetectionError::Unknown),
            }
        }

        #[test]
        fn compile_time_succeeds() {
            get_host_endianness();
        }

        #[test]
        fn run_time_succeeds() {
            get_host_endianness_runtime().unwrap();
        }

        #[test]
        fn basic() {
            assert_eq!(
                get_host_endianness(),
                get_host_endianness_runtime().expect("")
            );
        }
    }
}
