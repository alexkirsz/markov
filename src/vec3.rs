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

impl From<(f32, f32, f32)> for Vec3 {
    fn from(arr: (f32, f32, f32)) -> Vec3 {
        Vec3 {
            x: arr.0,
            y: arr.1,
            z: arr.2,
        }
    }
}
