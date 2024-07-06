pub struct World {
    pub camera: Vec3,
    pub cubes: Vec<Cube>
}

pub struct Cube {
    pub position: Vec3,
    pub length: usize
}

pub struct Vec3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Vec3 {
    pub fn new(x: usize, y: usize, z: usize) -> Vec3 {
        Self {x, y, z}
    }
}
