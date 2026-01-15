use std::ops;

use crate::util::rand::PCG32RNG;

pub type Point3 = Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn len_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_squared())
    }

    pub fn random(rng: &mut PCG32RNG) -> Self {
        Self {
            e: [rng.random_f64(), rng.random_f64(), rng.random_f64()],
        }
    }

    pub fn random_bounded(rng: &mut PCG32RNG, min: f64, max: f64) -> Self {
        Self {
            e: [
                rng.random_bounded_f64(min, max),
                rng.random_bounded_f64(min, max),
                rng.random_bounded_f64(min, max),
            ],
        }
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.e.iter().all(|v| v.abs() < EPS)
    }

    #[inline]
    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - 2.0 * Self::dot(v, n) * *n
    }

    #[inline]
    pub fn refract(uv: &Self, n: &Self, ri: f64) -> Self {
        let cos_theta = Self::dot(&-*uv, n).min(1.0);
        let r_out_perp = ri * (*uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * *n;

        r_out_perp + r_out_parallel
    }

    #[inline]
    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    #[inline]
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    #[inline]
    pub fn random_unit_vector(rng: &mut PCG32RNG) -> Self {
        let mut p: Self;
        loop {
            p = Self::random_bounded(rng, -1.0, 1.0);
            let lensq = p.len_squared();
            if (1e-160 < lensq) && (lensq <= 1.0) {
                // reject very small vlaues as they can lead to
                // infinities when squared
                return p / lensq.sqrt();
            }
        }
    }

    #[inline]
    pub fn random_on_hemisphere(rng: &mut PCG32RNG, normal: &Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector(rng);
        if Self::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline]
    pub fn random_in_unit_disk(rng: &mut PCG32RNG) -> Self {
        loop {
            let p = Vec3::new(
                rng.random_bounded_f64(-1.0, 1.0),
                rng.random_bounded_f64(-1.0, 1.0),
                0.0,
            );

            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
