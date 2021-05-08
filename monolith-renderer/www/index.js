import * as wasm from "monolith-renderer";

var x = new wasm.RenderJob();
x.set_seed(BigInt(15));

const canvas_element = document.getElementById("test-canvas");
const canvas_context = canvas_element.getContext("2d");
const canvasImageData = canvas_context.createImageData(256, 256);