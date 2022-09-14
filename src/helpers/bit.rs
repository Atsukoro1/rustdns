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
            is_zero
        }, 

        1 => {
            is_one
        },

        _ => {
            panic!("Not valid bit!");
        }
    }
}

/// Convert one unsigned 16 bit integer into two unsigned 8 bit integers
/// 
/// Author - https://stackoverflow.com/users/1021920/hellow
/// 
/// Returns an array with two 8-bit integers
pub fn convert_u16_to_two_u8s(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}