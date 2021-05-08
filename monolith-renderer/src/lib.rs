use image::{Rgba, RgbaImage};
use monolith_finder::coord::{BlockPos2D, SamplePos2D};
use monolith_finder::finder::search_monoliths;
use monolith_finder::worldgen::ChunkGenerator;
use wasm_bindgen::prelude::*;

pub const TILE_SIZE: u32 = 256;
pub const BYTES_PER_PIXEL: usize = 4;

#[wasm_bindgen]
pub struct RenderJob {
    chunk_generator: ChunkGenerator,
    results: Vec<u8>,
}

impl RenderJob {
    pub fn render_section_to_buf(&mut self, start_pos: BlockPos2D) {
        let mut image = RgbaImage::new(TILE_SIZE, TILE_SIZE);
        let start_pos: SamplePos2D = start_pos.into();
        for fragment_x in 0..64u32 {
            for fragment_z in 0..64u32 {
                let pos = SamplePos2D {
                    x: start_pos.x + (4 * fragment_x as i32),
                    z: start_pos.z + (4 * fragment_z as i32),
                };
                let is_monolith = search_monoliths(&self.chunk_generator, pos.into(), 4, 4);
                for px_x in 0..4u32 {
                    for px_z in 0..4u32 {
                        let is_monolith = is_monolith[(4 * px_x + px_z) as usize];
                        assert_eq!(
                            *image.get_pixel(fragment_x * 4 + px_x, fragment_z * 4 + px_z),
                            Rgba([0, 0, 0, 0])
                        );
                        image.put_pixel(
                            fragment_x * 4 + px_x,
                            fragment_z * 4 + px_z,
                            Rgba([0, if is_monolith { 255 } else { 0 }, 128, 255]),
                        );
                    }
                }
            }
        }
        self.results = image.into_raw();
    }
}

#[wasm_bindgen]
impl RenderJob {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            chunk_generator: ChunkGenerator::new(0),
            results: Vec::new(),
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.chunk_generator = ChunkGenerator::new(seed);
    }
    pub fn render_section(&mut self, block_x: i32, block_z: i32) -> *const u8 {
        self.render_section_to_buf(BlockPos2D {
            x: block_x,
            z: block_z,
        });
        self.results.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use crate::RenderJob;
    use monolith_finder::coord::BlockPos2D;

    #[test]
    fn output_is_reasonable() {
        let mut x = RenderJob::new();
        x.set_seed(8676641231682978167);
        x.render_section_to_buf(BlockPos2D { x: -2624, z: 4343 });
        assert_eq!(255, x.results[1]);
    }

    #[test]
    fn output_is_reasonable_2() {
        let mut x = RenderJob::new();
        x.set_seed(8676641231682978167);
        let res_ptr = x.render_section(-2624, 4343);
        unsafe {
            assert_eq!(255, *res_ptr.add(1));
        }
    }
}
