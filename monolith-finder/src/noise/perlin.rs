use crate::noise::cuboid::SamplingCuboid;
use java_rand::Random;

/// A single octave of Minecraft's 3-D Perlin noise
#[derive(Debug)]
pub struct PerlinNoise {
    permutations: [u8; 512],
    x_offset: f64,
    y_offset: f64,
    z_offset: f64,
}

impl PerlinNoise {
    pub const RESULT_RANGE: f64 = 1.0;

    /// Creates a noise object using the given random source. All fields will be identical to vanilla.
    pub fn with_random_permutations(random: &mut Random) -> Self {
        let x_offset = random.next_f64() * 256.0;
        let y_offset = random.next_f64() * 256.0;
        let z_offset = random.next_f64() * 256.0;
        let mut permutations = [0u8; 512];
        for i in 0..256 {
            permutations[i as usize] = i as u8;
        }
        for i in 0..256 {
            let n = random.next_i32_bound(256 - (i as i32)) as usize + i;
            permutations.swap(i, n);
        }
        for i in 256..512 {
            permutations[i] = permutations[i - 256];
        }
        PerlinNoise {
            x_offset,
            y_offset,
            z_offset,
            permutations,
        }
    }

    fn fade(x: f64) -> f64 {
        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    }

    fn lerp(sel: f64, low: f64, high: f64) -> f64 {
        low + sel * (high - low)
    }

    fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
        let hash = hash & 0xF;
        let u = if hash < 8 { x } else { y };
        let v = if hash < 4 {
            y
        } else if hash == 12 || hash == 14 {
            x
        } else {
            z
        };
        (if hash & 0x1 == 0 { u } else { -u }) + (if hash & 0x2 == 0 { v } else { -v })
    }

    /// Samples a cuboid of noise.
    ///
    /// `cuboid` describes the points to sample.
    /// Results are multiplied by `intensity` and added to `arr`. The point (`x`, `y`, `z`)
    /// within the cuboid is stored at index `y + res_y * (z + res_z * x)`.
    ///
    /// # Panics
    ///
    /// If `arr.len()` is less than `res_x * res_y * res_z`.
    pub fn sample_cuboid(&self, arr: &mut [f64], cuboid: SamplingCuboid, intensity: f64) {
        let mut output_idx = 0;
        let mut lerp0 = 0.0;
        let mut lerp1 = 0.0;
        let mut lerp2 = 0.0;
        let mut lerp3 = 0.0;
        let mut last_cube_y = Option::None;
        for x_idx in 0..cuboid.x_extent {
            for z_idx in 0..cuboid.z_extent {
                for y_idx in 0..cuboid.y_extent {
                    let x_pos_base = cuboid.scaled_x(x_idx) + self.x_offset;
                    let y_pos_base = cuboid.scaled_y(y_idx) + self.y_offset;
                    let z_pos_base = cuboid.scaled_z(z_idx) + self.z_offset;
                    let cube_x = (x_pos_base.floor() as i32 & 0xFF) as usize;
                    let cube_y = (y_pos_base.floor() as i32 & 0xFF) as usize;
                    let cube_z = (z_pos_base.floor() as i32 & 0xFF) as usize;
                    let x_pos = x_pos_base - x_pos_base.floor();
                    let y_pos = y_pos_base - y_pos_base.floor();
                    let z_pos = z_pos_base - z_pos_base.floor();
                    let u = Self::fade(x_pos);
                    let v = Self::fade(y_pos);
                    let w = Self::fade(z_pos);

                    // This caching of lerps0-3 happens in vanilla too, but is incorrect!
                    // The value is only recalculated if cube_y changes, but the calculation also depends on y_pos.
                    // This means that vertically overlapping cuboids can have different values in their overlaps.
                    if y_idx == 0 || Some(cube_y) != last_cube_y {
                        last_cube_y = Option::Some(cube_y);
                        let big_a = self.permutations[cube_x as usize] as usize + cube_y;
                        let big_aa = self.permutations[big_a] as usize + cube_z;
                        let big_ab = self.permutations[big_a + 1] as usize + cube_z;
                        let big_b = self.permutations[cube_x + 1] as usize + cube_y;
                        let big_ba = self.permutations[big_b] as usize + cube_z;
                        let big_bb = self.permutations[big_b + 1] as usize + cube_z;

                        lerp0 = Self::lerp(
                            u,
                            Self::grad(self.permutations[big_aa], x_pos, y_pos, z_pos),
                            Self::grad(self.permutations[big_ba], x_pos - 1.0, y_pos, z_pos),
                        );

                        lerp1 = Self::lerp(
                            u,
                            Self::grad(self.permutations[big_ab], x_pos, y_pos - 1.0, z_pos),
                            Self::grad(self.permutations[big_bb], x_pos - 1.0, y_pos - 1.0, z_pos),
                        );

                        lerp2 = Self::lerp(
                            u,
                            Self::grad(self.permutations[big_aa + 1], x_pos, y_pos, z_pos - 1.0),
                            Self::grad(
                                self.permutations[big_ba + 1],
                                x_pos - 1.0,
                                y_pos,
                                z_pos - 1.0,
                            ),
                        );

                        lerp3 = Self::lerp(
                            u,
                            Self::grad(
                                self.permutations[big_ab + 1],
                                x_pos,
                                y_pos - 1.0,
                                z_pos - 1.0,
                            ),
                            Self::grad(
                                self.permutations[big_bb + 1],
                                x_pos - 1.0,
                                y_pos - 1.0,
                                z_pos - 1.0,
                            ),
                        )
                    }
                    arr[output_idx] +=
                        Self::lerp(w, Self::lerp(v, lerp0, lerp1), Self::lerp(v, lerp2, lerp3))
                            * intensity;
                    output_idx += 1;
                }
            }
        }
    }
}
