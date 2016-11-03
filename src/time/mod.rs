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




#[cfg(test)]
mod time_tests {
    use time::*;

    #[test]
    fn add_seconds() { assert_eq!(Sec(15), Sec(10) + Sec(5)); }
    #[test]
    fn sub_seconds() { assert_eq!(Sec(5), Sec(10) - Sec(5)); }
    #[test]
    fn mul_seconds() { assert_eq!(Sec(30), Sec(10) * 3); }
    #[test]
    fn div_seconds() { assert_eq!(Sec(10), Sec(30) / 3); }

    #[test]
    fn add_milliseconds() { assert_eq!(MilliSec(15), MilliSec(10) + MilliSec(5)); }
    #[test]
    fn sub_milliseconds() { assert_eq!(MilliSec(5), MilliSec(10) - MilliSec(5)); }
    #[test]
    fn mul_milliseconds() { assert_eq!(MilliSec(30), MilliSec(10) * 3); }
    #[test]
    fn div_milliseconds() { assert_eq!(MilliSec(10), MilliSec(30) / 3); }

    #[test]
    fn add_microseconds() { assert_eq!(MicroSec(15), MicroSec(10) + MicroSec(5)); }
    #[test]
    fn sub_microseconds() { assert_eq!(MicroSec(5), MicroSec(10) - MicroSec(5)); }
    #[test]
    fn mul_microseconds() { assert_eq!(MicroSec(30), MicroSec(10) * 3); }
    #[test]
    fn div_microseconds() { assert_eq!(MicroSec(10), MicroSec(30) / 3); }

    #[test]
    fn add_nanoseconds() { assert_eq!(NanoSec(15), NanoSec(10) + NanoSec(5)); }
    #[test]
    fn sub_nanoseconds() { assert_eq!(NanoSec(5), NanoSec(10) - NanoSec(5)); }
    #[test]
    fn mul_nanoseconds() { assert_eq!(NanoSec(30), NanoSec(10) * 3); }
    #[test]
    fn div_nanoseconds() { assert_eq!(NanoSec(10), NanoSec(30) / 3); }



    #[test]
    fn seconds_to_milliseconds() {
        let secs = Sec(300);
        assert_eq!(MilliSec(secs.0 * 1_000), secs.into());
    }

    #[test]
    fn seconds_to_microseconds() {
        let secs = Sec(300);
        assert_eq!(MicroSec(secs.0 * 1_000_000), secs.into());
    }

    #[test]
    fn seconds_to_nanoseconds() {
        let secs = Sec(300);
        assert_eq!(NanoSec(secs.0 * 1_000_000_000), secs.into());
    }



    #[test]
    fn milliseconds_to_seconds() {
        let millis = MilliSec(2_000);
        assert_eq!(Sec(millis.0 / 1_000), millis.into());
    }

    #[test]
    fn milliseconds_to_microseconds() {
        let millis = MilliSec(2_000);
        assert_eq!(MicroSec(millis.0 * 1_000), millis.into());
    }

    #[test]
    fn milliseconds_to_nanoseconds() {
        let millis = MilliSec(2_000);
        assert_eq!(NanoSec(millis.0 * 1_000_000), millis.into());
    }



    #[test]
    fn microseconds_to_seconds() {
        let micros = MicroSec(2_000_000);
        assert_eq!(Sec(micros.0 / 1_000_000), micros.into());
    }

    #[test]
    fn microseconds_to_milliseconds() {
        let micros = MicroSec(300);
        assert_eq!(MilliSec(micros.0 / 1_000), micros.into());
    }

    #[test]
    fn microseconds_to_nanoseconds() {
        let micros = MicroSec(2_000_000);
        assert_eq!(NanoSec(micros.0 * 1_000), micros.into());
    }



    #[test]
    fn nanoseconds_to_seconds() {
        let nanos = NanoSec(2_000_000_000);
        let secs: Sec = nanos.into();
        assert_eq!(Sec(nanos.0 / 1_000_000_000), secs);
    }

    #[test]
    fn nanoseconds_to_milliseconds() {
        let nanos = NanoSec(300);
        let millis: MilliSec = nanos.into();
        assert_eq!(MilliSec(nanos.0 / 1_000_000), millis);
    }

    #[test]
    fn nanoseconds_to_microseconds() {
        let nanos = NanoSec(2_000_000_000);
        assert_eq!(MicroSec(nanos.0 / 1_000), nanos.into());
    }



}
