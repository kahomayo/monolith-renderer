use crate::noise::FractalNoise;
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

    pub fn sample2d(&self, x: i32, z: i32, res_x: usize, res_z: usize) -> Box<[f64]> {
        self.sample3d(x, 0, z, res_x, 1, res_z)
    }

    pub fn sample3d(
        &self,
        x: i32,
        y: i32,
        z: i32,
        res_x: usize,
        res_y: usize,
        res_z: usize,
    ) -> Box<[f64]> {
        self.noise
            .begin_sampling(
                x,
                y,
                z,
                res_x,
                res_y,
                res_z,
                self.scale_x_z,
                self.scale_y,
                self.scale_x_z,
            )
            .sample_all()
    }
}
