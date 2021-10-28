use hal::gpio::{gpiod, gpioe, EPin, Floating, Input};

pub trait PortState {
    type SP0;
    type SP1;
    type SP2;
    type SP3;
    type SP4;
    type SP5;
    type SP6;
    type SP7;
}

pub enum DefaultState {}

impl PortState for DefaultState {
    type SP0 = Input<Floating>;
    type SP1 = Input<Floating>;
    type SP2 = Input<Floating>;
    type SP3 = Input<Floating>;
    type SP4 = Input<Floating>;
    type SP5 = Input<Floating>;
    type SP6 = Input<Floating>;
    type SP7 = Input<Floating>;
}

pub struct Port<S: PortState = DefaultState> {
    pub p0: EPin<S::SP0>,
    pub p1: EPin<S::SP1>,
    pub p2: EPin<S::SP2>,
    pub p3: EPin<S::SP3>,
    pub p4: EPin<S::SP4>,
    pub p5: EPin<S::SP5>,
    pub p6: EPin<S::SP6>,
    pub p7: EPin<S::SP7>,
}

impl<S: PortState> Port<S> {
    pub fn config<T: PortState>(self) -> Port<T>
    where
        EPin<T::SP0>: From<EPin<S::SP0>>,
        EPin<T::SP1>: From<EPin<S::SP1>>,
        EPin<T::SP2>: From<EPin<S::SP2>>,
        EPin<T::SP3>: From<EPin<S::SP3>>,
        EPin<T::SP4>: From<EPin<S::SP4>>,
        EPin<T::SP5>: From<EPin<S::SP5>>,
        EPin<T::SP6>: From<EPin<S::SP6>>,
        EPin<T::SP7>: From<EPin<S::SP7>>,
    {
        Port {
            p0: self.p0.into(),
            p1: self.p1.into(),
            p2: self.p2.into(),
            p3: self.p3.into(),
            p4: self.p4.into(),
            p5: self.p5.into(),
            p6: self.p6.into(),
            p7: self.p7.into(),
        }
    }
}

pub trait PartsExt {
    fn ports(self) -> (Port, Port);
}

impl PartsExt for gpiod::Parts {
    fn ports(self) -> (Port, Port) {
        let low = Port {
            p0: self.pd0.erase(),
            p1: self.pd1.erase(),
            p2: self.pd2.erase(),
            p3: self.pd3.erase(),
            p4: self.pd4.erase(),
            p5: self.pd5.erase(),
            p6: self.pd6.erase(),
            p7: self.pd7.erase(),
        };

        let high = Port {
            p0: self.pd8.erase(),
            p1: self.pd9.erase(),
            p2: self.pd10.erase(),
            p3: self.pd11.erase(),
            p4: self.pd12.erase(),
            p5: self.pd13.erase(),
            p6: self.pd14.erase(),
            p7: self.pd15.erase(),
        };

        (low, high)
    }
}

impl PartsExt for gpioe::Parts {
    fn ports(self) -> (Port, Port) {
        let low = Port {
            p0: self.pe0.erase(),
            p1: self.pe1.erase(),
            p2: self.pe2.erase(),
            p3: self.pe3.erase(),
            p4: self.pe4.erase(),
            p5: self.pe5.erase(),
            p6: self.pe6.erase(),
            p7: self.pe7.erase(),
        };

        let high = Port {
            p0: self.pe8.erase(),
            p1: self.pe9.erase(),
            p2: self.pe10.erase(),
            p3: self.pe11.erase(),
            p4: self.pe12.erase(),
            p5: self.pe13.erase(),
            p6: self.pe14.erase(),
            p7: self.pe15.erase(),
        };

        (low, high)
    }
}
