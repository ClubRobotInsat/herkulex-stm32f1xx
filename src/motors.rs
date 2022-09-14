use core::cell::RefCell;
use crate::communication::HerkulexCommunication;
use crate::motor::Motor;

/// This structure allows you to have multiple servomotors under the same communication
pub struct Motors<Comm: HerkulexCommunication> {
    communication: RefCell<Comm>,
}

impl<'a, Comm: HerkulexCommunication> Motors<Comm> {
    /// Create a new group of servomotors.
    pub fn new(comm: Comm) -> Motors<Comm> {
        Motors {
            communication: RefCell::new(comm),
        }
    }

    /// Create a new servomotor
    pub fn new_motor(&self, id: u8) -> Motor<Comm> {
        Motor::new(id, &self.communication)
    }
}
