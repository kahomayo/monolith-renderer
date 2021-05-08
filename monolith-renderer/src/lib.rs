use image::{Rgba, RgbaImage};
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
        let mut image = RgbaImage::new(TILE_SIZE, TILE_SIZE);
        for fragment_x in 0..64u32 {
            for fragment_z in 0..64u32 {
                let is_monolith = search_monoliths(
                    &self.chunk_generator,
                    block_x + (4 * fragment_x as i32),
                    block_z + (4 * fragment_z as i32),
                    4,
                    4,
                );
                for px_x in 0..4u32 {
                    for px_z in 0..4u32 {
                        let is_monolith = is_monolith[(px_z + 4 * px_x) as usize];
                        image.put_pixel(
                            fragment_x * 4 + px_x,
                            fragment_z * 4 + px_z,
                            Rgba([0, if is_monolith { 255 } else { 0 }, 128, 0]),
                        );
                    }
                }
            }
        }
        self.results = image.into_raw();
        self.results.as_ptr()
    }
}
