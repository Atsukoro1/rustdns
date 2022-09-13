/// Simple bit writer to create packets in raw byte format with
pub struct BitWriter {
    pub bytes: Box<[u8]>,

    /// Offset in bits
    pub offset: u128
}

pub trait Methods {
    fn new() -> BitWriter;

    /// Skip an arbitary number of bits
    fn skip(&mut self, num: u8);

    /// Set bit at current offset to specified value
    /// 
    /// Based on set_to function parameter
    /// true -> 1
    /// false -> 0
    fn write_one(&mut self, set_to: bool);

    /// Write eight bit unsigned integer into buffer
    fn write_u8(&mut self, set_to: u8); 

    /// Write eight bit unsigned integer into buffer
    fn write_u16(&mut self, set_to: u16);
}

impl Methods for BitWriter {
    fn new() -> BitWriter {
        BitWriter { 
            bytes: Box::new([]),
            offset: 0
        }
    }

    fn skip(&mut self, num: u8) {
        self.offset += num as u128;
    }

    fn write_one(&mut self, _set_to: bool) {
        self.offset += 1;
    }

    fn write_u8(&mut self, _set_to: u8) {
        
    }

    fn write_u16(&mut self, _set_to: u16) {
        
    }
}