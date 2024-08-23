use crate::worldgen::ScaledFractalNoise;
use java_rand::Random;

#[derive(Debug)]
pub struct ChunkGenerator {
    hill_noise: ScaledFractalNoise<10>,
    depth_noise: ScaledFractalNoise<16>,
}

impl ChunkGenerator {
    pub fn new(seed: u64) -> ChunkGenerator {
        let mut random = Random::new(seed);
        ScaledFractalNoise::<16>::discard_noise(&mut random);
        ScaledFractalNoise::<16>::discard_noise(&mut random);
        ScaledFractalNoise::<8>::discard_noise(&mut random);
        ScaledFractalNoise::<4>::discard_noise(&mut random);
        ScaledFractalNoise::<4>::discard_noise(&mut random);
        let hill_noise = ScaledFractalNoise::new(&mut random, 1.0, 0.0);
        let depth_noise = ScaledFractalNoise::new(&mut random, 100.0, 0.0);

        ChunkGenerator {
            hill_noise,
            depth_noise,
        }
    }

    pub fn hill_noise(&self) -> &ScaledFractalNoise<10> {
        &self.hill_noise
    }

    pub fn depth_noise(&self) -> &ScaledFractalNoise<16> {
        &self.depth_noise
    }
}

#[cfg(test)]
mod tests {
    use super::ChunkGenerator;
    use crate::coord::SamplePos2D;
    use crate::util::DerefSliceArrayVal;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn hill_noise_is_correct() {
        let chunk_gen = ChunkGenerator::new(8676641231682978167);
        let results = chunk_gen
            .hill_noise
            .sample2d(
                SamplePos2D { x: -656, z: 1084 },
                1,
                1,
                DerefSliceArrayVal([0.0; 1]),
            )
            .sample_all();
        assert_approx_eq!(results[0], -523.681051)
    }
}
