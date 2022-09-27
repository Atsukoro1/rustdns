#[macro_export]
/// Assign given value based on if the provided unsigned integer 
/// is one or zero
/// 
/// ### Panics
/// - If the integer is not one or zero
macro_rules! bit_assign {
    ($is_zero:expr, $is_one:expr, $reader:expr) => {
        match $reader.read_u8(1).unwrap() {
            0 => {
                $is_zero
            }, 
    
            1 => {
                $is_one
            },
    
            _ => {
                panic!("Not valid bit!");
            }
        }
    };
}

/// ### Simple any-unsigned integer to 8-bit converting macro
/// 
/// This macro is made for converting unsigned 32 or 16 bit 
/// integers to array of unsigned 8-bit integers
/// 
/// Example
/// ```rust
/// let bytes: [u8; 2] = convert_u16_to_two_u8s!(100, u16);
/// // Use these bytes
/// bytes[0];
/// bytes[1];
/// ```
/// 
/// Returns (data type / 8) unsigned 8-bit integers
#[macro_export]
macro_rules! convert_u16_to_two_u8s {
    ($num:expr, u16)=>{
        [($num >> 8) as u8, $num as u8]
    };

    ($num:expr, u32)=>{
        [
            ($num >> 24) as u8,
            ($num >> 16) as u8,
            ($num >> 8) as u8,
            $num as u8
        ]
    }
}

/// Push variables to vec multiple times
/// Writes values straight into Vector instead of returning it
#[macro_export]
macro_rules! push_byte_vec {
    ($vec:expr, $repeat:expr, $item:expr) => {
        for _ in 0..$repeat {
            $vec.push($item);
        }
    };
}

/// Prepend slice to front of vector
pub fn prepend<T>(v: Vec<T>, s: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}