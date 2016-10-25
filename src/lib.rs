macro_rules! conversion {
    (for $from:ident, $fromtype:ty => $totype:ty = $formula:expr) => {
        impl From<$fromtype> for $totype {
            fn from($from: $fromtype) -> Self { $formula }
        }
    }
}

// /// Generate an operation.
// macro_rules! gen_op {
//     (for $this:ident, $rhs:ident as $out_type:ty :
//      $op_name:ident<$op_type:ty> = $result:expr) => {
//         impl $op_type for $out_type {
//             type Output = $out_type;
//             fn $op_name($this, $rhs: Self) -> Self { $result }
//         }
//     }
// }


macro_rules! gen_add {
    (for $this:ident, $rhs:ident as $OUT:ty = $formula:expr) => {
        impl Add for $OUT {
            type Output = $OUT;
            fn add($this, $rhs: Self) -> Self { $formula }
        }
    }
}

macro_rules! gen_sub {
    (for $this:ident, $rhs:ident as $OUT:ty = $formula:expr) => {
        impl Sub for $OUT {
            type Output = $OUT;
            fn sub($this, $rhs: Self) -> Self { $formula }
        }
    }
}

macro_rules! gen_mul {
    (for $this:ident, $rhs:ident: $RHS:ty as $OUT:ty = $formula:expr) => {
        impl Mul<$RHS> for $OUT {
            type Output = $OUT;
            fn mul($this, $rhs: $RHS) -> Self { $formula }
        }
    }
}

macro_rules! gen_div {
    (for $this:ident, $rhs:ident: $RHS:ty as $OUT:ty = $formula:expr) => {
        impl Div<$RHS> for $OUT {
            type Output = $OUT;
            fn div($this, $rhs: $RHS) -> Self { $formula }
        }
    }
}


pub mod time {
    use std::convert::From;
    use std::ops::{Add, Div, Sub, Mul};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
    pub struct      Sec(pub u64);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
    pub struct MilliSec(pub u64);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
    pub struct MicroSec(pub u64);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
    pub struct  NanoSec(pub u64);

    conversion! { for t,      Sec => NanoSec = NanoSec(t.0 * 1_000_000_000) }
    conversion! { for t, MilliSec => NanoSec = NanoSec(t.0 * 1_000_000) }
    conversion! { for t, MicroSec => NanoSec = NanoSec(t.0 * 1_000) }

    conversion! { for t,      Sec => MicroSec = MicroSec(t.0 * 1_000_000) }
    conversion! { for t, MilliSec => MicroSec = MicroSec(t.0 * 1_000) }
    conversion! { for t,  NanoSec => MicroSec = MicroSec(t.0 / 1_000) }

    conversion! { for t,      Sec => MilliSec = MilliSec(t.0 * 1_000) }
    conversion! { for t, MicroSec => MilliSec = MilliSec(t.0 / 1_000) }
    conversion! { for t,  NanoSec => MilliSec = MilliSec(t.0 / 1_000_000) }

    conversion! { for t, MilliSec => Sec = Sec(t.0 / 1_000) }
    conversion! { for t, MicroSec => Sec = Sec(t.0 / 1_000_000) }
    conversion! { for t,  NanoSec => Sec = Sec(t.0 / 1_000_000_000) }


    gen_add! { for self, rhs      as Sec = Sec(self.0 + rhs.0) }
    gen_sub! { for self, rhs      as Sec = Sec(self.0 - rhs.0) }
    gen_mul! { for self, rhs: u64 as Sec = Sec(self.0 * rhs) }
    gen_div! { for self, rhs: u64 as Sec = Sec(self.0 / rhs) }

    gen_add! { for self, rhs      as MilliSec = MilliSec(self.0 + rhs.0) }
    gen_sub! { for self, rhs      as MilliSec = MilliSec(self.0 - rhs.0) }
    gen_mul! { for self, rhs: u64 as MilliSec = MilliSec(self.0 * rhs)   }
    gen_div! { for self, rhs: u64 as MilliSec = MilliSec(self.0 / rhs)   }

    gen_add! { for self, rhs      as MicroSec = MicroSec(self.0 + rhs.0) }
    gen_sub! { for self, rhs      as MicroSec = MicroSec(self.0 - rhs.0) }
    gen_mul! { for self, rhs: u64 as MicroSec = MicroSec(self.0 * rhs)   }
    gen_div! { for self, rhs: u64 as MicroSec = MicroSec(self.0 / rhs)   }

    gen_add! { for self, rhs      as NanoSec = NanoSec(self.0 + rhs.0) }
    gen_sub! { for self, rhs      as NanoSec = NanoSec(self.0 - rhs.0) }
    gen_mul! { for self, rhs: u64 as NanoSec = NanoSec(self.0 * rhs)   }
    gen_div! { for self, rhs: u64 as NanoSec = NanoSec(self.0 / rhs)   }

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
    fn mul_seconds() {
        let t0 = Sec(10);
        assert_eq!(Sec(30), t0 * 3);
    }

    #[test]
    fn div_seconds() {
        let t0 = Sec(30);
        assert_eq!(Sec(10), t0 / 3);
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
