use std::convert::From;
use std::ops::{Add, Div, Sub, Mul};

use length::*;
use time::*;

defunit! { MPerSec(f64), KmPerHour(f64) }


defadd! { for self: MPerSec, rhs      = self.0 + rhs.0 }
defsub! { for self: MPerSec, rhs      = self.0 - rhs.0 }
defmul! { for self: MPerSec, rhs: f64 = self.0 * rhs   }
defdiv! { for self: MPerSec, rhs: f64 = self.0 / rhs   }

defadd! { for self: KmPerHour, rhs      = self.0 + rhs.0 }
defsub! { for self: KmPerHour, rhs      = self.0 - rhs.0 }
defmul! { for self: KmPerHour, rhs: f64 = self.0 * rhs   }
defdiv! { for self: KmPerHour, rhs: f64 = self.0 / rhs   }


defdiv! { for (self:     M, rhs:  Sec) to MPerSec   = self.0 as f64 / rhs.0 }
defdiv! { for (self: KiloM, rhs: Hour) to KmPerHour = self.0 as f64 / rhs.0 }


conversion! { for (s: MPerSec)   to KmPerHour = s.0 * 3.6 }
conversion! { for (s: KmPerHour) to   MPerSec = s.0 / 3.6 }



#[cfg(test)]
mod speed_tests {
    use length::*;
    use time::*;
    use speed::*;

    #[test]
    fn create_mps() { assert_eq!(MPerSec(0.3), M(30.0) / Sec(100.0)); }
    #[test]
    fn create_kmph() { assert_eq!(KmPerHour(600.5), KiloM(1201.0) / Hour(2.0)); }

    #[test]
    fn kmph_to_mps() { assert_eq!(KiloM(3600.0) / Hour(1.0), MPerSec(1000.0).into()); }
    #[test]
    fn mps_to_kmph() { assert_eq!(M(1000.0) / Sec(1.0), KmPerHour(3600.0).into()); }

    #[test]
    fn add_mps() { assert_eq!(MPerSec(15.5), MPerSec(10.2) + MPerSec(5.3)); }
    #[test]
    fn add_kmph() { assert_eq!(KmPerHour(15.0), KmPerHour(10.0) + KmPerHour(5.0)); }


}
