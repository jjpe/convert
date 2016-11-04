use std::convert::From;
use std::ops::{Add, Div, Sub, Mul};


defunit! {
    KiloM(f64), HectoM(f64), DecaM(f64), M(f64), DeciM(f64),
    CentiM(f64), MilliM(f64), MicroM(f64), NanoM(f64)
}

conversion! { for t: KiloM => HectoM = t.0 *                10.0 }
conversion! { for t: KiloM =>  DecaM = t.0 *               100.0 }
conversion! { for t: KiloM =>      M = t.0 *             1_000.0 }
conversion! { for t: KiloM =>  DeciM = t.0 *            10_000.0 }
conversion! { for t: KiloM => CentiM = t.0 *           100_000.0 }
conversion! { for t: KiloM => MilliM = t.0 *         1_000_000.0 }
conversion! { for t: KiloM => MicroM = t.0 *     1_000_000_000.0 }
conversion! { for t: KiloM =>  NanoM = t.0 * 1_000_000_000_000.0 }

conversion! { for t: HectoM =>  KiloM = t.0 / 10.0 }
conversion! { for t: HectoM =>  DecaM = t.0 *              10.0 }
conversion! { for t: HectoM =>      M = t.0 *             100.0 }
conversion! { for t: HectoM =>  DeciM = t.0 *           1_000.0 }
conversion! { for t: HectoM => CentiM = t.0 *          10_000.0 }
conversion! { for t: HectoM => MilliM = t.0 *         100_000.0 }
conversion! { for t: HectoM => MicroM = t.0 *     100_000_000.0 }
conversion! { for t: HectoM =>  NanoM = t.0 * 100_000_000_000.0 }

conversion! { for t: DecaM =>   KiloM = t.0 / 100.0 }
conversion! { for t: DecaM =>  HectoM = t.0 /  10.0 }
conversion! { for t: DecaM =>       M = t.0 *             10.0 }
conversion! { for t: DecaM =>   DeciM = t.0 *            100.0 }
conversion! { for t: DecaM =>  CentiM = t.0 *           1000.0 }
conversion! { for t: DecaM =>  MilliM = t.0 *         10_000.0 }
conversion! { for t: DecaM =>  MicroM = t.0 *     10_000_000.0 }
conversion! { for t: DecaM =>   NanoM = t.0 * 10_000_000_000.0 }

conversion! { for t: M =>  KiloM = t.0 / 1000.0 }
conversion! { for t: M => HectoM = t.0 /  100.0 }
conversion! { for t: M =>  DecaM = t.0 /   10.0 }
conversion! { for t: M =>  DeciM = t.0 *           10.0 }
conversion! { for t: M => CentiM = t.0 *          100.0 }
conversion! { for t: M => MilliM = t.0 *        1_000.0 }
conversion! { for t: M => MicroM = t.0 *     1000_000.0 }
conversion! { for t: M =>  NanoM = t.0 * 1000_000_000.0 }

conversion! { for t: DeciM =>  KiloM = t.0 / 10_000.0 }
conversion! { for t: DeciM => HectoM = t.0 /  1_000.0 }
conversion! { for t: DeciM =>  DecaM = t.0 /    100.0 }
conversion! { for t: DeciM =>      M = t.0 /     10.0 }
conversion! { for t: DeciM => CentiM = t.0 *          10.0 }
conversion! { for t: DeciM => MilliM = t.0 *         100.0 }
conversion! { for t: DeciM => MicroM = t.0 *     100_000.0 }
conversion! { for t: DeciM =>  NanoM = t.0 * 100_000_000.0 }

conversion! { for t: CentiM =>  KiloM = t.0 / 100_000.0 }
conversion! { for t: CentiM => HectoM = t.0 /  10_000.0 }
conversion! { for t: CentiM =>  DecaM = t.0 /   1_000.0 }
conversion! { for t: CentiM =>      M = t.0 /     100.0 }
conversion! { for t: CentiM =>  DeciM = t.0 /      10.0 }
conversion! { for t: CentiM => MilliM = t.0 *         10.0 }
conversion! { for t: CentiM => MicroM = t.0 *     10_000.0 }
conversion! { for t: CentiM =>  NanoM = t.0 * 10_000_000.0 }

conversion! { for t: MilliM =>  KiloM = t.0 / 1_000_000.0 }
conversion! { for t: MilliM => HectoM = t.0 /   100_000.0 }
conversion! { for t: MilliM =>  DecaM = t.0 /    10_000.0 }
conversion! { for t: MilliM =>      M = t.0 /     1_000.0 }
conversion! { for t: MilliM =>  DeciM = t.0 /       100.0 }
conversion! { for t: MilliM => CentiM = t.0 /        10.0 }
conversion! { for t: MilliM => MicroM = t.0 *     1_000.0 }
conversion! { for t: MilliM =>  NanoM = t.0 * 1_000_000.0 }

conversion! { for t: MicroM =>  KiloM = t.0 / 1_000_000_000.0 }
conversion! { for t: MicroM => HectoM = t.0 /   100_000_000.0 }
conversion! { for t: MicroM =>  DecaM = t.0 /    10_000_000.0 }
conversion! { for t: MicroM =>      M = t.0 /     1_000_000.0 }
conversion! { for t: MicroM =>  DeciM = t.0 /       100_000.0 }
conversion! { for t: MicroM => CentiM = t.0 /        10_000.0 }
conversion! { for t: MicroM => MilliM = t.0 /         1_000.0 }
conversion! { for t: MicroM =>  NanoM = t.0 * 1_000.0 }

conversion! { for t: NanoM =>  KiloM = t.0 / 1_000_000_000_000.0 }
conversion! { for t: NanoM => HectoM = t.0 /   100_000_000_000.0 }
conversion! { for t: NanoM =>  DecaM = t.0 /    10_000_000_000.0 }
conversion! { for t: NanoM =>      M = t.0 /     1_000_000_000.0 }
conversion! { for t: NanoM =>  DeciM = t.0 /       100_000_000.0 }
conversion! { for t: NanoM => CentiM = t.0 /        10_000_000.0 }
conversion! { for t: NanoM => MilliM = t.0 /         1_000_000.0 }
conversion! { for t: NanoM => MicroM = t.0 /             1_000.0 }




defadd! { for self: KiloM, rhs      = self.0 + rhs.0 }
defsub! { for self: KiloM, rhs      = self.0 - rhs.0 }
defmul! { for self: KiloM, rhs: f64 = self.0 * rhs   }
defdiv! { for self: KiloM, rhs: f64 = self.0 / rhs   }

defadd! { for self: HectoM, rhs      = self.0 + rhs.0 }
defsub! { for self: HectoM, rhs      = self.0 - rhs.0 }
defmul! { for self: HectoM, rhs: f64 = self.0 * rhs   }
defdiv! { for self: HectoM, rhs: f64 = self.0 / rhs   }

defadd! { for self: DecaM, rhs      = self.0 + rhs.0 }
defsub! { for self: DecaM, rhs      = self.0 - rhs.0 }
defmul! { for self: DecaM, rhs: f64 = self.0 * rhs   }
defdiv! { for self: DecaM, rhs: f64 = self.0 / rhs   }

defadd! { for self: M, rhs      = self.0 + rhs.0 }
defsub! { for self: M, rhs      = self.0 - rhs.0 }
defmul! { for self: M, rhs: f64 = self.0 * rhs   }
defdiv! { for self: M, rhs: f64 = self.0 / rhs   }

defadd! { for self: DeciM, rhs      = self.0 + rhs.0 }
defsub! { for self: DeciM, rhs      = self.0 - rhs.0 }
defmul! { for self: DeciM, rhs: f64 = self.0 * rhs   }
defdiv! { for self: DeciM, rhs: f64 = self.0 / rhs   }

defadd! { for self: CentiM, rhs      = self.0 + rhs.0 }
defsub! { for self: CentiM, rhs      = self.0 - rhs.0 }
defmul! { for self: CentiM, rhs: f64 = self.0 * rhs   }
defdiv! { for self: CentiM, rhs: f64 = self.0 / rhs   }

defadd! { for self: MilliM, rhs      = self.0 + rhs.0 }
defsub! { for self: MilliM, rhs      = self.0 - rhs.0 }
defmul! { for self: MilliM, rhs: f64 = self.0 * rhs   }
defdiv! { for self: MilliM, rhs: f64 = self.0 / rhs   }

defadd! { for self: MicroM, rhs      = self.0 + rhs.0 }
defsub! { for self: MicroM, rhs      = self.0 - rhs.0 }
defmul! { for self: MicroM, rhs: f64 = self.0 * rhs   }
defdiv! { for self: MicroM, rhs: f64 = self.0 / rhs   }

defadd! { for self: NanoM, rhs      = self.0 + rhs.0 }
defsub! { for self: NanoM, rhs      = self.0 - rhs.0 }
defmul! { for self: NanoM, rhs: f64 = self.0 * rhs   }
defdiv! { for self: NanoM, rhs: f64 = self.0 / rhs   }


#[cfg(test)]
mod test {
    use length::*;

    #[test]
    fn add_kilometers() { assert_eq!(KiloM(15.0), KiloM(10.0) + KiloM(5.0)); }
    #[test]
    fn sub_kilometers() { assert_eq!(KiloM(5.0), KiloM(10.0) - KiloM(5.0)); }
    #[test]
    fn mul_kilometers() { assert_eq!(KiloM(30.0), KiloM(10.0) * 3.0); }
    #[test]
    fn div_kilometers() { assert_eq!(KiloM(10.0), KiloM(30.0) / 3.0); }

    #[test]
    fn add_hectometers() { assert_eq!(HectoM(15.0), HectoM(10.0) + HectoM(5.0)); }
    #[test]
    fn sub_hectometers() { assert_eq!(HectoM(5.0), HectoM(10.0) - HectoM(5.0)); }
    #[test]
    fn mul_hectometers() { assert_eq!(HectoM(30.0), HectoM(10.0) * 3.0); }
    #[test]
    fn div_hectometers() { assert_eq!(HectoM(10.0), HectoM(30.0) / 3.0); }

    #[test]
    fn add_decameters() { assert_eq!(DecaM(15.0), DecaM(10.0) + DecaM(5.0)); }
    #[test]
    fn sub_decameters() { assert_eq!(DecaM(5.0), DecaM(10.0) - DecaM(5.0)); }
    #[test]
    fn mul_decameters() { assert_eq!(DecaM(30.0), DecaM(10.0) * 3.0); }
    #[test]
    fn div_decameters() { assert_eq!(DecaM(10.0), DecaM(30.0) / 3.0); }

    #[test]
    fn add_meters() { assert_eq!(M(15.0), M(10.0) + M(5.0)); }
    #[test]
    fn sub_meters() { assert_eq!(M(5.0), M(10.0) - M(5.0)); }
    #[test]
    fn mul_meters() { assert_eq!(M(30.0), M(10.0) * 3.0); }
    #[test]
    fn div_meters() { assert_eq!(M(10.0), M(30.0) / 3.0); }

    #[test]
    fn add_decimeters() { assert_eq!(DeciM(15.0), DeciM(10.0) + DeciM(5.0)); }
    #[test]
    fn sub_decimeters() { assert_eq!(DeciM(5.0), DeciM(10.0) - DeciM(5.0)); }
    #[test]
    fn mul_decimeters() { assert_eq!(DeciM(30.0), DeciM(10.0) * 3.0); }
    #[test]
    fn div_decimeters() { assert_eq!(DeciM(10.0), DeciM(30.0) / 3.0); }

    #[test]
    fn add_centimeters() { assert_eq!(CentiM(15.0), CentiM(10.0) + CentiM(5.0)); }
    #[test]
    fn sub_centimeters() { assert_eq!(CentiM(5.0), CentiM(10.0) - CentiM(5.0)); }
    #[test]
    fn mul_centimeters() { assert_eq!(CentiM(30.0), CentiM(10.0) * 3.0); }
    #[test]
    fn div_centimeters() { assert_eq!(CentiM(10.0), CentiM(30.0) / 3.0); }

    #[test]
    fn add_millimeters() { assert_eq!(MilliM(15.0), MilliM(10.0) + MilliM(5.0)); }
    #[test]
    fn sub_millimeters() { assert_eq!(MilliM(5.0), MilliM(10.0) - MilliM(5.0)); }
    #[test]
    fn mul_millimeters() { assert_eq!(MilliM(30.0), MilliM(10.0) * 3.0); }
    #[test]
    fn div_millimeters() { assert_eq!(MilliM(10.0), MilliM(30.0) / 3.0); }

    #[test]
    fn add_micrometers() { assert_eq!(MicroM(15.0), MicroM(10.0) + MicroM(5.0)); }
    #[test]
    fn sub_micrometers() { assert_eq!(MicroM(5.0), MicroM(10.0) - MicroM(5.0)); }
    #[test]
    fn mul_micrometers() { assert_eq!(MicroM(30.0), MicroM(10.0) * 3.0); }
    #[test]
    fn div_micrometers() { assert_eq!(MicroM(10.0), MicroM(30.0) / 3.0); }

    #[test]
    fn add_nanometers() { assert_eq!(NanoM(15.0), NanoM(10.0) + NanoM(5.0)); }
    #[test]
    fn sub_nanometers() { assert_eq!(NanoM(5.0), NanoM(10.0) - NanoM(5.0)); }
    #[test]
    fn mul_nanometers() { assert_eq!(NanoM(30.0), NanoM(10.0) * 3.0); }
    #[test]
    fn div_nanometers() { assert_eq!(NanoM(10.0), NanoM(30.0) / 3.0); }



    #[test]
    fn kilometers_to_hectometers() {
        let km = KiloM(50.0);
        assert_eq!(HectoM(km.0 * 10.0), km.into());
    }

    #[test]
    fn kilometers_to_decameters() {
        let km = KiloM(50.0);
        assert_eq!(DecaM(km.0 * 100.0), km.into());
    }

    #[test]
    fn kilometers_to_meters() {
        let km = KiloM(50.0);
        assert_eq!(M(km.0 * 1000.0), km.into());
    }

    #[test]
    fn kilometers_to_decimeters() {
        let km = KiloM(50.0);
        assert_eq!(DeciM(km.0 * 10_000.0), km.into());
    }

    #[test]
    fn kilometers_to_centimeters() {
        let km = KiloM(50.0);
        assert_eq!(CentiM(km.0 * 100_000.0), km.into());
    }

    #[test]
    fn kilometers_to_millimeters() {
        let km = KiloM(50.0);
        assert_eq!(MilliM(km.0 * 1_000_000.0), km.into());
    }

    #[test]
    fn kilometers_to_micrometers() {
        let km = KiloM(50.0);
        assert_eq!(MicroM(km.0 * 1_000_000_000.0), km.into());
    }

    #[test]
    fn kilometers_to_nanometers() {
        let km = KiloM(50.0);
        assert_eq!(NanoM(km.0 * 1_000_000_000_000.0), km.into());
    }




    #[test]
    fn hectometers_to_kilometers() {
        let hm = HectoM(50.0);
        assert_eq!(KiloM(hm.0 / 10.0), hm.into());
    }

    #[test]
    fn hectometers_to_decameters() {
        let hm = HectoM(50.0);
        assert_eq!(DecaM(hm.0 * 10.0), hm.into());
    }

    #[test]
    fn hectometers_to_meters() {
        let hm = HectoM(50.0);
        assert_eq!(M(hm.0 * 100.0), hm.into());
    }

    #[test]
    fn hectometers_to_decimeters() {
        let hm = HectoM(50.0);
        assert_eq!(DeciM(hm.0 * 1_000.0), hm.into());
    }

    #[test]
    fn hectometers_to_centimeters() {
        let hm = HectoM(50.0);
        assert_eq!(CentiM(hm.0 * 10_000.0), hm.into());
    }

    #[test]
    fn hectometers_to_millimeters() {
        let hm = HectoM(50.0);
        assert_eq!(MilliM(hm.0 * 100_000.0), hm.into());
    }

    #[test]
    fn hectometers_to_micrometers() {
        let hm = HectoM(50.0);
        assert_eq!(MicroM(hm.0 * 100_000_000.0), hm.into());
    }

    #[test]
    fn hectometers_to_nanometers() {
        let hm = HectoM(50.0);
        assert_eq!(NanoM(hm.0 * 100_000_000_000.0), hm.into());
    }



    #[test]
    fn decameters_to_kilometers() {
        let dam = DecaM(50.0);
        assert_eq!(KiloM(dam.0 / 100.0), dam.into());
    }

    #[test]
    fn decameters_to_hectometers() {
        let dam = DecaM(50.0);
        assert_eq!(HectoM(dam.0 / 10.0), dam.into());
    }

    #[test]
    fn decameters_to_meters() {
        let dam = DecaM(50.0);
        assert_eq!(M(dam.0 * 10.0), dam.into());
    }

    #[test]
    fn decameters_to_decimeters() {
        let dam = DecaM(50.0);
        assert_eq!(DeciM(dam.0 * 100.0), dam.into());
    }

    #[test]
    fn decameters_to_centimeters() {
        let dam = DecaM(50.0);
        assert_eq!(CentiM(dam.0 * 1_000.0), dam.into());
    }

    #[test]
    fn decameters_to_millimeters() {
        let dam = DecaM(50.0);
        assert_eq!(MilliM(dam.0 * 10_000.0), dam.into());
    }

    #[test]
    fn decameters_to_micrometers() {
        let dam = DecaM(50.0);
        assert_eq!(MicroM(dam.0 * 10_000_000.0), dam.into());
    }

    #[test]
    fn decameters_to_nanometers() {
        let dam = DecaM(50.0);
        assert_eq!(NanoM(dam.0 * 10_000_000_000.0), dam.into());
    }



    #[test]
    fn meters_to_kilometers() {
        let m = M(50.0);
        assert_eq!(KiloM(m.0 / 1000.0), m.into());
    }

    #[test]
    fn meters_to_hectometers() {
        let m = M(50.0);
        assert_eq!(HectoM(m.0 / 100.0), m.into());
    }

    #[test]
    fn meters_to_decameters() {
        let m = M(50.0);
        assert_eq!(DecaM(m.0 / 10.0), m.into());
    }

    #[test]
    fn meters_to_decimeters() {
        let m = M(50.0);
        assert_eq!(DeciM(m.0 * 10.0), m.into());
    }

    #[test]
    fn meters_to_centimeters() {
        let m = M(50.0);
        assert_eq!(CentiM(m.0 * 100.0), m.into());
    }

    #[test]
    fn meters_to_millimeters() {
        let m = M(50.0);
        assert_eq!(MilliM(m.0 * 1_000.0), m.into());
    }

    #[test]
    fn meters_to_micrometers() {
        let m = M(50.0);
        assert_eq!(MicroM(m.0 * 1_000_000.0), m.into());
    }

    #[test]
    fn meters_to_nanometers() {
        let m = M(50.0);
        assert_eq!(NanoM(m.0 * 1_000_000_000.0), m.into());
    }



    #[test]
    fn decimeters_to_kilometers() {
        let dm = DeciM(50.0);
        assert_eq!(KiloM(dm.0 / 10_000.0), dm.into());
    }

    #[test]
    fn decimeters_to_hectometers() {
        let dm = DeciM(50.0);
        assert_eq!(HectoM(dm.0 / 1_000.0), dm.into());
    }

    #[test]
    fn decimeters_to_decameters() {
        let dm = DeciM(50.0);
        assert_eq!(DecaM(dm.0 / 100.0), dm.into());
    }

    #[test]
    fn decimeters_to_meters() {
        let dm = DeciM(50.0);
        assert_eq!(M(dm.0 / 10.0), dm.into());
    }

    #[test]
    fn decimeters_to_centimeters() {
        let dm = DeciM(50.0);
        assert_eq!(CentiM(dm.0 * 10.0), dm.into());
    }

    #[test]
    fn decimeters_to_millimeters() {
        let dm = DeciM(50.0);
        assert_eq!(MilliM(dm.0 * 100.0), dm.into());
    }

    #[test]
    fn decimeters_to_micrometers() {
        let dm = DeciM(50.0);
        assert_eq!(MicroM(dm.0 * 100_000.0), dm.into());
    }

    #[test]
    fn decimeters_to_nanometers() {
        let dm = DeciM(50.0);
        assert_eq!(NanoM(dm.0 * 100_000_000.0), dm.into());
    }


    #[test]
    fn centimeters_to_kilometers() {
        let cm = CentiM(50.0);
        assert_eq!(KiloM(cm.0 / 100_000.0), cm.into());
    }

    #[test]
    fn centimeters_to_hectometers() {
        let cm = CentiM(50.0);
        assert_eq!(HectoM(cm.0 / 10_000.0), cm.into());
    }

    #[test]
    fn centimeters_to_decameters() {
        let cm = CentiM(50.0);
        assert_eq!(DecaM(cm.0 / 1_000.0), cm.into());
    }

    #[test]
    fn centimeters_to_meters() {
        let cm = CentiM(50.0);
        assert_eq!(M(cm.0 / 100.0), cm.into());
    }

    #[test]
    fn centimeters_to_decimeters() {
        let cm = CentiM(50.0);
        assert_eq!(DeciM(cm.0 / 10.0), cm.into());
    }

    #[test]
    fn centimeters_to_millimeters() {
        let cm = CentiM(50.0);
        assert_eq!(MilliM(cm.0 * 10.0), cm.into());
    }

    #[test]
    fn centimeters_to_micrometers() {
        let cm = CentiM(50.0);
        assert_eq!(MicroM(cm.0 * 10_000.0), cm.into());
    }

    #[test]
    fn centimeters_to_nanometers() {
        let cm = CentiM(50.0);
        assert_eq!(NanoM(cm.0 * 10_000_000.0), cm.into());
    }



    #[test]
    fn millimeters_to_kilometers() {
        let mm = MilliM(50.0);
        assert_eq!(KiloM(mm.0 / 1_000_000.0), mm.into());
    }

    #[test]
    fn millimeters_to_hectometers() {
        let mm = MilliM(50.0);
        assert_eq!(HectoM(mm.0 / 100_000.0), mm.into());
    }

    #[test]
    fn millimeters_to_decameters() {
        let mm = MilliM(50.0);
        assert_eq!(DecaM(mm.0 / 10_000.0), mm.into());
    }

    #[test]
    fn millimeters_to_meters() {
        let mm = MilliM(50.0);
        assert_eq!(M(mm.0 / 1_000.0), mm.into());
    }

    #[test]
    fn millimeters_to_decimeters() {
        let mm = MilliM(50.0);
        assert_eq!(DeciM(mm.0 / 100.0), mm.into());
    }

    #[test]
    fn millimeters_to_centimeters() {
        let mm = MilliM(50.0);
        assert_eq!(CentiM(mm.0 / 10.0), mm.into());
    }

    #[test]
    fn millimeters_to_micrometers() {
        let mm = MilliM(50.0);
        assert_eq!(MicroM(mm.0 * 1_000.0), mm.into());
    }

    #[test]
    fn millimeters_to_nanometers() {
        let mm = MilliM(50.0);
        assert_eq!(NanoM(mm.0 * 1_000_000.0), mm.into());
    }



    #[test]
    fn micrometers_to_kilometers() {
        let um = MicroM(50.0);
        assert_eq!(KiloM(um.0 / 1_000_000_000.0), um.into());
    }

    #[test]
    fn micrometers_to_hectometers() {
        let um = MicroM(50.0);
        assert_eq!(HectoM(um.0 / 100_000_000.0), um.into());
    }

    #[test]
    fn micrometers_to_decameters() {
        let um = MicroM(50.0);
        assert_eq!(DecaM(um.0 / 10_000_000.0), um.into());
    }

    #[test]
    fn micrometers_to_meters() {
        let um = MicroM(50.0);
        assert_eq!(M(um.0 / 1_000_000.0), um.into());
    }

    #[test]
    fn micrometers_to_decimeters() {
        let um = MicroM(50.0);
        assert_eq!(DeciM(um.0 / 100_000.0), um.into());
    }

    #[test]
    fn micrometers_to_centimeters() {
        let um = MicroM(50.0);
        assert_eq!(CentiM(um.0 / 10_000.0), um.into());
    }

    #[test]
    fn micrometers_to_millimeters() {
        let um = MicroM(50.0);
        assert_eq!(MilliM(um.0 / 1_000.0), um.into());
    }

    #[test]
    fn micrometers_to_nanometers() {
        let um = MicroM(50.0);
        assert_eq!(NanoM(um.0 * 1_000.0), um.into());
    }



    #[test]
    fn nanometers_to_kilometers() {
        let nm = NanoM(50.0);
        assert_eq!(KiloM(nm.0 / 1_000_000_000_000.0), nm.into());
    }

    #[test]
    fn nanometers_to_hectometers() {
        let nm = NanoM(50.0);
        assert_eq!(HectoM(nm.0 / 100_000_000_000.0), nm.into());
    }

    #[test]
    fn nanometers_to_decameters() {
        let nm = NanoM(50.0);
        assert_eq!(DecaM(nm.0 / 10_000_000_000.0), nm.into());
    }

    #[test]
    fn nanometers_to_meters() {
        let nm = NanoM(50.0);
        assert_eq!(M(nm.0 / 1_000_000_000.0), nm.into());
    }

    #[test]
    fn nanometers_to_decimeters() {
        let nm = NanoM(50.0);
        assert_eq!(DeciM(nm.0 / 100_000_000.0), nm.into());
    }

    #[test]
    fn nanometers_to_centimeters() {
        let nm = NanoM(50.0);
        assert_eq!(CentiM(nm.0 / 10_000_000.0), nm.into());
    }

    #[test]
    fn nanometers_to_millimeters() {
        let nm = NanoM(50.0);
        assert_eq!(MilliM(nm.0 / 1_000_000.0), nm.into());
    }

    #[test]
    fn nanometers_to_micrometers() {
        let nm = NanoM(50.0);
        assert_eq!(MicroM(nm.0 / 1_000.0), nm.into());
    }



}
