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

    pub fn cross_z(v0: &Vec3, v1: &Vec3, p: &Vec3) -> i32 {
        //(v1x−v0x)(py−v0y)−(v1y−v0y)(px−v0x)
        (v1.x - v0.x) * (p.y - v0.y) - (v1.y - v0.y)*(p.x - v0.x)
    }
}

#[derive(Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Self {x, y, z}
    }

    pub fn add(&self, v: &Vec3f) -> Vec3f {
        return Vec3f::new(self.x + v.x, self.y + v.y, self.z + v.z);
    }

    pub fn sub(&self, v: &Vec3f) -> Vec3f {
        return Vec3f::new(self.x - v.x, self.y - v.y, self.z - v.z);
    }    
}
