use std::{
    fmt,
    ops::{Add, Mul},
    simd::f64x2,
};

#[derive(Clone, Copy, Default)]
pub struct Complex {
    pub r: f64,
    pub i: f64,
}

impl Complex {
    #[inline(always)]
    pub fn magnitude(&self) -> f64 {
        self.r.hypot(self.i)
    }
}

impl Add for Complex {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        let [r, i] = f64x2::from_array([self.r, self.i])
            .add(f64x2::from_array([other.r, other.i]))
            .to_array();

        Self { r, i }
    }
}

impl Mul for Complex {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        let [r, i] = f64x2::splat(self.r)
            .mul(f64x2::from_array([other.r, other.i]))
            .add(f64x2::splat(self.i).mul(f64x2::from_array([-other.i, other.r])))
            .to_array();

        Self { r, i }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.i >= 0.0 {
            write!(f, "{0}+{1}i", self.r, self.i)
        } else {
            write!(f, "{0}{1}i", self.r, self.i)
        }
    }
}
