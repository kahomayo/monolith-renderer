use crate::coord::{SamplePos2D, SamplePos3D};
use crate::noise::SampleJobImpl;
use crate::noise::{FractalNoise, SamplingCuboid};
use core::ops::DerefMut;
use java_rand::Random;

#[derive(Debug)]
pub struct ScaledNoise<const I: usize> {
    noise: FractalNoise<I>,
    scale_x_z: f64,
    scale_y: f64,
}

impl<const I: usize> ScaledNoise<I> {
    pub fn new(random: &mut Random, scale_x_z: f64, scale_y: f64) -> Self {
        Self {
            noise: FractalNoise::with_random_octaves(random),
            scale_x_z,
            scale_y,
        }
    }

    pub fn discard_noise(random: &mut Random) {
        let _ = FractalNoise::<I>::with_random_octaves(random);
    }

    pub fn sample2d<T>(
        &self,
        pos: SamplePos2D,
        res_x: usize,
        res_z: usize,
        result_buf: T,
    ) -> SampleJobImpl<T>
    where
        T: DerefMut<Target = [f64]>,
    {
        self.sample3d(pos.at_y(0), res_x, 1, res_z, result_buf)
    }

    pub fn sample3d<T>(
        &self,
        start_pos: SamplePos3D,
        res_x: usize,
        res_y: usize,
        res_z: usize,
        result_buf: T,
    ) -> SampleJobImpl<T>
    where
        T: DerefMut<Target = [f64]>,
    {
        self.noise.begin_sampling_into(
            SamplingCuboid {
                start_pos,
                x_extent: res_x,
                y_extent: res_y,
                z_extent: res_z,
                x_scale: self.scale_x_z,
                y_scale: self.scale_y,
                z_scale: self.scale_x_z,
            },
            result_buf,
        )
    }
}
