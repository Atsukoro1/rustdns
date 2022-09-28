use std::str::Chars;
use bitreader::BitReader;

fn resolve_pointer(reader: &mut BitReader, bytes: &[u8]) -> String {
    let offset = reader.read_u8(8).unwrap();
    
    // Create a new bitreader starting from the offset
    let mut off_reader = BitReader::new(&bytes[offset as usize..bytes.len()]);

    let mut ending = false;
    let mut result = String::new();
 
    while !ending {
        let num = off_reader.read_u8(8).unwrap();

        if num == 0x0 {
            ending = true;
        } else {
            result.push(off_reader.read_u8(8).unwrap() as char);
        }
    }

    println!("{}", result);
    String::from(result)
}

fn resolve_no_pointer(reader: &mut BitReader, first_byte: u8) -> String {
    let mut final_res = String::new();

    for _ in 0..first_byte {
        final_res.push(
            reader.read_u8(8).unwrap() as char
        );
    }

    final_res
}

/// Sometime names can be compressed with pointers and that's exactly
/// why this function is here. It checks for pointer in message and then 
/// use the correct function to resolve the name
pub fn resolve_name(reader: &mut BitReader, bytes: &[u8]) -> String {
    let mut final_name: Vec<String> = vec![];

    let resolve = |res_str: &mut Vec<String>, reader: &mut BitReader| -> () {
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
            res_str.push(resolve_no_pointer(reader, first_byte));
        }
    };

    resolve(&mut final_name, reader);

    println!("{:?}", &final_name);

    return final_name.join(".");
}