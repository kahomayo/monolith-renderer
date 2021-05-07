use crate::noise::PerlinNoise;

use java_rand::Random;
use std::borrow::BorrowMut;
use std::ops::DerefMut;

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
        x.reverse();
        Self {
            octaves: x.into_boxed_slice(),
        }
    }

    pub fn begin_sampling(
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
    ) -> SampleJob<Box<[f64]>> {
        SampleJob {
            noise: self.octaves.as_ref(),
            applied_noises: 0,
            results: vec![0.0; res_x * res_y * res_z].into_boxed_slice(),
            x,
            y,
            z,
            res_x,
            res_y,
            res_z,
            scale_x,
            scale_y,
            scale_z,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum SamplingStatus {
    NotStarted,
    Started,
    Done,
}

pub struct SampleJob<'a, TResult: DerefMut<Target = [f64]>> {
    noise: &'a [PerlinNoise],
    applied_noises: usize,
    results: TResult,
    x: i32,
    y: i32,
    z: i32,
    res_x: usize,
    res_y: usize,
    res_z: usize,
    scale_x: f64,
    scale_y: f64,
    scale_z: f64,
}

impl<'a, TResult: DerefMut<Target = [f64]>> SampleJob<'a, TResult> {
    pub fn new(
        noise: &'a [PerlinNoise],
        results: TResult,
        x: i32,
        y: i32,
        z: i32,
        res_x: usize,
        res_y: usize,
        res_z: usize,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> Self {
        assert_eq!(
            results.len(),
            res_x * res_y * res_z,
            "The results slice must exactly match the given dimensions"
        );

        Self {
            noise,
            applied_noises: 0,
            results,
            x,
            y,
            z,
            res_x,
            res_y,
            res_z,
            scale_x,
            scale_y,
            scale_z,
        }
    }

    pub fn sample_once(&mut self) {
        if let Some(perlin_noise) = self.noise.get(self.applied_noises) {
            let inv_intensity = f64::powi(2.0, self.applied_noises as i32);
            perlin_noise.sample(
                self.results.borrow_mut(),
                self.x,
                self.y,
                self.z,
                self.res_x,
                self.res_y,
                self.res_z,
                self.scale_x,
                self.scale_y,
                self.scale_z,
                inv_intensity,
            );
            self.applied_noises += 1;
        }
    }

    pub fn status(&self) -> SamplingStatus {
        match self.applied_noises {
            0 => SamplingStatus::NotStarted,
            x if x == self.noise.len() => SamplingStatus::Done,
            _ => SamplingStatus::Started,
        }
    }

    pub fn is_done(&self) -> bool {
        self.status() == SamplingStatus::Done
    }

    pub fn sample_all(mut self) -> TResult {
        while !self.is_done() {
            self.sample_once();
        }
        self.results
    }
}

#[cfg(test)]
mod tests {
    use crate::noise::fractal::FractalNoise;
    use assert_approx_eq::assert_approx_eq;
    use java_rand::Random;

    #[test]
    fn basic_data_matches() {
        let noise = FractalNoise::with_random_octaves(&mut Random::new(15), 16);
        let noises = noise
            .begin_sampling(15, 52, 6, 16, 4, 29, 0.512386, 198.1293, 9999.1283)
            .sample_all();
        const EXPECTED: f64 = 10828.95355391629;
        let actual = noises[592];

        assert_approx_eq!(actual, EXPECTED, 1E-8);
    }
}
