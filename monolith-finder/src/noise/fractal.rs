use crate::noise::PerlinNoise;

use crate::noise::cuboid::SamplingCuboid;
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

    pub fn begin_sampling(&self, cuboid: SamplingCuboid) -> SampleJobImpl<Box<[f64]>> {
        SampleJobImpl {
            noise: self.octaves.as_ref(),
            applied_noises: 0,
            results: vec![0.0; cuboid.len()].into_boxed_slice(),
            cuboid,
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
    cuboid: SamplingCuboid,
}

impl<'a, TResult: DerefMut<Target = [f64]>> SampleJobImpl<'a, TResult> {
    pub fn new(noise: &'a [PerlinNoise], results: TResult, cuboid: SamplingCuboid) -> Self {
        assert_eq!(
            results.len(),
            cuboid.len(),
            "The results slice must exactly match the given dimensions"
        );

        Self {
            noise,
            applied_noises: 0,
            results,
            cuboid,
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
            let intensity = 2.0_f64.powi(self.remaining_steps() as i32 - 1);
            perlin_noise.sample_cuboid(
                self.results.borrow_mut(),
                self.cuboid.scale_all(1.0 / intensity),
                intensity,
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
    use crate::coord::SamplePos3D;
    use crate::noise::cuboid::SamplingCuboid;
    use crate::noise::fractal::{FractalNoise, SamplingJob};
    use assert_approx_eq::assert_approx_eq;
    use java_rand::Random;

    #[test]
    fn basic_data_matches() {
        let noise = FractalNoise::with_random_octaves(&mut Random::new(15), 16);
        let noises = noise
            .begin_sampling(SamplingCuboid {
                start_pos: SamplePos3D { x: 15, y: 52, z: 6 },
                x_extent: 16,
                y_extent: 4,
                z_extent: 29,
                x_scale: 0.512386,
                y_scale: 198.1293,
                z_scale: 9999.1283,
            })
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
            let mut job = noise.begin_sampling(SamplingCuboid {
                start_pos: SamplePos3D { x: 0, y: 0, z: 0 },
                x_extent: 1,
                y_extent: 1,
                z_extent: 1,
                x_scale: 1.0,
                y_scale: 1.0,
                z_scale: 1.0,
            });

            for _ in 0..self.samples {
                job.sample_once();
            }

            assert_eq!(job.remaining_variation(), self.expected);
        }
    }
}
