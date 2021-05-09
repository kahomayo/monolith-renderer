use crate::util::DerefSliceArray;
use image::{ImageBuffer, Rgba};
use monolith_finder::coord::{BlockPos2D, SamplePos2D};
use monolith_finder::finder::search_monoliths;
use monolith_finder::worldgen::ChunkGenerator;
use wasm_bindgen::prelude::*;

mod util;

pub const TILE_SIZE: usize = 256;
pub const BYTES_PER_PIXEL: usize = 4;
pub const RESULT_LENGTH: usize = TILE_SIZE * TILE_SIZE * BYTES_PER_PIXEL;

static mut GLOBAL_GENERATOR: Option<SeededGenerator> = None;
static mut GLOBAL_BUFFER: [u8; RESULT_LENGTH] = [0; RESULT_LENGTH];

struct SeededGenerator {
    seed: u64,
    generator: ChunkGenerator,
}

fn get_pixel(is_monolith: bool) -> Rgba<u8> {
    Rgba([0, if is_monolith { 255 } else { 0 }, 128, 255])
}

fn render_section_to_buf_blip(
    chunk_generator: &ChunkGenerator,
    results: &mut [u8; RESULT_LENGTH],
    start_pos: BlockPos2D,
    stride: i32,
) {
    let mut image =
        ImageBuffer::from_raw(TILE_SIZE as u32, TILE_SIZE as u32, DerefSliceArray(results))
            .expect("Buffer size should match exactly");
    let start_pos: SamplePos2D = start_pos.into();
    for point_x in 0..TILE_SIZE {
        for point_z in 0..TILE_SIZE {
            let pos = SamplePos2D {
                x: start_pos.x + (stride * point_x as i32),
                z: start_pos.z + (stride * point_z as i32),
            };
            let is_monolith = search_monoliths(chunk_generator, pos.into(), 1, 1)[0];
            image.put_pixel(point_x as u32, point_z as u32, get_pixel(is_monolith));
        }
    }
}

// These should be UNSAFE, but wasm_bindgen doesn't let me mark them as such
fn use_seed(seed: u64) {
    util::set_panic_hook();
    unsafe {
        if GLOBAL_GENERATOR
            .as_ref()
            .filter(|g| g.seed == seed)
            .is_none()
        {
            GLOBAL_GENERATOR = Some(SeededGenerator {
                seed,
                generator: ChunkGenerator::new(seed),
            });
        }
    }
}

#[wasm_bindgen]
pub fn get_result_data() -> *const u8 {
    unsafe { GLOBAL_BUFFER.as_ptr() }
}

#[wasm_bindgen]
pub fn get_result_len() -> usize {
    RESULT_LENGTH
}

#[wasm_bindgen]
pub fn fill_tile(seed: u64, tile_x: i32, tile_y: i32, tile_z: i32) {
    assert!(-16 <= tile_z && tile_z <= -2);
    use_seed(seed);
    let scale_factor = 1 << -(tile_z + 2);
    let block_x = tile_x * 1024 * scale_factor;
    let block_z = tile_y * 1024 * scale_factor;
    unsafe {
        render_section_to_buf_blip(
            &GLOBAL_GENERATOR
                .as_ref()
                .expect("should have seeded")
                .generator,
            &mut GLOBAL_BUFFER,
            BlockPos2D {
                x: block_x,
                z: block_z,
            },
            scale_factor,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{render_section_to_buf_via_fragments, RESULT_LENGTH};
    use monolith_finder::coord::BlockPos2D;
    use monolith_finder::worldgen::ChunkGenerator;

    #[test]
    fn output_is_reasonable() {
        let gen = ChunkGenerator::new(8676641231682978167);
        let mut buf = [0; RESULT_LENGTH];
        render_section_to_buf_via_fragments(&gen, &mut buf, BlockPos2D { x: -2624, z: 4343 });
        assert_eq!(255, buf[1]);
    }
}
