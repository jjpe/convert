use std::convert::From;
use std::ops::{Add, Div, Sub, Mul};

defunit! {
    KiloM(u64), HectoM(u64), DecaM(u64), M(u64), DeciM(u64),
    CentiM(u64), MilliM(u64), MicroM(u64), NanoM(u64)
}

conversion! { for t: KiloM => HectoM = t.0 *                10 }
conversion! { for t: KiloM =>  DecaM = t.0 *               100 }
conversion! { for t: KiloM =>      M = t.0 *             1_000 }
conversion! { for t: KiloM =>  DeciM = t.0 *            10_000 }
conversion! { for t: KiloM => CentiM = t.0 *           100_000 }
conversion! { for t: KiloM => MilliM = t.0 *         1_000_000 }
conversion! { for t: KiloM => MicroM = t.0 *     1_000_000_000 }
conversion! { for t: KiloM =>  NanoM = t.0 * 1_000_000_000_000 }

conversion! { for t: HectoM =>  KiloM = t.0 / 10 }
conversion! { for t: HectoM =>  DecaM = t.0 *              10 }
conversion! { for t: HectoM =>      M = t.0 *             100 }
conversion! { for t: HectoM =>  DeciM = t.0 *           1_000 }
conversion! { for t: HectoM => CentiM = t.0 *          10_000 }
conversion! { for t: HectoM => MilliM = t.0 *         100_000 }
conversion! { for t: HectoM => MicroM = t.0 *     100_000_000 }
conversion! { for t: HectoM =>  NanoM = t.0 * 100_000_000_000 }

conversion! { for t: DecaM =>   KiloM = t.0 / 100 }
conversion! { for t: DecaM =>  HectoM = t.0 /  10 }
conversion! { for t: DecaM =>       M = t.0 *             10 }
conversion! { for t: DecaM =>   DeciM = t.0 *            100 }
conversion! { for t: DecaM =>  CentiM = t.0 *           1000 }
conversion! { for t: DecaM =>  MilliM = t.0 *         10_000 }
conversion! { for t: DecaM =>  MicroM = t.0 *     10_000_000 }
conversion! { for t: DecaM =>   NanoM = t.0 * 10_000_000_000 }

conversion! { for t: M =>  KiloM = t.0 / 1000 }
conversion! { for t: M => HectoM = t.0 /  100 }
conversion! { for t: M =>  DecaM = t.0 /   10 }
conversion! { for t: M =>  DeciM = t.0 *           10 }
conversion! { for t: M => CentiM = t.0 *          100 }
conversion! { for t: M => MilliM = t.0 *        1_000 }
conversion! { for t: M => MicroM = t.0 *     1000_000 }
conversion! { for t: M =>  NanoM = t.0 * 1000_000_000 }

conversion! { for t: DeciM =>  KiloM = t.0 / 10_000 }
conversion! { for t: DeciM => HectoM = t.0 /  1_000 }
conversion! { for t: DeciM =>  DecaM = t.0 /    100 }
conversion! { for t: DeciM =>      M = t.0 /     10 }
conversion! { for t: DeciM => CentiM = t.0 *          10 }
conversion! { for t: DeciM => MilliM = t.0 *         100 }
conversion! { for t: DeciM => MicroM = t.0 *     100_000 }
conversion! { for t: DeciM =>  NanoM = t.0 * 100_000_000 }

conversion! { for t: CentiM =>  KiloM = t.0 / 100_000 }
conversion! { for t: CentiM => HectoM = t.0 /  10_000 }
conversion! { for t: CentiM =>  DecaM = t.0 /   1_000 }
conversion! { for t: CentiM =>      M = t.0 /     100 }
conversion! { for t: CentiM =>  DeciM = t.0 /      10 }
conversion! { for t: CentiM => MilliM = t.0 *         10 }
conversion! { for t: CentiM => MicroM = t.0 *     10_000 }
conversion! { for t: CentiM =>  NanoM = t.0 * 10_000_000 }

conversion! { for t: MilliM =>  KiloM = t.0 / 1_000_000 }
conversion! { for t: MilliM => HectoM = t.0 /   100_000 }
conversion! { for t: MilliM =>  DecaM = t.0 /    10_000 }
conversion! { for t: MilliM =>      M = t.0 /     1_000 }
conversion! { for t: MilliM =>  DeciM = t.0 /       100 }
conversion! { for t: MilliM => CentiM = t.0 /        10 }
conversion! { for t: MilliM => MicroM = t.0 *     1_000 }
conversion! { for t: MilliM =>  NanoM = t.0 * 1_000_000 }

conversion! { for t: MicroM =>  KiloM = t.0 / 1_000_000_000 }
conversion! { for t: MicroM => HectoM = t.0 /   100_000_000 }
conversion! { for t: MicroM =>  DecaM = t.0 /    10_000_000 }
conversion! { for t: MicroM =>      M = t.0 /     1_000_000 }
conversion! { for t: MicroM =>  DeciM = t.0 /       100_000 }
conversion! { for t: MicroM => CentiM = t.0 /        10_000 }
conversion! { for t: MicroM => MilliM = t.0 /         1_000 }
conversion! { for t: MicroM =>  NanoM = t.0 * 1_000 }

conversion! { for t: NanoM =>  KiloM = t.0 / 1_000_000_000_000 }
conversion! { for t: NanoM => HectoM = t.0 /   100_000_000_000 }
conversion! { for t: NanoM =>  DecaM = t.0 /    10_000_000_000 }
conversion! { for t: NanoM =>      M = t.0 /     1_000_000_000 }
conversion! { for t: NanoM =>  DeciM = t.0 /       100_000_000 }
conversion! { for t: NanoM => CentiM = t.0 /        10_000_000 }
conversion! { for t: NanoM => MilliM = t.0 /         1_000_000 }
conversion! { for t: NanoM => MicroM = t.0 /             1_000 }




defadd! { for self: KiloM, rhs      = self.0 + rhs.0 }
defsub! { for self: KiloM, rhs      = self.0 - rhs.0 }
defmul! { for self: KiloM, rhs: u64 = self.0 * rhs   }
defdiv! { for self: KiloM, rhs: u64 = self.0 / rhs   }

defadd! { for self: HectoM, rhs      = self.0 + rhs.0 }
defsub! { for self: HectoM, rhs      = self.0 - rhs.0 }
defmul! { for self: HectoM, rhs: u64 = self.0 * rhs   }
defdiv! { for self: HectoM, rhs: u64 = self.0 / rhs   }

defadd! { for self: DecaM, rhs      = self.0 + rhs.0 }
defsub! { for self: DecaM, rhs      = self.0 - rhs.0 }
defmul! { for self: DecaM, rhs: u64 = self.0 * rhs   }
defdiv! { for self: DecaM, rhs: u64 = self.0 / rhs   }

defadd! { for self: M, rhs      = self.0 + rhs.0 }
defsub! { for self: M, rhs      = self.0 - rhs.0 }
defmul! { for self: M, rhs: u64 = self.0 * rhs   }
defdiv! { for self: M, rhs: u64 = self.0 / rhs   }

defadd! { for self: DeciM, rhs      = self.0 + rhs.0 }
defsub! { for self: DeciM, rhs      = self.0 - rhs.0 }
defmul! { for self: DeciM, rhs: u64 = self.0 * rhs   }
defdiv! { for self: DeciM, rhs: u64 = self.0 / rhs   }

defadd! { for self: CentiM, rhs      = self.0 + rhs.0 }
defsub! { for self: CentiM, rhs      = self.0 - rhs.0 }
defmul! { for self: CentiM, rhs: u64 = self.0 * rhs   }
defdiv! { for self: CentiM, rhs: u64 = self.0 / rhs   }

defadd! { for self: MilliM, rhs      = self.0 + rhs.0 }
defsub! { for self: MilliM, rhs      = self.0 - rhs.0 }
defmul! { for self: MilliM, rhs: u64 = self.0 * rhs   }
defdiv! { for self: MilliM, rhs: u64 = self.0 / rhs   }

defadd! { for self: MicroM, rhs      = self.0 + rhs.0 }
defsub! { for self: MicroM, rhs      = self.0 - rhs.0 }
defmul! { for self: MicroM, rhs: u64 = self.0 * rhs   }
defdiv! { for self: MicroM, rhs: u64 = self.0 / rhs   }

defadd! { for self: NanoM, rhs      = self.0 + rhs.0 }
defsub! { for self: NanoM, rhs      = self.0 - rhs.0 }
defmul! { for self: NanoM, rhs: u64 = self.0 * rhs   }
defdiv! { for self: NanoM, rhs: u64 = self.0 / rhs   }


#[cfg(test)]
mod length_tests {
    use length::*;

    #[test]
    fn add_kilometers() { assert_eq!(KiloM(15), KiloM(10) + KiloM(5)); }
    #[test]
    fn sub_kilometers() { assert_eq!(KiloM(5), KiloM(10) - KiloM(5)); }
    #[test]
    fn mul_kilometers() { assert_eq!(KiloM(30), KiloM(10) * 3); }
    #[test]
    fn div_kilometers() { assert_eq!(KiloM(10), KiloM(30) / 3); }

    #[test]
    fn add_hectometers() { assert_eq!(HectoM(15), HectoM(10) + HectoM(5)); }
    #[test]
    fn sub_hectometers() { assert_eq!(HectoM(5), HectoM(10) - HectoM(5)); }
    #[test]
    fn mul_hectometers() { assert_eq!(HectoM(30), HectoM(10) * 3); }
    #[test]
    fn div_hectometers() { assert_eq!(HectoM(10), HectoM(30) / 3); }

    #[test]
    fn add_decameters() { assert_eq!(DecaM(15), DecaM(10) + DecaM(5)); }
    #[test]
    fn sub_decameters() { assert_eq!(DecaM(5), DecaM(10) - DecaM(5)); }
    #[test]
    fn mul_decameters() { assert_eq!(DecaM(30), DecaM(10) * 3); }
    #[test]
    fn div_decameters() { assert_eq!(DecaM(10), DecaM(30) / 3); }

    #[test]
    fn add_meters() { assert_eq!(M(15), M(10) + M(5)); }
    #[test]
    fn sub_meters() { assert_eq!(M(5), M(10) - M(5)); }
    #[test]
    fn mul_meters() { assert_eq!(M(30), M(10) * 3); }
    #[test]
    fn div_meters() { assert_eq!(M(10), M(30) / 3); }

    #[test]
    fn add_decimeters() { assert_eq!(DeciM(15), DeciM(10) + DeciM(5)); }
    #[test]
    fn sub_decimeters() { assert_eq!(DeciM(5), DeciM(10) - DeciM(5)); }
    #[test]
    fn mul_decimeters() { assert_eq!(DeciM(30), DeciM(10) * 3); }
    #[test]
    fn div_decimeters() { assert_eq!(DeciM(10), DeciM(30) / 3); }

    #[test]
    fn add_centimeters() { assert_eq!(CentiM(15), CentiM(10) + CentiM(5)); }
    #[test]
    fn sub_centimeters() { assert_eq!(CentiM(5), CentiM(10) - CentiM(5)); }
    #[test]
    fn mul_centimeters() { assert_eq!(CentiM(30), CentiM(10) * 3); }
    #[test]
    fn div_centimeters() { assert_eq!(CentiM(10), CentiM(30) / 3); }

    #[test]
    fn add_millimeters() { assert_eq!(MilliM(15), MilliM(10) + MilliM(5)); }
    #[test]
    fn sub_millimeters() { assert_eq!(MilliM(5), MilliM(10) - MilliM(5)); }
    #[test]
    fn mul_millimeters() { assert_eq!(MilliM(30), MilliM(10) * 3); }
    #[test]
    fn div_millimeters() { assert_eq!(MilliM(10), MilliM(30) / 3); }

    #[test]
    fn add_micrometers() { assert_eq!(MicroM(15), MicroM(10) + MicroM(5)); }
    #[test]
    fn sub_micrometers() { assert_eq!(MicroM(5), MicroM(10) - MicroM(5)); }
    #[test]
    fn mul_micrometers() { assert_eq!(MicroM(30), MicroM(10) * 3); }
    #[test]
    fn div_micrometers() { assert_eq!(MicroM(10), MicroM(30) / 3); }

    #[test]
    fn add_nanometers() { assert_eq!(NanoM(15), NanoM(10) + NanoM(5)); }
    #[test]
    fn sub_nanometers() { assert_eq!(NanoM(5), NanoM(10) - NanoM(5)); }
    #[test]
    fn mul_nanometers() { assert_eq!(NanoM(30), NanoM(10) * 3); }
    #[test]
    fn div_nanometers() { assert_eq!(NanoM(10), NanoM(30) / 3); }



    #[test]
    fn kilometers_to_hectometers() {
        let km = KiloM(50);
        assert_eq!(HectoM(km.0 * 10), km.into());
    }

    #[test]
    fn kilometers_to_decameters() {
        let km = KiloM(50);
        assert_eq!(DecaM(km.0 * 100), km.into());
    }

    #[test]
    fn kilometers_to_meters() {
        let km = KiloM(50);
        assert_eq!(M(km.0 * 1000), km.into());
    }

    #[test]
    fn kilometers_to_decimeters() {
        let km = KiloM(50);
        assert_eq!(DeciM(km.0 * 10_000), km.into());
    }

    #[test]
    fn kilometers_to_centimeters() {
        let km = KiloM(50);
        assert_eq!(CentiM(km.0 * 100_000), km.into());
    }

    #[test]
    fn kilometers_to_millimeters() {
        let km = KiloM(50);
        assert_eq!(MilliM(km.0 * 1_000_000), km.into());
    }

    #[test]
    fn kilometers_to_micrometers() {
        let km = KiloM(50);
        assert_eq!(MicroM(km.0 * 1_000_000_000), km.into());
    }

    #[test]
    fn kilometers_to_nanometers() {
        let km = KiloM(50);
        assert_eq!(NanoM(km.0 * 1_000_000_000_000), km.into());
    }




    #[test]
    fn hectometers_to_kilometers() {
        let hm = HectoM(50);
        assert_eq!(KiloM(hm.0 / 10), hm.into());
    }

    #[test]
    fn hectometers_to_decameters() {
        let hm = HectoM(50);
        assert_eq!(DecaM(hm.0 * 10), hm.into());
    }

    #[test]
    fn hectometers_to_meters() {
        let hm = HectoM(50);
        assert_eq!(M(hm.0 * 100), hm.into());
    }

    #[test]
    fn hectometers_to_decimeters() {
        let hm = HectoM(50);
        assert_eq!(DeciM(hm.0 * 1_000), hm.into());
    }

    #[test]
    fn hectometers_to_centimeters() {
        let hm = HectoM(50);
        assert_eq!(CentiM(hm.0 * 10_000), hm.into());
    }

    #[test]
    fn hectometers_to_millimeters() {
        let hm = HectoM(50);
        assert_eq!(MilliM(hm.0 * 100_000), hm.into());
    }

    #[test]
    fn hectometers_to_micrometers() {
        let hm = HectoM(50);
        assert_eq!(MicroM(hm.0 * 100_000_000), hm.into());
    }

    #[test]
    fn hectometers_to_nanometers() {
        let hm = HectoM(50);
        assert_eq!(NanoM(hm.0 * 100_000_000_000), hm.into());
    }



    #[test]
    fn decameters_to_kilometers() {
        let hm = DecaM(50);
        assert_eq!(KiloM(hm.0 / 100), hm.into());
    }

    #[test]
    fn decameters_to_hectometers() {
        let hm = DecaM(50);
        assert_eq!(HectoM(hm.0 / 10), hm.into());
    }

    #[test]
    fn decameters_to_meters() {
        let hm = DecaM(50);
        assert_eq!(M(hm.0 * 10), hm.into());
    }

    #[test]
    fn decameters_to_decimeters() {
        let hm = DecaM(50);
        assert_eq!(DeciM(hm.0 * 100), hm.into());
    }

    #[test]
    fn decameters_to_centimeters() {
        let hm = DecaM(50);
        assert_eq!(CentiM(hm.0 * 1_000), hm.into());
    }

    #[test]
    fn decameters_to_millimeters() {
        let hm = DecaM(50);
        assert_eq!(MilliM(hm.0 * 10_000), hm.into());
    }

    #[test]
    fn decameters_to_micrometers() {
        let hm = DecaM(50);
        assert_eq!(MicroM(hm.0 * 10_000_000), hm.into());
    }

    #[test]
    fn decameters_to_nanometers() {
        let hm = DecaM(50);
        assert_eq!(NanoM(hm.0 * 10_000_000_000), hm.into());
    }



    #[test]
    fn meters_to_kilometers() {
        let hm = M(50);
        assert_eq!(KiloM(hm.0 / 1000), hm.into());
    }

    #[test]
    fn meters_to_hectometers() {
        let hm = M(50);
        assert_eq!(HectoM(hm.0 / 100), hm.into());
    }

    #[test]
    fn meters_to_decameters() {
        let hm = M(50);
        assert_eq!(DecaM(hm.0 / 10), hm.into());
    }

    #[test]
    fn meters_to_decimeters() {
        let hm = M(50);
        assert_eq!(DeciM(hm.0 * 10), hm.into());
    }

    #[test]
    fn meters_to_centimeters() {
        let hm = M(50);
        assert_eq!(CentiM(hm.0 * 100), hm.into());
    }

    #[test]
    fn meters_to_millimeters() {
        let hm = M(50);
        assert_eq!(MilliM(hm.0 * 1_000), hm.into());
    }

    #[test]
    fn meters_to_micrometers() {
        let hm = M(50);
        assert_eq!(MicroM(hm.0 * 1_000_000), hm.into());
    }

    #[test]
    fn meters_to_nanometers() {
        let hm = M(50);
        assert_eq!(NanoM(hm.0 * 1_000_000_000), hm.into());
    }



    #[test]
    fn decimeters_to_kilometers() {
        let hm = DeciM(50);
        assert_eq!(KiloM(hm.0 / 10_000), hm.into());
    }

    #[test]
    fn decimeters_to_hectometers() {
        let hm = DeciM(50);
        assert_eq!(HectoM(hm.0 / 1_000), hm.into());
    }

    #[test]
    fn decimeters_to_decameters() {
        let hm = DeciM(50);
        assert_eq!(DecaM(hm.0 / 100), hm.into());
    }

    #[test]
    fn decimeters_to_meters() {
        let hm = DeciM(50);
        assert_eq!(M(hm.0 / 10), hm.into());
    }

    #[test]
    fn decimeters_to_centimeters() {
        let hm = DeciM(50);
        assert_eq!(CentiM(hm.0 * 10), hm.into());
    }

    #[test]
    fn decimeters_to_millimeters() {
        let hm = DeciM(50);
        assert_eq!(MilliM(hm.0 * 100), hm.into());
    }

    #[test]
    fn decimeters_to_micrometers() {
        let hm = DeciM(50);
        assert_eq!(MicroM(hm.0 * 100_000), hm.into());
    }

    #[test]
    fn decimeters_to_nanometers() {
        let hm = DeciM(50);
        assert_eq!(NanoM(hm.0 * 100_000_000), hm.into());
    }


    #[test]
    fn centimeters_to_kilometers() {
        let hm = CentiM(50);
        assert_eq!(KiloM(hm.0 / 10_000), hm.into());
    }

    #[test]
    fn centimeters_to_hectometers() {
        let hm = CentiM(50);
        assert_eq!(HectoM(hm.0 / 1_000), hm.into());
    }

    #[test]
    fn centimeters_to_decameters() {
        let hm = CentiM(50);
        assert_eq!(DecaM(hm.0 / 100), hm.into());
    }

    #[test]
    fn centimeters_to_meters() {
        let hm = CentiM(50);
        assert_eq!(M(hm.0 / 100), hm.into());
    }

    #[test]
    fn centimeters_to_decimeters() {
        let hm = CentiM(50);
        assert_eq!(DeciM(hm.0 / 10), hm.into());
    }

    #[test]
    fn centimeters_to_millimeters() {
        let hm = CentiM(50);
        assert_eq!(MilliM(hm.0 * 10), hm.into());
    }

    #[test]
    fn centimeters_to_micrometers() {
        let hm = CentiM(50);
        assert_eq!(MicroM(hm.0 * 10_000), hm.into());
    }

    #[test]
    fn centimeters_to_nanometers() {
        let hm = CentiM(50);
        assert_eq!(NanoM(hm.0 * 10_000_000), hm.into());
    }



    #[test]
    fn millimeters_to_kilometers() {
        let hm = MilliM(50);
        assert_eq!(KiloM(hm.0 / 10_000), hm.into());
    }

    #[test]
    fn millimeters_to_hectometers() {
        let hm = MilliM(50);
        assert_eq!(HectoM(hm.0 / 1_000), hm.into());
    }

    #[test]
    fn millimeters_to_decameters() {
        let hm = MilliM(50);
        assert_eq!(DecaM(hm.0 / 10_000), hm.into());
    }

    #[test]
    fn millimeters_to_meters() {
        let hm = MilliM(50);
        assert_eq!(M(hm.0 / 1000), hm.into());
    }

    #[test]
    fn millimeters_to_decimeters() {
        let hm = MilliM(50);
        assert_eq!(DeciM(hm.0 / 100), hm.into());
    }

    #[test]
    fn millimeters_to_centimeters() {
        let hm = MilliM(50);
        assert_eq!(CentiM(hm.0 / 10), hm.into());
    }

    #[test]
    fn millimeters_to_micrometers() {
        let hm = MilliM(50);
        assert_eq!(MicroM(hm.0 * 1_000), hm.into());
    }

    #[test]
    fn millimeters_to_nanometers() {
        let hm = MilliM(50);
        assert_eq!(NanoM(hm.0 * 1_000_000), hm.into());
    }



    #[test]
    fn micrometers_to_kilometers() {
        let hm = MicroM(50);
        assert_eq!(KiloM(hm.0 / 1_000_000_000), hm.into());
    }

    #[test]
    fn micrometers_to_hectometers() {
        let hm = MicroM(50);
        assert_eq!(HectoM(hm.0 / 100_000_000), hm.into());
    }

    #[test]
    fn micrometers_to_decameters() {
        let hm = MicroM(50);
        assert_eq!(DecaM(hm.0 / 10_000_000), hm.into());
    }

    #[test]
    fn micrometers_to_meters() {
        let hm = MicroM(50);
        assert_eq!(M(hm.0 / 1_000_000), hm.into());
    }

    #[test]
    fn micrometers_to_decimeters() {
        let hm = MicroM(50);
        assert_eq!(DeciM(hm.0 / 100_000), hm.into());
    }

    #[test]
    fn micrometers_to_centimeters() {
        let hm = MicroM(50);
        assert_eq!(CentiM(hm.0 / 10_000), hm.into());
    }

    #[test]
    fn micrometers_to_millimeters() {
        let hm = MicroM(50);
        assert_eq!(MilliM(hm.0 / 1_000), hm.into());
    }

    #[test]
    fn micrometers_to_nanometers() {
        let hm = MicroM(50);
        assert_eq!(NanoM(hm.0 * 1_000), hm.into());
    }



    #[test]
    fn nanometers_to_kilometers() {
        let hm = NanoM(50);
        assert_eq!(KiloM(hm.0 / 1_000_000_000_000), hm.into());
    }

    #[test]
    fn nanometers_to_hectometers() {
        let hm = NanoM(50);
        assert_eq!(HectoM(hm.0 / 100_000_000_000), hm.into());
    }

    #[test]
    fn nanometers_to_decameters() {
        let hm = NanoM(50);
        assert_eq!(DecaM(hm.0 / 10_000_000_000), hm.into());
    }

    #[test]
    fn nanometers_to_meters() {
        let hm = NanoM(50);
        assert_eq!(M(hm.0 / 1_000_000_000), hm.into());
    }

    #[test]
    fn nanometers_to_decimeters() {
        let hm = NanoM(50);
        assert_eq!(DeciM(hm.0 / 100_000_000), hm.into());
    }

    #[test]
    fn nanometers_to_centimeters() {
        let hm = NanoM(50);
        assert_eq!(CentiM(hm.0 / 10_000_000), hm.into());
    }

    #[test]
    fn nanometers_to_millimeters() {
        let hm = NanoM(50);
        assert_eq!(MilliM(hm.0 / 1_000), hm.into());
    }

    #[test]
    fn nanometers_to_micrometers() {
        let hm = NanoM(50);
        assert_eq!(MicroM(hm.0 / 1_000), hm.into());
    }



}
