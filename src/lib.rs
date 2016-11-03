//! This crate deals with physical units.
//! It's only a matter of time until other units get added, har har :)

/// Define one or more units.
macro_rules! defunit {
    ($name:ident( $($fields:ty),+ )) => {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
        pub struct $name($(pub $fields)+);
    };
    ($($names:ident( $($fields:ty),+ )),+) => {
        $(
            defunit! { $names( $($fields),+ ) }
        )+
    };
}

/// Define a conversion between 2 units.
macro_rules! conversion {
    (for $from:ident: $fromtype:ty => $totype:tt = $formula:expr) => {
        impl From<$fromtype> for $totype {
            fn from($from: $fromtype) -> Self { $totype($formula) }
        }
    };
}


/// Define addition for a unit.
macro_rules! defadd {
    (for $this:ident: $OUT:tt, $rhs:ident = $formula:expr) => {
        impl Add for $OUT {
            type Output = $OUT;
            fn add($this, $rhs: Self) -> Self { $OUT($formula) }
        }
    };
}

/// Define subtraction for a unit.
macro_rules! defsub {
    (for $this:ident: $OUT:tt, $rhs:ident = $formula:expr) => {
        impl Sub for $OUT {
            type Output = $OUT;
            fn sub($this, $rhs: Self) -> Self { $OUT($formula) }
        }
    }
}

/// Define multiplication for a unit.
macro_rules! defmul {
    (for $this:ident: $OUT:tt, $rhs:ident: $RHS:ty = $formula:expr) => {
        impl Mul<$RHS> for $OUT {
            type Output = $OUT;
            fn mul($this, $rhs: $RHS) -> Self { $OUT($formula) }
        }
    }
}

/// Define division for a unit.
macro_rules! defdiv {
    (for $this:ident: $OUT:tt, $rhs:ident: $RHS:ty = $formula:expr) => {
        impl Div<$RHS> for $OUT {
            type Output = $OUT;
            fn div($this, $rhs: $RHS) -> Self { $OUT($formula) }
        }
    }
}


pub mod time {
    use std::convert::From;
    use std::ops::{Add, Div, Sub, Mul};

    defunit! { Sec(u64), MilliSec(u64), MicroSec(u64), NanoSec(u64) }

    conversion! { for t: Sec => MilliSec = t.0 *         1_000 }
    conversion! { for t: Sec => MicroSec = t.0 *     1_000_000 }
    conversion! { for t: Sec => NanoSec  = t.0 * 1_000_000_000 }

    conversion! { for t: MilliSec =>      Sec = t.0 / 1_000     }
    conversion! { for t: MilliSec => MicroSec = t.0 * 1_000     }
    conversion! { for t: MilliSec =>  NanoSec = t.0 * 1_000_000 }

    conversion! { for t: MicroSec =>      Sec = t.0 / 1_000_000 }
    conversion! { for t: MicroSec => MilliSec = t.0 / 1_000     }
    conversion! { for t: MicroSec =>  NanoSec = t.0 * 1_000     }

    conversion! { for t: NanoSec =>      Sec = t.0 / 1_000_000_000 }
    conversion! { for t: NanoSec => MilliSec = t.0 /     1_000_000 }
    conversion! { for t: NanoSec => MicroSec = t.0 /         1_000 }


    defadd! { for self: Sec, rhs      = self.0 + rhs.0 }
    defsub! { for self: Sec, rhs      = self.0 - rhs.0 }
    defmul! { for self: Sec, rhs: u64 = self.0 * rhs   }
    defdiv! { for self: Sec, rhs: u64 = self.0 / rhs   }

    defadd! { for self: MilliSec, rhs      = self.0 + rhs.0 }
    defsub! { for self: MilliSec, rhs      = self.0 - rhs.0 }
    defmul! { for self: MilliSec, rhs: u64 = self.0 * rhs   }
    defdiv! { for self: MilliSec, rhs: u64 = self.0 / rhs   }

    defadd! { for self: MicroSec, rhs      = self.0 + rhs.0 }
    defsub! { for self: MicroSec, rhs      = self.0 - rhs.0 }
    defmul! { for self: MicroSec, rhs: u64 = self.0 * rhs   }
    defdiv! { for self: MicroSec, rhs: u64 = self.0 / rhs   }

    defadd! { for self: NanoSec, rhs      = self.0 + rhs.0 }
    defsub! { for self: NanoSec, rhs      = self.0 - rhs.0 }
    defmul! { for self: NanoSec, rhs: u64 = self.0 * rhs   }
    defdiv! { for self: NanoSec, rhs: u64 = self.0 / rhs   }

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
