use hal::gpio::{
    gpiob::{PBn, PB0, PB1},
    Output, PushPull,
};

pub type LD1 = PB1<Output<PushPull>>;
pub type LD2 = PB0<Output<PushPull>>;

/// One of the on-board user LEDs
pub struct Led {
    pbx: PBn<Output<PushPull>>,
}

impl Led {
    pub fn on(&mut self) {
        self.pbx.set_high();
    }

    pub fn off(&mut self) {
        self.pbx.set_low();
    }

    pub fn toggl(&mut self) {
        self.pbx.toggle()
    }
}

impl From<LD1> for Led {
    fn from(pin: LD1) -> Self {
        Self {
            pbx: pin.erase_number(),
        }
    }
}

impl From<LD2> for Led {
    fn from(pin: LD2) -> Self {
        Self {
            pbx: pin.erase_number(),
        }
    }
}
