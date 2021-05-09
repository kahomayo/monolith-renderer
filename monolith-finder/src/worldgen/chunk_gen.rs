use crate::worldgen::ScaledNoise;
use java_rand::Random;

#[derive(Debug)]
pub struct ChunkGenerator {
    hill_noise: ScaledNoise<10>,
    depth_noise: ScaledNoise<16>,
}

impl ChunkGenerator {
    pub fn new(seed: u64) -> ChunkGenerator {
        let mut random = Random::new(seed);
        ScaledNoise::<16>::discard_noise(&mut random);
        ScaledNoise::<16>::discard_noise(&mut random);
        ScaledNoise::<8>::discard_noise(&mut random);
        ScaledNoise::<4>::discard_noise(&mut random);
        ScaledNoise::<4>::discard_noise(&mut random);
        let hill_noise = ScaledNoise::new(&mut random, 1.0, 0.0);
        let depth_noise = ScaledNoise::new(&mut random, 100.0, 0.0);

        ChunkGenerator {
            hill_noise,
            depth_noise,
        }
    }

    pub fn hill_noise(&self) -> &ScaledNoise<10> {
        &self.hill_noise
    }

    pub fn depth_noise(&self) -> &ScaledNoise<16> {
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
