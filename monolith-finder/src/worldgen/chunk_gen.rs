use crate::worldgen::ScaledNoise;
use java_rand::Random;

#[derive(Debug)]
pub struct ChunkGenerator {
    hill_noise: ScaledNoise,
    depth_noise: ScaledNoise,
}

impl ChunkGenerator {
    pub fn new(seed: u64) -> ChunkGenerator {
        let mut random = Random::new(seed);
        ScaledNoise::discard_noise(&mut random, 16);
        ScaledNoise::discard_noise(&mut random, 16);
        ScaledNoise::discard_noise(&mut random, 8);
        ScaledNoise::discard_noise(&mut random, 4);
        let hill_noise = ScaledNoise::new(&mut random, 10, 1.0, 0.0);
        let depth_noise = ScaledNoise::new(&mut random, 16, 100.0, 0.0);

        ChunkGenerator {
            hill_noise,
            depth_noise,
        }
    }

    pub fn hill_noise(&self) -> &ScaledNoise {
        &self.hill_noise
    }

    pub fn depth_noise(&self) -> &ScaledNoise {
        &self.depth_noise
    }
}
