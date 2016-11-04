//! This crate deals with physical units.
//! It's only a matter of time until other units get added, har har :)

/// Define one or more units.
macro_rules! defunit {
    ($name:ident( $($fields:ty),+ )) => {
        #[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
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

pub mod time;
pub mod length;
