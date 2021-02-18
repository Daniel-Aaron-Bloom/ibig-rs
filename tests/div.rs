use ibig::prelude::*;

#[test]
fn test_div_rem_ubig() {
    let test_cases = [
        (ubig!(331), ubig!(10), ubig!(33), ubig!(1)),
        (ubig!(17),
         ubig!(_0x987987123984798abbcc213789723948792138479837492837498cc),
         ubig!(0),
         ubig!(17)
        ),
        (ubig!(_0x987987123984798abbcc213789723948792138479837492837498cc),
         ubig!(_0x1234),
         ubig!(_0x86054c502f0a4e43e2d0de91f1029d251ce67bbdb88dc3edbb40),
         ubig!(_0xfcc)
        ),
        // Random 500-bit by random 250-bit.
        (
         ubig!(_0x2b8f1bb75f1ca5bf3400549a663d503d298da7f53942cd3c5c6a1bc50598d091e8ca30896413783e9b001572e28808c4dc9598bdd17ef3ce35b40e0368b60),
         ubig!(_0x3e880309f5e48d145337aae47694a74f2860db8e49665f03978f1b11665dc80),
         ubig!(_0xb254145d6f736c22ed5fca6a41f4c883a59fc32c638710758bb50fa532b31f),
         ubig!(_0x195afda8e35e347c65ed01409c73d1c820ed78a87e83cf6cfdad1a25fb357e0),
        ),
        (
         ubig!(_0x3e880309f5e48d145337aae47694a74f2860db8e49665f03978f1b11665dc80),
         ubig!(_0x2b8f1bb75f1ca5bf3400549a663d503d298da7f53942cd3c5c6a1bc50598d091e8ca30896413783e9b001572e28808c4dc9598bdd17ef3ce35b40e0368b60),
         ubig!(0),
         ubig!(_0x3e880309f5e48d145337aae47694a74f2860db8e49665f03978f1b11665dc80),
        ),
        // 3^300 - 1 by 3^150
        (
         ubig!(_0xb39cfff485a5dbf4d6aae030b91bfb0ec6bba389cd8d7f85bba3985c19c5e24e40c543a123c6e028a873e9e3874e1b4623a44be39b34e67dc5c2670),
         ubig!(_0x359ba2b98ca11d6864a331b45ae7114c01ffbdcf60cc16e692fb63c6e219),
         ubig!(_0x359ba2b98ca11d6864a331b45ae7114c01ffbdcf60cc16e692fb63c6e218),
         ubig!(_0x359ba2b98ca11d6864a331b45ae7114c01ffbdcf60cc16e692fb63c6e218),
        ),
        // 7^70-1 by 7^35
        (
         ubig!(_0x16dc8782276b9f7addf9768f33c8007ce903866a4546c1a190),
         ubig!(_0x4c8077a58a0a8cb7c24960e57),
         ubig!(_0x4c8077a58a0a8cb7c24960e56),
         ubig!(_0x4c8077a58a0a8cb7c24960e56),
        ),
    ];

    for (a, b, q, r) in &test_cases {
        let qr = (q.clone(), r.clone());

        assert_eq!(a / b, *q);
        assert_eq!(a.clone() / b, *q);
        assert_eq!(a / b.clone(), *q);
        assert_eq!(a.clone() / b.clone(), *q);

        let mut x = a.clone();
        x /= b;
        assert_eq!(x, *q);

        let mut x = a.clone();
        x /= b.clone();
        assert_eq!(x, *q);

        assert_eq!(a % b, *r);
        assert_eq!(a.clone() % b, *r);
        assert_eq!(a % b.clone(), *r);
        assert_eq!(a.clone() % b.clone(), *r);

        let mut x = a.clone();
        x %= b;
        assert_eq!(x, *r);

        let mut x = a.clone();
        x %= b.clone();
        assert_eq!(x, *r);

        assert_eq!(a.div_rem(b), qr);
        assert_eq!(a.clone().div_rem(b), qr);
        assert_eq!(a.div_rem(b.clone()), qr);
        assert_eq!(a.clone().div_rem(b.clone()), qr);

        assert_eq!(a.div_euclid(b), *q);
        assert_eq!(a.clone().div_euclid(b), *q);
        assert_eq!(a.div_euclid(b.clone()), *q);
        assert_eq!(a.clone().div_euclid(b.clone()), *q);

        assert_eq!(a.rem_euclid(b), *r);
        assert_eq!(a.clone().rem_euclid(b), *r);
        assert_eq!(a.rem_euclid(b.clone()), *r);
        assert_eq!(a.clone().rem_euclid(b.clone()), *r);

        assert_eq!(a.div_rem_euclid(b), qr);
        assert_eq!(a.clone().div_rem_euclid(b), qr);
        assert_eq!(a.div_rem_euclid(b.clone()), qr);
        assert_eq!(a.clone().div_rem_euclid(b.clone()), qr);
    }
}

#[test]
#[should_panic]
fn test_divide_by_0_ubig() {
    let _ = ubig!(5) / ubig!(0);
}

#[test]
fn test_div_rem_ibig() {
    for a in -20i8..=20i8 {
        for b in -20i8..=20i8 {
            if b == 0 {
                continue;
            }

            let a_big: IBig = a.into();
            let b_big: IBig = b.into();
            let q: IBig = (a / b).into();
            let r: IBig = (a % b).into();
            let qr = (q.clone(), r.clone());

            assert_eq!(a_big.clone() / b_big.clone(), q);
            assert_eq!(&a_big / b_big.clone(), q);
            assert_eq!(a_big.clone() / &b_big, q);
            assert_eq!(&a_big / &b_big, q);

            let mut x = a_big.clone();
            x /= b_big.clone();
            assert_eq!(x, q);

            let mut x = a_big.clone();
            x /= &b_big;
            assert_eq!(x, q);

            assert_eq!(a_big.clone() % b_big.clone(), r);
            assert_eq!(&a_big % b_big.clone(), r);
            assert_eq!(a_big.clone() % &b_big, r);
            assert_eq!(&a_big % &b_big, r);

            let mut x = a_big.clone();
            x %= b_big.clone();
            assert_eq!(x, r);

            let mut x = a_big.clone();
            x %= &b_big;
            assert_eq!(x, r);

            assert_eq!(a_big.clone().div_rem(b_big.clone()), qr);
            assert_eq!((&a_big).div_rem(b_big.clone()), qr);
            assert_eq!(a_big.clone().div_rem(&b_big), qr);
            assert_eq!((&a_big).div_rem(&b_big), qr);
        }
    }
}

#[test]
fn test_div_rem_euclid_ibig() {
    for a in -20i8..=20i8 {
        for b in -20i8..=20i8 {
            if b == 0 {
                continue;
            }

            let a_big: IBig = a.into();
            let b_big: IBig = b.into();
            let q: IBig = a.div_euclid(b).into();
            let r: IBig = a.rem_euclid(b).into();
            let qr = (q.clone(), r.clone());

            assert_eq!(a_big.clone().div_euclid(b_big.clone()), q);
            assert_eq!((&a_big).div_euclid(b_big.clone()), q);
            assert_eq!(a_big.clone().div_euclid(&b_big), q);
            assert_eq!((&a_big).div_euclid(&b_big), q);

            assert_eq!(a_big.clone().rem_euclid(b_big.clone()), r);
            assert_eq!((&a_big).rem_euclid(b_big.clone()), r);
            assert_eq!(a_big.clone().rem_euclid(&b_big), r);
            assert_eq!((&a_big).rem_euclid(&b_big), r);

            assert_eq!(a_big.clone().div_rem_euclid(b_big.clone()), qr);
            assert_eq!((&a_big).div_rem_euclid(b_big.clone()), qr);
            assert_eq!(a_big.clone().div_rem_euclid(&b_big), qr);
            assert_eq!((&a_big).div_rem_euclid(&b_big), qr);
        }
    }
}

#[test]
#[should_panic]
fn test_divide_by_0_ibig() {
    let _ = ibig!(5) / ibig!(0);
}