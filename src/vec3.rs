use palette::Lab;
use std::ops::{Add, Div, Sub};
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(arr: [f32; 3]) -> Vec3 {
        Vec3 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl From<Lab> for Vec3 {
    fn from(arr: Lab) -> Vec3 {
        Vec3 {
            x: arr.l,
            y: arr.a,
            z: arr.b,
        }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(arr: (f32, f32, f32)) -> Vec3 {
        Vec3 {
            x: arr.0,
            y: arr.1,
            z: arr.2,
        }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, denominator: f32) -> Vec3 {
        Vec3::new(
            self.x / denominator,
            self.y / denominator,
            self.z / denominator,
        )
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, denominator: f32) -> Vec3 {
        Vec3::new(
            self.x / denominator,
            self.y / denominator,
            self.z / denominator,
        )
    }
}

impl std::iter::Sum<Vec3> for Vec3 {
    fn sum<I>(iter: I) -> Vec3
    where
        I: Iterator<Item = Vec3>,
    {
        iter.fold(Vec3::zero(), |acc, v| acc + v)
    }
}
