#![no_std]

use monolith_finder::coord::{BlockPos2D, SamplePos2D};
use monolith_finder::finder::{FarLandsKind, inspect_point, PointResult};
use monolith_finder::worldgen::ChunkGenerator;
use wasm_bindgen::prelude::*;

const TILE_SIZE: usize = 256;
const BYTES_PER_PIXEL: usize = 4;
const RESULT_LENGTH: usize = TILE_SIZE * TILE_SIZE * BYTES_PER_PIXEL;

type Color = [u8; 4];
const COLOR_MONOLITH: Color = [255, 0, 0, 255];
const COLOR_LAND: Color = [141, 179, 96, 255];
const COLOR_WATER: Color = [0, 0, 86, 255];
const COLOR_FAR_LANDS: Color = [127, 51, 0, 255];
const COLOR_CORNER_FAR_LANDS: Color = [255, 106, 0, 255];
const COLOR_OOB: Color=[0, 0, 0, 0];

static mut GLOBAL_GENERATOR: Option<SeededGenerator> = None;
static mut GLOBAL_BUFFER: [u8; RESULT_LENGTH] = [0; RESULT_LENGTH];

struct SeededGenerator {
    seed: u64,
    generator: ChunkGenerator,
}

fn get_pixel(point_result: PointResult) -> Color {
    match point_result {
        PointResult::Land => COLOR_LAND,
        PointResult::Water => COLOR_WATER,
        PointResult::Monolith => COLOR_MONOLITH,
        PointResult::FarLands(FarLandsKind::Corner) => COLOR_CORNER_FAR_LANDS,
        PointResult::FarLands(_) => COLOR_FAR_LANDS,
        PointResult::OOB => COLOR_OOB,
    }
}

fn render_section_to_buf_blip(
    chunk_generator: &ChunkGenerator,
    results: &mut [u8; RESULT_LENGTH],
    start_pos: BlockPos2D,
    stride: u32,
) {
    assert!(stride > 0);
    let start_pos: SamplePos2D = start_pos.into();
    for point_x in 0..TILE_SIZE {
        for point_z in 0..TILE_SIZE {
            let pos = SamplePos2D {
                x: start_pos.x + (stride as i32 * point_x as i32),
                z: start_pos.z + (stride as i32 * point_z as i32),
            };
            let point_result = inspect_point(chunk_generator, pos.into());
            let pixel = get_pixel(point_result);
            for i in 0..4 {
                results[(point_z * TILE_SIZE + point_x) * 4 + i] = pixel[i];
            }
        }
    }
}

// These should be UNSAFE, but wasm_bindgen doesn't let me mark them as such
fn use_seed(seed: u64) {
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
            scale_factor,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{render_section_to_buf_blip, RESULT_LENGTH};
    use monolith_finder::coord::{BlockPos2D, SamplePos2D};
    use monolith_finder::finder::{FarLandsKind, inspect_point, PointResult};
    use monolith_finder::worldgen::ChunkGenerator;

    #[test]
    fn output_is_reasonable() {
        let gen = ChunkGenerator::new(8676641231682978167);
        let mut buf = [0; RESULT_LENGTH];
        render_section_to_buf_blip(&gen, &mut buf, BlockPos2D { x: -2624, z: 4343 }, 1);
        assert_eq!(255, buf[3]);
    }

    #[test]
    fn oob_is_found() {
        let gen = ChunkGenerator::new(1);
        let point_result = inspect_point(&gen, BlockPos2D{x: 33_000_000, z: 33_000_000}.into());
        assert_eq!(point_result, PointResult::OOB);
    }

    #[test]
    fn far_lands_is_found() {
        let gen = ChunkGenerator::new(1);
        let point_result = inspect_point(&gen, BlockPos2D{x: 20_000_000, z: 0}.into());
        assert_eq!(point_result, PointResult::FarLands(FarLandsKind::X));
    }
}
