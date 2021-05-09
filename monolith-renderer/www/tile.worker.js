import * as wasm from "monolith-renderer";
import { memory } from "monolith-renderer/monolith_renderer_bg"

const result_ptr = wasm.get_result_data();
const result_len = wasm.get_result_len();

export function onmessage(e) {
    const data = e.data;
    wasm.fill_tile(data.seed, data.tile_x, data.tile_y, data.tile_z);
    const img = new ImageData(256, 256);
    // Does this copy ?!?
    const wasm_bytes = new Uint8Array(memory.buffer);
    const result_bytes = wasm_bytes.slice(result_ptr, result_ptr + result_len);
    // Copy 1
    img.data.set(result_bytes);
    // Copy 2 -_-
    postMessage({id: data.id, img: img});
}