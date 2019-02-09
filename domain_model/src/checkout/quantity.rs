#[derive(Clone, Copy)]
pub struct Quantity {
    val: u8
}

impl Quantity {
    pub fn new(val: u8) -> Result<Self, ()> {
        if val < 1 {
            Err(())
        } else {
            Ok( Self { val } )
        }
    }

    pub fn val(&self) -> u8 {
        self.val
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Quantity) -> bool {
        self.val == other.val
    }
}

impl Eq for Quantity {
}
