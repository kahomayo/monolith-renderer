import * as wasm from "monolith-renderer";
import {memory} from "monolith-renderer/monolith_renderer_bg";

var x = new wasm.RenderJob();
x.set_seed("8676641231682978167");

const canvas_element = document.getElementById("test-canvas");
const canvas_context = canvas_element.getContext("2d");
const canvasImageData = canvas_context.createImageData(256, 256);

const result_buf = x.render_section(-2624 - 400, 4343 - 400);
const wasmByteMemoryArray = new Uint8Array(memory.buffer);
const imageDataArray = wasmByteMemoryArray.slice(result_buf, result_buf + 256 * 256 * 4);
// for (let i = 0; i < canvasImageData.data.length; i+= 4) {
//   // Percentage in the x direction, times 255
//   let x = (i % 1024) / 1024 * 255;
//   // Percentage in the y direction, times 255
//   let y = Math.ceil(i / 1024) / 256 * 255;
//
//   // Modify pixel data
//   canvasImageData.data[i + 0] = x;        // R value
//   canvasImageData.data[i + 1] = y;        // G value
//   canvasImageData.data[i + 2] = 255 - x;  // B value
//   canvasImageData.data[i + 3] = 255;      // A value
// }
// canvasImageData.data.set(imageDataArray);
for (let i = 0; i < canvasImageData.data.length; ++i) {
canvasImageData.data[i] = imageDataArray[i];
}
console.log(imageDataArray);
console.log(imageDataArray[1]);
canvas_context.putImageData(canvasImageData, 0, 0);
