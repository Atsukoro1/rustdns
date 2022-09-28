use std::str::Chars;
use bitreader::BitReader;

fn resolve_pointer(reader: &mut BitReader, bytes: &[u8]) -> String {
    let offset = reader.read_u8(8).unwrap();
    
    // Create a new bitreader starting from the offset
    let mut off_reader = BitReader::new(&bytes[offset as usize + 3..bytes.len()]);

    let mut ending = false;
    let mut result = String::new();
 
    while !ending {
        let num = off_reader.read_u8(8).unwrap();
        println!("{}", num as char);

        if num == 0x0 {
            ending = true;
        } else {
            match std::str::from_utf8(&[num]) {
                Ok(ch) => {
                    let ch: char = ch.chars().nth(0).unwrap();
                    result.push(ch);
                }, 

                Err(..) => {
                    result.push('.');
                }
            }
        }
    }

    String::from(result)
}

fn resolve_no_pointer(reader: &mut BitReader, first_byte: u8, bytes: &[u8]) -> Option<String> {
    let mut final_res = String::new();

    for _ in 0..first_byte {
        let num = reader.read_u8(8).unwrap();

        if num == 0 {
            return None;
        }

        if num == 192 {
            for ch in resolve_pointer(reader, bytes).chars() {
                final_res.push(ch);
            }

        } else {
            match std::str::from_utf8(&[num]) {
                Ok(ch) => {
                    final_res.push(ch.chars().nth(0).unwrap());
                }, 

                Err(..) => {
                    final_res.push('.');
                }
            }
        }
    }

    Some(final_res)
}

pub fn resolve(res_str: &mut Vec<String>, reader: &mut BitReader, bytes: &[u8]) -> () {
    let first_byte: u8 = reader.read_u8(8).unwrap();

    // This is the end of the name
    if first_byte == 0x0 {
        return;
    };

    // Check if value has initial 2-bit pointer
    let form = format!("{:#010b}", first_byte);

    let mut bits: Chars = form.chars();
    bits.next();
    bits.next();

    if bits.next().unwrap() == '1' && bits.next().unwrap() == '1' {
        // Initial pointer
        res_str.push(resolve_pointer(reader, bytes));
    } else {
        // No pointer clear name
        match resolve_no_pointer(reader, first_byte, bytes) {
            Some(resolved_str) => {
                println!("{}", resolved_str);
                res_str.push(resolved_str);
            },

            None => {
                return;
            }
        }
    }

    resolve(res_str, reader, bytes)
}

/// Sometime names can be compressed with pointers and that's exactly
/// why this function is here. It checks for pointer in message and then 
/// use the correct function to resolve the name
pub fn resolve_name(reader: &mut BitReader, bytes: &[u8]) -> String {
    let mut final_name: Vec<String> = vec![];

    resolve(&mut final_name, reader, bytes);

    return final_name.join(".");
}