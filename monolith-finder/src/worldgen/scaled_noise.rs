use crate::coord::{SamplePos2D, SamplePos3D};
use crate::noise::SampleJobImpl;
use crate::noise::{FractalNoise, SamplingCuboid};
use java_rand::Random;

#[derive(Debug)]
pub struct ScaledNoise {
    noise: FractalNoise,
    scale_x_z: f64,
    scale_y: f64,
}

impl ScaledNoise {
    pub fn new(random: &mut Random, octaves: usize, scale_x_z: f64, scale_y: f64) -> Self {
        Self {
            noise: FractalNoise::with_random_octaves(random, octaves),
            scale_x_z,
            scale_y,
        }
    }

    pub fn discard_noise(random: &mut Random, octaves: usize) {
        let _ = FractalNoise::with_random_octaves(random, octaves);
    }

    pub fn sample2d(
        &self,
        pos: SamplePos2D,
        res_x: usize,
        res_z: usize,
    ) -> SampleJobImpl<Box<[f64]>> {
        self.sample3d(pos.at_y(0), res_x, 1, res_z)
    }

    pub fn sample3d(
        &self,
        start_pos: SamplePos3D,
        res_x: usize,
        res_y: usize,
        res_z: usize,
    ) -> SampleJobImpl<Box<[f64]>> {
        self.noise.begin_sampling(SamplingCuboid {
            start_pos,
            x_extent: res_x,
            y_extent: res_y,
            z_extent: res_z,
            x_scale: self.scale_x_z,
            y_scale: self.scale_y,
            z_scale: self.scale_x_z,
        })
    }
}
