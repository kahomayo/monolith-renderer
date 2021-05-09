use crate::util::DerefSliceArray;
use image::{ImageBuffer, Rgb, Rgba};
use monolith_finder::coord::{BlockPos2D, SamplePos2D};
use monolith_finder::finder::{inspect_point, PointResult};
use monolith_finder::worldgen::ChunkGenerator;
use std::num::NonZeroU32;
use wasm_bindgen::prelude::*;

mod util;

const TILE_SIZE: usize = 256;
const BYTES_PER_PIXEL: usize = 4;
const RESULT_LENGTH: usize = TILE_SIZE * TILE_SIZE * BYTES_PER_PIXEL;

const COLOR_MONOLITH: Rgba<u8> = Rgba([255, 0, 0, 255]);
const COLOR_LAND: Rgba<u8> = Rgba([141, 179, 96, 255]);
const COLOR_WATER: Rgba<u8> = Rgba([0, 0, 86, 255]);
const COLOR_CANDIDATE: Rgba<u8> = Rgba([0, 0, 64, 255]);

static mut GLOBAL_GENERATOR: Option<SeededGenerator> = None;
static mut GLOBAL_BUFFER: [u8; RESULT_LENGTH] = [0; RESULT_LENGTH];

struct SeededGenerator {
    seed: u64,
    generator: ChunkGenerator,
}

fn get_pixel(point_result: PointResult) -> Rgba<u8> {
    match point_result {
        PointResult {
            is_candidate: true,
            is_land: true,
        } => COLOR_MONOLITH,
        PointResult {
            is_candidate: true,
            is_land: false,
        } => COLOR_CANDIDATE,
        PointResult { is_land: true, .. } => COLOR_LAND,
        PointResult { is_land: false, .. } => COLOR_WATER,
    }
}

fn render_section_to_buf_blip(
    chunk_generator: &ChunkGenerator,
    results: &mut [u8; RESULT_LENGTH],
    start_pos: BlockPos2D,
    stride: NonZeroU32,
) {
    let mut image =
        ImageBuffer::from_raw(TILE_SIZE as u32, TILE_SIZE as u32, DerefSliceArray(results))
            .expect("Buffer size should match exactly");
    let start_pos: SamplePos2D = start_pos.into();
    for point_x in 0..TILE_SIZE {
        for point_z in 0..TILE_SIZE {
            let pos = SamplePos2D {
                x: start_pos.x + (stride.get() as i32 * point_x as i32),
                z: start_pos.z + (stride.get() as i32 * point_z as i32),
            };
            let point_result = inspect_point(chunk_generator, pos.into());
            image.put_pixel(point_x as u32, point_z as u32, get_pixel(point_result));
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
    let block_x = tile_x * 1024 * scale_factor as i32;
    let block_z = tile_y * 1024 * scale_factor as i32;
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
            NonZeroU32::new(scale_factor).expect("Scale factor was miscalculated"),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{render_section_to_buf_blip, RESULT_LENGTH};
    use monolith_finder::coord::BlockPos2D;
    use monolith_finder::worldgen::ChunkGenerator;
    use std::num::NonZeroU32;

    #[test]
    fn output_is_reasonable() {
        let gen = ChunkGenerator::new(8676641231682978167);
        let mut buf = [0; RESULT_LENGTH];
        render_section_to_buf_blip(
            &gen,
            &mut buf,
            BlockPos2D { x: -2624, z: 4343 },
            NonZeroU32::new(1).unwrap(),
        );
        assert_eq!(255, buf[1]);
    }
}
