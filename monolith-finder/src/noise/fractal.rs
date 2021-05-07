use crate::noise::PerlinNoise;

use java_rand::Random;

#[derive(Debug)]
pub struct FractalNoise {
    octaves: Box<[PerlinNoise]>,
}

impl FractalNoise {
    pub fn with_random_octaves(random: &mut Random, count: usize) -> Self {
        let mut x = Vec::with_capacity(count);
        for _ in 0..count {
            x.push(PerlinNoise::with_random_permutations(random));
        }
        Self {
            octaves: x.into_boxed_slice(),
        }
    }

    pub fn sample(
        &self,
        x: i32,
        y: i32,
        z: i32,
        res_x: usize,
        res_y: usize,
        res_z: usize,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> Box<[f64]> {
        let mut results = vec![0.0; res_x * res_y * res_z].into_boxed_slice();
        let mut inv_intensity = 1.0;
        for i in 0..self.octaves.len() {
            self.octaves[i].sample(
                &mut results,
                x,
                y,
                z,
                res_x,
                res_y,
                res_z,
                scale_x * inv_intensity,
                scale_y * inv_intensity,
                scale_z * inv_intensity,
                inv_intensity,
            );
            inv_intensity /= 2.0;
        }

        results
    }
}
