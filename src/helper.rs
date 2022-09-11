use core::fmt::Debug;
use std::any::Any;
use bitreader::BitReader;

/// Return value based on if bit is zero or one
pub fn bit_assign<'a, T: Any + Debug>(
    is_zero: T, 
    is_one: T,
    reader: &mut BitReader
) -> T {
    match reader.read_u8(1).unwrap() {
        0 => {
            reader.skip(1).unwrap();
            is_zero
        }, 

        1 => {
            reader.skip(1).unwrap();
            is_one
        },

        _ => {
            panic!("Not valid bit!");
        }
    }
}