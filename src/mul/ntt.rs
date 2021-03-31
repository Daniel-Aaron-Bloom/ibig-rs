//! Number-theoretic multiplication algorithm.

use crate::{
    arch::{
        ntt::{MAX_ORDER, PRIMES},
        word::Word,
    },
    modular::modulo_ring::ModuloRingSmall,
};

/// The number of prime factors in the ring.
pub(crate) const NUM_PRIMES: usize = 3;

/// A prime to be used for the number-theoretic transform.
pub(crate) struct Prime {
    /// A prime of the form k * 2^MAX_ORDER + 1.
    pub(crate) prime: Word,
    /// max_order_root has order 2^MAX_ORDER.
    pub(crate) max_order_root: Word,
}

/// Factor fields of the three-prime ring.
const FIELDS: [ModuloRingSmall; NUM_PRIMES] = [
    ModuloRingSmall::new(PRIMES[0].prime),
    ModuloRingSmall::new(PRIMES[1].prime),
    ModuloRingSmall::new(PRIMES[2].prime),
];

/// An element of the three-prime ring.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RingElement {
    normalized: [Word; NUM_PRIMES],
}

impl From<Word> for RingElement {
    /// Convert a `Word` to `RingElement`.

    fn from(x: Word) -> RingElement {
        RingElement {
            normalized: [
                FIELDS[0].normalize_word(x),
                FIELDS[1].normalize_word(x),
                FIELDS[2].normalize_word(x),
            ],
        }
    }
}

impl RingElement {
    const fn zero() -> RingElement {
        RingElement {
            normalized: [0; NUM_PRIMES],
        }
    }

    const fn mul(&self, rhs: &RingElement) -> RingElement {
        RingElement {
            normalized: [
                FIELDS[0].mul_normalized(self.normalized[0], rhs.normalized[0]),
                FIELDS[1].mul_normalized(self.normalized[1], rhs.normalized[1]),
                FIELDS[2].mul_normalized(self.normalized[2], rhs.normalized[2]),
            ],
        }
    }

    const fn inverse(&self) -> RingElement {
        RingElement {
            normalized: [
                FIELDS[0].const_pow_normalized(self.normalized[0], PRIMES[0].prime - 2),
                FIELDS[1].const_pow_normalized(self.normalized[1], PRIMES[1].prime - 2),
                FIELDS[2].const_pow_normalized(self.normalized[2], PRIMES[2].prime - 2),
            ],
        }
    }
}

const MAX_ORDER_ROOT: RingElement = RingElement {
    normalized: [
        FIELDS[0].normalize_word(PRIMES[0].max_order_root),
        FIELDS[1].normalize_word(PRIMES[1].max_order_root),
        FIELDS[2].normalize_word(PRIMES[2].max_order_root),
    ],
};

type RootTable = [RingElement; MAX_ORDER as usize + 1];

/// ROOTS[order]^(2^order) = 1
#[allow(dead_code)]
static ROOTS: RootTable = generate_roots(MAX_ORDER_ROOT);

/// INVERSE_ROOTS[order]^(2^order) = 1
#[allow(dead_code)]
static INVERSE_ROOTS: RootTable = generate_roots(MAX_ORDER_ROOT.inverse());

const fn generate_roots(max_order_root: RingElement) -> RootTable {
    let mut table = [RingElement::zero(); MAX_ORDER as usize + 1];
    table[MAX_ORDER as usize] = max_order_root;
    generate_roots_prefix(MAX_ORDER as usize, table)
}

const fn generate_roots_prefix(n: usize, mut table: RootTable) -> RootTable {
    if n == 0 {
        table
    } else {
        table[n - 1] = table[n].mul(&table[n]);
        generate_roots_prefix(n - 1, table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let one: Word = 1;
        let one = RingElement::from(one);
        assert_eq!(MAX_ORDER_ROOT.inverse().mul(&MAX_ORDER_ROOT), one);
        assert_eq!(MAX_ORDER_ROOT.inverse().inverse(), MAX_ORDER_ROOT);
    }

    #[test]
    fn test_roots() {
        let one: Word = 1;
        let one = RingElement::from(one);
        assert_eq!(ROOTS[0], one);
        assert_ne!(ROOTS[1], one);
        assert_eq!(INVERSE_ROOTS[0], one);
        assert_ne!(INVERSE_ROOTS[1], one);
    }
}
