macro_rules! gen_converter {
    (for $from:ident, $fromtype:ty => $totype:ty = $formula:expr) => {
        impl From<$fromtype> for $totype {
            fn from($from: $fromtype) -> Self { $formula }
        }
    }
}

macro_rules! gen_op_impl {
    (for $this:ident, $rhs:ident as $out_type:ty :
     $op_name:ident<$op_type:ty> = $result:expr) => {
        impl $op_type for $out_type {
            type Output = $out_type;
            fn $op_name($this, $rhs: Self) -> Self { $result }
        }
    }
}


pub mod time {
    use std::convert::From;
    use std::ops::{Add, Sub};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
    pub struct      Sec(pub u64);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
    pub struct MilliSec(pub u64);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
    pub struct MicroSec(pub u64);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
    pub struct  NanoSec(pub u64);

    gen_converter! { for t,      Sec => NanoSec = NanoSec(t.0 * 1_000_000_000) }
    gen_converter! { for t, MilliSec => NanoSec = NanoSec(t.0 * 1_000_000) }
    gen_converter! { for t, MicroSec => NanoSec = NanoSec(t.0 * 1_000) }

    gen_converter! { for t,      Sec => MicroSec = MicroSec(t.0 * 1_000_000) }
    gen_converter! { for t, MilliSec => MicroSec = MicroSec(t.0 * 1_000) }
    gen_converter! { for t,  NanoSec => MicroSec = MicroSec(t.0 / 1_000) }

    gen_converter! { for t,      Sec => MilliSec = MilliSec(t.0 * 1_000) }
    gen_converter! { for t, MicroSec => MilliSec = MilliSec(t.0 / 1_000) }
    gen_converter! { for t,  NanoSec => MilliSec = MilliSec(t.0 / 1_000_000) }

    gen_converter! { for t, MilliSec => Sec = Sec(t.0 / 1_000) }
    gen_converter! { for t, MicroSec => Sec = Sec(t.0 / 1_000_000) }
    gen_converter! { for t,  NanoSec => Sec = Sec(t.0 / 1_000_000_000) }


    gen_op_impl! { for self, rhs as Sec      : add<Add> =      Sec(self.0 + rhs.0) }
    gen_op_impl! { for self, rhs as MilliSec : add<Add> = MilliSec(self.0 + rhs.0) }
    gen_op_impl! { for self, rhs as MicroSec : add<Add> = MicroSec(self.0 + rhs.0) }
    gen_op_impl! { for self, rhs as  NanoSec : add<Add> =  NanoSec(self.0 + rhs.0) }

    gen_op_impl! { for self, rhs as Sec      : sub<Sub> =      Sec(self.0 - rhs.0) }
    gen_op_impl! { for self, rhs as MilliSec : sub<Sub> = MilliSec(self.0 - rhs.0) }
    gen_op_impl! { for self, rhs as MicroSec : sub<Sub> = MicroSec(self.0 - rhs.0) }
    gen_op_impl! { for self, rhs as  NanoSec : sub<Sub> =  NanoSec(self.0 - rhs.0) }

}

#[cfg(test)]
mod time_tests {
    use time::*;

    #[test]
    fn add_seconds() {
        let t0 = Sec(10);
        let t1 = Sec(5);
        assert_eq!(Sec(15), t0 + t1);
    }

    #[test]
    fn sub_seconds() {
        let t0 = Sec(10);
        let t1 = Sec(5);
        assert_eq!(Sec(5), t0 - t1);
    }

    #[test]
    fn seconds_to_nanoseconds() {
        let secs = Sec(300);
        let nanos: NanoSec = secs.into();
        assert_eq!(secs.0 * 1_000_000_000, nanos.0);
    }

    #[test]
    fn milliseconds_to_nanoseconds() {
        let millis = MilliSec(2_000);
        let nanos: NanoSec = millis.into();
        assert_eq!(millis.0 * 1_000_000, nanos.0);
    }

    #[test]
    fn microseconds_to_nanoseconds() {
        let micros = MicroSec(2_000_000);
        let nanos: NanoSec = micros.into();
        assert_eq!(micros.0 * 1_000, nanos.0);
    }


    #[test]
    fn seconds_to_milliseconds() {
        let secs = Sec(300);
        let millis: MilliSec = secs.into();
        assert_eq!(secs.0 * 1_000, millis.0);
    }

    #[test]
    fn microseconds_to_milliseconds() {
        let micros = MicroSec(300);
        let millis: MilliSec = micros.into();
        assert_eq!(micros.0 / 1_000, millis.0);
    }

    #[test]
    fn nanoseconds_to_milliseconds() {
        let nanos = NanoSec(300);
        let millis: MilliSec = nanos.into();
        assert_eq!(nanos.0 / 1_000_000, millis.0);
    }


    #[test]
    fn milliseconds_to_seconds() {
        let millis = MilliSec(2_000);
        let secs: Sec = millis.into();
        assert_eq!(millis.0 / 1_000, secs.0);
    }


    #[test]
    fn microseconds_to_seconds() {
        let micros = MicroSec(2_000_000);
        let secs: Sec = micros.into();
        assert_eq!(micros.0 / 1_000_000, secs.0);
    }

    #[test]
    fn nanoseconds_to_seconds() {
        let nanos = NanoSec(2_000_000_000);
        let secs: Sec = nanos.into();
        assert_eq!(secs.0, nanos.0 / 1_000_000_000);
    }


}
