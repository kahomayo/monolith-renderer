#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockPos2D {
    pub x: i32,
    pub z: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockPos3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SamplePos2D {
    pub x: i32,
    pub z: i32,
}

impl SamplePos2D {
    pub fn at_y(&self, y: i32) -> SamplePos3D {
        SamplePos3D {
            x: self.x,
            y,
            z: self.z,
        }
    }
}

impl From<BlockPos2D> for SamplePos2D {
    fn from(pos: BlockPos2D) -> Self {
        Self {
            x: pos.x / 4,
            z: pos.z / 4,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SamplePos3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
