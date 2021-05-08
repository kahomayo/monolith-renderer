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
canvasImageData.data.set(imageDataArray);
canvas_context.putImageData(canvasImageData, 0, 0);
