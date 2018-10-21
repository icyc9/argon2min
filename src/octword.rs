use std::ops::{Add, BitXor, Mul};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct u64x2(pub u64, pub u64);

impl Add for u64x2 {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        u64x2(self.0.wrapping_add(r.0), self.1.wrapping_add(r.1))
    }
}

impl Mul for u64x2 {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        u64x2(self.0.wrapping_mul(r.0), self.1.wrapping_mul(r.1))
    }
}

impl BitXor for u64x2 {
    type Output = Self;
    fn bitxor(self, r: Self) -> u64x2 {
        u64x2(self.0 ^ r.0, self.1 ^ r.1)
    }
}

fn lo(n: u64) -> u64 {
    n & 0xffffffff
}

impl u64x2 {
    pub fn lower_mult(self, r: Self) -> Self {
        u64x2(lo(self.0) * lo(r.0), lo(self.1) * lo(r.1))
    }

    pub fn rotate_right(self, n: u32) -> Self {
        u64x2(self.0.rotate_right(n), self.1.rotate_right(n))
    }

    pub fn cross_swap(self, r: Self) -> (Self, Self) {
        let u64x2(v4, v5) = self;
        let u64x2(v6, v7) = r;
        (u64x2(v7, v4), u64x2(v5, v6))
    }
}

#[cfg(test)]
mod test {
    use super::u64x2;

    // XXX Use function call to workaround rust-lang/rust#33764.
    fn t0() -> u64x2 {
        u64x2(0xdeadbeef_01234567, 0xcafe3210_babe9932)
    }
    fn t1() -> u64x2 {
        u64x2(0x09990578_01234567, 0x1128f9a9_88e89448)
    }

    #[test]
    fn test_rotate_right() {
        for &s in [8 as u32, 16, 24, 32].iter() {
            assert_eq!(t0().rotate_right(s).0, t0().0.rotate_right(s));
            assert_eq!(t0().rotate_right(s).1, t0().1.rotate_right(s));
        }
    }

    #[test]
    fn test_lower_mult() {
        use super::lo;
        assert_eq!(t0().lower_mult(t1()), t1().lower_mult(t0()));
        assert_eq!(t0().lower_mult(t1()).0, lo(t0().0) * lo(t1().0));
        assert_eq!(t0().lower_mult(t1()).1, lo(t0().1) * lo(t1().1));
    }
}
