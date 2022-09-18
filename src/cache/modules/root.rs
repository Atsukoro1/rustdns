pub struct RootController;

pub trait RootT {
    fn new() -> RootController;
}

impl RootT for RootController {
    fn new() -> RootController {
        RootController
    }
}