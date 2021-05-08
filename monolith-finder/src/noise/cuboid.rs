use crate::coord::SamplePos3D;

/// Describes a section of 3D noise to sample.
pub struct SamplingCuboid {
    pub start_pos: SamplePos3D,
    pub x_extent: usize,
    pub y_extent: usize,
    pub z_extent: usize,
    pub x_scale: f64,
    pub y_scale: f64,
    pub z_scale: f64,
}

impl SamplingCuboid {
    /// Produces a copy of this cuboid that is scaled by the given factor.
    /// Note that only the `*_scale` fields are affected by this transformation.
    pub fn scale_all(&self, factor: f64) -> Self {
        SamplingCuboid {
            x_scale: self.x_scale * factor,
            y_scale: self.y_scale * factor,
            z_scale: self.z_scale * factor,
            ..*self
        }
    }

    pub fn scaled_x(&self, index: usize) -> f64 {
        assert!(index < self.x_extent);
        (self.start_pos.x + index as i32) as f64 * self.x_scale
    }

    pub fn scaled_y(&self, index: usize) -> f64 {
        assert!(index < self.y_extent);
        (self.start_pos.y + index as i32) as f64 * self.y_scale
    }

    pub fn scaled_z(&self, index: usize) -> f64 {
        assert!(index < self.z_extent);
        (self.start_pos.z + index as i32) as f64 * self.z_scale
    }

    /// Returns the number of points in the cuboid.
    pub fn len(&self) -> usize {
        self.x_extent * self.y_extent * self.z_extent
    }
}
