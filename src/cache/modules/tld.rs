pub struct TLDController;

pub trait TLDT {
    fn new() -> TLDController;
}

impl TLDT for TLDController {
    fn new() -> TLDController {
        TLDController 
    }
}