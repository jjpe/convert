use std::convert::From;
use std::ops::{Add, Div, Sub, Mul};

defunit! {
    Day(f64), Hour(f64), Min(f64), Sec(f64),
    MilliSec(f64), MicroSec(f64), NanoSec(f64)
}


conversion! { for t: Day =>     Hour = t.0 *                 24.0 }
conversion! { for t: Day =>      Min = t.0 *              1_440.0 }
conversion! { for t: Day =>      Sec = t.0 *             86_400.0 }
conversion! { for t: Day => MilliSec = t.0 *         86_400_000.0 }
conversion! { for t: Day => MicroSec = t.0 *     86_400_000_000.0 }
conversion! { for t: Day =>  NanoSec = t.0 * 86_400_000_000_000.0 }

conversion! { for t: Hour =>      Day = t.0 / 24.0 }
conversion! { for t: Hour =>      Min = t.0 *                60.0 }
conversion! { for t: Hour =>      Sec = t.0 *             3_600.0 }
conversion! { for t: Hour => MilliSec = t.0 *         3_600_000.0 }
conversion! { for t: Hour => MicroSec = t.0 *     3_600_000_000.0 }
conversion! { for t: Hour =>  NanoSec = t.0 * 3_600_000_000_000.0 }

conversion! { for t: Min =>      Day = t.0 / 1_440.0 }
conversion! { for t: Min =>     Hour = t.0 /    60.0 }
conversion! { for t: Min =>      Sec = t.0 *             60.0 }
conversion! { for t: Min => MilliSec = t.0 *         60_000.0 }
conversion! { for t: Min => MicroSec = t.0 *     60_000_000.0 }
conversion! { for t: Min =>  NanoSec = t.0 * 60_000_000_000.0 }

conversion! { for t: Sec =>      Day = t.0 / 86_400.0 }
conversion! { for t: Sec =>     Hour = t.0 /  3_600.0 }
conversion! { for t: Sec =>      Min = t.0 /     60.0 }
conversion! { for t: Sec => MilliSec = t.0 *         1_000.0 }
conversion! { for t: Sec => MicroSec = t.0 *     1_000_000.0 }
conversion! { for t: Sec =>  NanoSec = t.0 * 1_000_000_000.0 }

conversion! { for t: MilliSec =>      Day = t.0 / 86_400_000.0 }
conversion! { for t: MilliSec =>     Hour = t.0 /  3_600_000.0 }
conversion! { for t: MilliSec =>      Min = t.0 /     60_000.0 }
conversion! { for t: MilliSec =>      Sec = t.0 / 1_000.0     }
conversion! { for t: MilliSec => MicroSec = t.0 * 1_000.0     }
conversion! { for t: MilliSec =>  NanoSec = t.0 * 1_000_000.0 }

conversion! { for t: MicroSec =>      Day = t.0 / 86_400_000_000.0 }
conversion! { for t: MicroSec =>     Hour = t.0 /  3_600_000_000.0 }
conversion! { for t: MicroSec =>      Min = t.0 /     60_000_000.0 }
conversion! { for t: MicroSec =>      Sec = t.0 / 1_000_000.0 }
conversion! { for t: MicroSec => MilliSec = t.0 / 1_000.0     }
conversion! { for t: MicroSec =>  NanoSec = t.0 * 1_000.0     }

conversion! { for t: NanoSec =>      Day = t.0 / 86_400_000_000_000.0 }
conversion! { for t: NanoSec =>     Hour = t.0 /  3_600_000_000_000.0 }
conversion! { for t: NanoSec =>      Min = t.0 /     60_000_000_000.0 }
conversion! { for t: NanoSec =>      Sec = t.0 / 1_000_000_000.0 }
conversion! { for t: NanoSec => MilliSec = t.0 /     1_000_000.0 }
conversion! { for t: NanoSec => MicroSec = t.0 /         1_000.0 }


defadd! { for self: Day, rhs      = self.0 + rhs.0 }
defsub! { for self: Day, rhs      = self.0 - rhs.0 }
defmul! { for self: Day, rhs: f64 = self.0 * rhs   }
defdiv! { for self: Day, rhs: f64 = self.0 / rhs   }

defadd! { for self: Hour, rhs      = self.0 + rhs.0 }
defsub! { for self: Hour, rhs      = self.0 - rhs.0 }
defmul! { for self: Hour, rhs: f64 = self.0 * rhs   }
defdiv! { for self: Hour, rhs: f64 = self.0 / rhs   }

defadd! { for self: Min, rhs      = self.0 + rhs.0 }
defsub! { for self: Min, rhs      = self.0 - rhs.0 }
defmul! { for self: Min, rhs: f64 = self.0 * rhs   }
defdiv! { for self: Min, rhs: f64 = self.0 / rhs   }

defadd! { for self: Sec, rhs      = self.0 + rhs.0 }
defsub! { for self: Sec, rhs      = self.0 - rhs.0 }
defmul! { for self: Sec, rhs: f64 = self.0 * rhs   }
defdiv! { for self: Sec, rhs: f64 = self.0 / rhs   }

defadd! { for self: MilliSec, rhs      = self.0 + rhs.0 }
defsub! { for self: MilliSec, rhs      = self.0 - rhs.0 }
defmul! { for self: MilliSec, rhs: f64 = self.0 * rhs   }
defdiv! { for self: MilliSec, rhs: f64 = self.0 / rhs   }

defadd! { for self: MicroSec, rhs      = self.0 + rhs.0 }
defsub! { for self: MicroSec, rhs      = self.0 - rhs.0 }
defmul! { for self: MicroSec, rhs: f64 = self.0 * rhs   }
defdiv! { for self: MicroSec, rhs: f64 = self.0 / rhs   }

defadd! { for self: NanoSec, rhs      = self.0 + rhs.0 }
defsub! { for self: NanoSec, rhs      = self.0 - rhs.0 }
defmul! { for self: NanoSec, rhs: f64 = self.0 * rhs   }
defdiv! { for self: NanoSec, rhs: f64 = self.0 / rhs   }




#[cfg(test)]
mod time_tests {
    use time::*;

    #[test]
    fn add_days() { assert_eq!(Day(15.0), Day(10.0) + Day(5.0)); }
    #[test]
    fn sub_days() { assert_eq!(Day(5.0), Day(10.0) - Day(5.0)); }
    #[test]
    fn mul_days() { assert_eq!(Day(30.0), Day(10.0) * 3.0); }
    #[test]
    fn div_days() { assert_eq!(Day(10.0), Day(30.0) / 3.0); }

    #[test]
    fn add_hours() { assert_eq!(Hour(15.0), Hour(10.0) + Hour(5.0)); }
    #[test]
    fn sub_hours() { assert_eq!(Hour(5.0), Hour(10.0) - Hour(5.0)); }
    #[test]
    fn mul_hours() { assert_eq!(Hour(30.0), Hour(10.0) * 3.0); }
    #[test]
    fn div_hours() { assert_eq!(Hour(10.0), Hour(30.0) / 3.0); }

    #[test]
    fn add_minutes() { assert_eq!(Min(15.0), Min(10.0) + Min(5.0)); }
    #[test]
    fn sub_minutes() { assert_eq!(Min(5.0), Min(10.0) - Min(5.0)); }
    #[test]
    fn mul_minutes() { assert_eq!(Min(30.0), Min(10.0) * 3.0); }
    #[test]
    fn div_minutes() { assert_eq!(Min(10.0), Min(30.0) / 3.0); }

    #[test]
    fn add_seconds() { assert_eq!(Sec(15.0), Sec(10.0) + Sec(5.0)); }
    #[test]
    fn sub_seconds() { assert_eq!(Sec(5.0), Sec(10.0) - Sec(5.0)); }
    #[test]
    fn mul_seconds() { assert_eq!(Sec(30.0), Sec(10.0) * 3.0); }
    #[test]
    fn div_seconds() { assert_eq!(Sec(10.0), Sec(30.0) / 3.0); }

    #[test]
    fn add_milliseconds() {
        assert_eq!(MilliSec(15.0), MilliSec(10.0) + MilliSec(5.0));
    }
    #[test]
    fn sub_milliseconds() {
        assert_eq!(MilliSec(5.0), MilliSec(10.0) - MilliSec(5.0));
    }
    #[test]
    fn mul_milliseconds() { assert_eq!(MilliSec(30.0), MilliSec(10.0) * 3.0); }
    #[test]
    fn div_milliseconds() { assert_eq!(MilliSec(10.0), MilliSec(30.0) / 3.0); }

    #[test]
    fn add_microseconds() {
        assert_eq!(MicroSec(15.0), MicroSec(10.0) + MicroSec(5.0)); }
    #[test]
    fn sub_microseconds() {
        assert_eq!(MicroSec(5.0), MicroSec(10.0) - MicroSec(5.0)); }
    #[test]
    fn mul_microseconds() { assert_eq!(MicroSec(30.0), MicroSec(10.0) * 3.0); }
    #[test]
    fn div_microseconds() { assert_eq!(MicroSec(10.0), MicroSec(30.0) / 3.0); }

    #[test]
    fn add_nanoseconds() { assert_eq!(NanoSec(15.0), NanoSec(10.0) + NanoSec(5.0)); }
    #[test]
    fn sub_nanoseconds() { assert_eq!(NanoSec(5.0), NanoSec(10.0) - NanoSec(5.0)); }
    #[test]
    fn mul_nanoseconds() { assert_eq!(NanoSec(30.0), NanoSec(10.0) * 3.0); }
    #[test]
    fn div_nanoseconds() { assert_eq!(NanoSec(10.0), NanoSec(30.0) / 3.0); }



    #[test]
    fn seconds_to_days() {
        let secs = Sec(300.0);
        assert_eq!(Day(secs.0 / 86_400.0), secs.into());
    }

    #[test]
    fn seconds_to_hours() {
        let secs = Sec(300.0);
        assert_eq!(Hour(secs.0 / 3_600.0), secs.into());
    }

    #[test]
    fn seconds_to_minutes() {
        let secs = Sec(300.0);
        assert_eq!(Min(secs.0 / 60.0), secs.into());
    }

    #[test]
    fn seconds_to_milliseconds() {
        let secs = Sec(300.0);
        assert_eq!(MilliSec(secs.0 * 1_000.0), secs.into());
    }

    #[test]
    fn seconds_to_microseconds() {
        let secs = Sec(300.0);
        assert_eq!(MicroSec(secs.0 * 1_000_000.0), secs.into());
    }

    #[test]
    fn seconds_to_nanoseconds() {
        let secs = Sec(300.0);
        assert_eq!(NanoSec(secs.0 * 1_000_000_000.0), secs.into());
    }



    #[test]
    fn milliseconds_to_days() {
        let secs = MilliSec(300.0);
        assert_eq!(Day(secs.0 / 86_400_000.0), secs.into());
    }

    #[test]
    fn milliseconds_to_hours() {
        let secs = MilliSec(300.0);
        assert_eq!(Hour(secs.0 / 3_600_000.0), secs.into());
    }

    #[test]
    fn milliseconds_to_minutes() {
        let secs = MilliSec(300.0);
        assert_eq!(Min(secs.0 / 60_000.0), secs.into());
    }

    #[test]
    fn milliseconds_to_seconds() {
        let millis = MilliSec(2_000.0);
        assert_eq!(Sec(millis.0 / 1_000.0), millis.into());
    }

    #[test]
    fn milliseconds_to_microseconds() {
        let millis = MilliSec(2_000.0);
        assert_eq!(MicroSec(millis.0 * 1_000.0), millis.into());
    }

    #[test]
    fn milliseconds_to_nanoseconds() {
        let millis = MilliSec(2_000.0);
        assert_eq!(NanoSec(millis.0 * 1_000_000.0), millis.into());
    }



    #[test]
    fn microseconds_to_days() {
        let secs = MicroSec(300.0);
        assert_eq!(Day(secs.0 / 86_400_000_000.0), secs.into());
    }

    #[test]
    fn microseconds_to_hours() {
        let secs = MicroSec(300.0);
        assert_eq!(Hour(secs.0 / 3_600_000_000.0), secs.into());
    }

    #[test]
    fn microseconds_to_minutes() {
        let secs = MicroSec(300.0);
        assert_eq!(Min(secs.0 / 60_000_000.0), secs.into());
    }

    #[test]
    fn microseconds_to_seconds() {
        let micros = MicroSec(2_000_000.0);
        assert_eq!(Sec(micros.0 / 1_000_000.0), micros.into());
    }

    #[test]
    fn microseconds_to_milliseconds() {
        let micros = MicroSec(300.0);
        assert_eq!(MilliSec(micros.0 / 1_000.0), micros.into());
    }

    #[test]
    fn microseconds_to_nanoseconds() {
        let micros = MicroSec(2_000_000.0);
        assert_eq!(NanoSec(micros.0 * 1_000.0), micros.into());
    }



    #[test]
    fn nanoseconds_to_days() {
        let secs = NanoSec(300.0);
        assert_eq!(Day(secs.0 / 86_400_000_000_000.0), secs.into());
    }

    #[test]
    fn nanoseconds_to_hours() {
        let secs = NanoSec(300.0);
        assert_eq!(Hour(secs.0 / 3_600_000_000_000.0), secs.into());
    }

    #[test]
    fn nanoseconds_to_minutes() {
        let secs = NanoSec(300.0);
        assert_eq!(Min(secs.0 / 60_000_000_000.0), secs.into());
    }

    #[test]
    fn nanoseconds_to_seconds() {
        let nanos = NanoSec(2_000_000_000.0);
        assert_eq!(Sec(nanos.0 / 1_000_000_000.0), nanos.into());
    }

    #[test]
    fn nanoseconds_to_milliseconds() {
        let nanos = NanoSec(300.0);
        assert_eq!(MilliSec(nanos.0 / 1_000_000.0), nanos.into());
    }

    #[test]
    fn nanoseconds_to_microseconds() {
        let nanos = NanoSec(2_000_000_000.0);
        assert_eq!(MicroSec(nanos.0 / 1_000.0), nanos.into());
    }



}
