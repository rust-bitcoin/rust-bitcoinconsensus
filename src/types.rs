// SPDX-License-Identifier: CC0-1.0

#![allow(non_camel_case_types)]

/// The C signed 32 bit integer type.
pub type c_int = i32;
/// The C signed 64 bit integer type.
pub type c_int64 = i64;
/// The C unsigned 8 bit integer type.
pub type c_uchar = u8;
/// The C unsigned 32 bit integer type.
pub type c_uint = u32;

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::os::raw;

    use crate::types;

    #[test]
    fn verify_types() {
        assert_eq!(TypeId::of::<types::c_int>(), TypeId::of::<raw::c_int>());
        assert_eq!(TypeId::of::<types::c_uchar>(), TypeId::of::<raw::c_uchar>());
        assert_eq!(TypeId::of::<types::c_uint>(), TypeId::of::<raw::c_uint>());
    }
}
