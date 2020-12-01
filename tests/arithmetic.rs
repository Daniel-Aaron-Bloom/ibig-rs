use ibig::prelude::*;

#[test]
fn test_neg() {
    assert_eq!(-ibig!(123), ibig!(-123));
    assert_eq!(-ibig!(-123), ibig!(123));
    assert_eq!(-ibig!(0), ibig!(0));

    assert_eq!(-&ibig!(123), ibig!(-123));
}

#[test]
fn test_abs() {
    assert_eq!(ibig!(123).abs(), ibig!(123));
    assert_eq!(ibig!(-123).abs(), ibig!(123));
}
