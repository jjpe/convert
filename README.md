Convert
====

Convert is a library to *safely*  manipulate and convert between various
[units of measurement](https://en.wikipedia.org/wiki/Units_of_measurement).
Currently supported are:

* Length: `kilometer`, `hectometer`, `decameter`, `meter`, `decimeter`,
  `centimeter`, `millimeter`, `micrometer`, `nanometer`
* Time: `day`, `hour`, `minute`, `second`, `millisecond`, `microsecond`,
  `nanosecond`
* Speed: `meters / second`, `kilometers / hour`

Caveats
----
This library is currently not finished and I do not claim it to be
production-ready in any way.
In particular:

1. The library does not perform any overflow checking on top of what Rust
   itself already does.
2. Most units utilize `f64` internally, which is a useful but limited type
   for representing decimals. This is because when the delta between 2 `f64`
   numbers is small enough there will be rounding errors. The internal unit
   type will be changed to `f128` once Rust supports it to allow for decimals
   of smaller magnitude without bumping into rounding errors.
   In addition, it is also somewhat limited in the magnitude of the non-decimal
   digits that can be represented. To remedy these issues, use units of an
   appropriate scale to perform calculations.
