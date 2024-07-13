#[cfg(test)]
mod test {
    #[test]
    fn partial_eq_demo() {
        let a = 42.0;
        let b = 42.0;
        assert_eq!(a, b);

        let c = f64::NAN;
        let d = f64::NAN;
        assert_ne!(c, d);

        let e = f64::NAN;
        assert_ne!(e, e);
    }

    #[test]
    fn partial_eq_check() {
        let f = 3.14;

        // Double implements PartialEq trait but not Eq trait
        // In IEEE 754, NaN is not equal to itself because only the exponent part of NaN is specified as all 1s, and the mantissa part is not specified.
        // f = (-1)^sign x 1.mantissa x 2^exponent
        // is_eq(f);
        is_partial_eq(f);
    }

    fn is_eq<T: Eq>(f: T) {}
    fn is_partial_eq<T: PartialEq>(f: T) {}
}