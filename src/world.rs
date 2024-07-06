pub struct World {
    pub camera: Vec3,
    pub cubes: Vec<Cube>
}

pub struct Cube {
    pub position: Vec3,
    pub length: i32
}

pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vec3 {
        Self {x, y, z}
    }
}
