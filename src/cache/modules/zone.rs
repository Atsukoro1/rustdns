pub struct ZoneController;

pub trait ZoneT {
    fn new() -> ZoneController;
}

impl ZoneT for ZoneController {
    fn new() -> ZoneController {
        ZoneController 
    }
}