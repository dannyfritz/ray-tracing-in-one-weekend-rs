use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy)]
pub(crate) struct Vec3(f32, f32, f32);

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 { *self }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vec3 {{ x: {}, y: {}, z: {} }}",
            self.x(),
            self.y(),
            self.z()
        )
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
    pub fn r(&self) -> f32 {
        self.x()
    }
    pub fn g(&self) -> f32 {
        self.y()
    }
    pub fn b(&self) -> f32 {
        self.z()
    }
    pub fn length(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }
    pub fn squared_length(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        v / v.length()
    }
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
    }
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(
            v1.y() * v2.z() - v1.z() * v2.y(),
            -v1.x() * v2.z() - v1.z() * v2.x(),
            v1.x() * v2.y() - v1.y() * v2.x(),
        )
    }
}

impl<'a> Add for &'a Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl<'a> Sub for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.0 -= other.x();
        self.1 -= other.y();
        self.2 -= other.z();
    }
}

impl<'a> Mul for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.0 *= other.x();
        self.1 *= other.y();
        self.2 *= other.z();
    }
}
impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}
impl<'a> Mul<&'a Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl<'a> Div for &'a Vec3 {
    type Output = Vec3;
    fn div(self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}
impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}
impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.0 /= other.x();
        self.1 /= other.y();
        self.2 /= other.z();
    }
}
impl<'a> Div<f32> for &'a Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}
