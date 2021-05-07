use crate::noise::PerlinNoise;

use java_rand::Random;
use std::borrow::{Borrow, BorrowMut};
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
    ) -> SampleJobImpl<Box<[f64]>> {
        SampleJobImpl {
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

pub trait SamplingJob {
    fn sample_once(&mut self);
    fn status(&self) -> SamplingStatus;
    fn results(&self) -> &[f64];
    fn results_mut(&mut self) -> &mut [f64];
    fn remaining_steps(&self) -> usize;
    fn remaining_variation(&self) -> f64;

    fn is_done(&self) -> bool {
        self.status() == SamplingStatus::Done
    }

    fn sample_n(&mut self, n: usize) {
        for _ in 0..n {
            self.sample_once()
        }
    }
}

pub struct SampleJobImpl<'a, TResult: DerefMut<Target = [f64]>> {
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

impl<'a, TResult: DerefMut<Target = [f64]>> SampleJobImpl<'a, TResult> {
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

    pub fn into_results(self) -> TResult {
        self.results
    }

    pub fn sample_all(mut self) -> TResult {
        while !self.is_done() {
            self.sample_once();
        }
        self.results
    }
}

impl<'a, TResult: DerefMut<Target = [f64]>> SamplingJob for SampleJobImpl<'a, TResult> {
    fn sample_once(&mut self) {
        if let Some(perlin_noise) = self.noise.get(self.applied_noises) {
            let inv_intensity = 0.5_f64.powi(self.remaining_steps() as i32 - 1);
            perlin_noise.sample(
                self.results.borrow_mut(),
                self.x,
                self.y,
                self.z,
                self.res_x,
                self.res_y,
                self.res_z,
                self.scale_x * inv_intensity,
                self.scale_y * inv_intensity,
                self.scale_z * inv_intensity,
                inv_intensity,
            );
            self.applied_noises += 1;
        }
    }

    fn status(&self) -> SamplingStatus {
        match self.applied_noises {
            0 => SamplingStatus::NotStarted,
            x if x == self.noise.len() => SamplingStatus::Done,
            _ => SamplingStatus::Started,
        }
    }

    fn results(&self) -> &[f64] {
        self.results.borrow()
    }

    fn results_mut(&mut self) -> &mut [f64] {
        self.results.borrow_mut()
    }

    fn remaining_steps(&self) -> usize {
        self.noise.len() - self.applied_noises
    }

    fn remaining_variation(&self) -> f64 {
        (2.0_f64.powi(self.remaining_steps() as i32) - 1.0) * PerlinNoise::RESULT_RANGE
    }
}

#[cfg(test)]
mod tests {
    use crate::noise::fractal::{FractalNoise, SamplingJob};
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

    #[test]
    fn remaining_variation_0_of_1() {
        RemainingVariationTest {
            octaves: 1,
            samples: 0,
            expected: 1.0,
        }
        .run()
    }

    #[test]
    fn remaining_variation_1_of_1() {
        RemainingVariationTest {
            octaves: 1,
            samples: 1,
            expected: 0.0,
        }
        .run()
    }

    #[test]
    fn remaining_variation_0_of_2() {
        RemainingVariationTest {
            octaves: 2,
            samples: 0,
            expected: 3.0,
        }
        .run()
    }

    struct RemainingVariationTest {
        octaves: usize,
        samples: usize,
        expected: f64,
    }

    impl RemainingVariationTest {
        pub fn run(self) {
            let noise = FractalNoise::with_random_octaves(&mut Random::new(0), self.octaves);
            let mut job = noise.begin_sampling(0, 0, 0, 1, 1, 1, 1.0, 1.0, 1.0);

            for _ in 0..self.samples {
                job.sample_once();
            }

            assert_eq!(job.remaining_variation(), self.expected);
        }
    }
}
