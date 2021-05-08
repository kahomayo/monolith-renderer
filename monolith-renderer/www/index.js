import * as wasm from "monolith-renderer";
import {memory} from "monolith-renderer/monolith_renderer_bg";

var x = new wasm.RenderJob();
x.set_seed("8676641231682978167");

const canvas_element = document.getElementById("test-canvas");
const canvas_context = canvas_element.getContext("2d");
const canvasImageData = canvas_context.createImageData(256, 256);

const result_buf = x.render_section(-3072, 4096);
// const result_buf = x.render_section(-3051, 3743);
const wasmByteMemoryArray = new Uint8Array(memory.buffer);
const imageDataArray = wasmByteMemoryArray.slice(result_buf, result_buf + 256 * 256 * 4);
canvasImageData.data.set(imageDataArray);
canvas_context.putImageData(canvasImageData, 0, 0);


var monoMap = L.map('leaflet-map', {
    maxZoom: 2,
    minZoom: -10,
    crs: L.CRS.Simple,
}).setView([-3743, -3051], 0);
var WasmLayer = L.GridLayer.extend({
    createTile: function(coord) {
        console.log(coord);
        let pos_x = coord.x * 1024;
        let pos_z = coord.y * 1024;
        console.log(pos_x, pos_z)
        var tile = L.DomUtil.create('canvas', 'leaflet-tile');
        var size = this.getTileSize();
        tile.width = size.x;
        tile.height = size.y;
        var context = tile.getContext("2d");
        const result_buf = x.render_section(pos_x, pos_z);
        const wasmByteMemoryArray = new Uint8Array(memory.buffer);
        const imageDataArray = wasmByteMemoryArray.slice(result_buf, result_buf + 256 * 256 * 4);
        const canvasImageData = context.createImageData(256, 256);
        canvasImageData.data.set(imageDataArray);
        context.putImageData(canvasImageData, 0, 0);
        // var tile = document.createElement('label');
        // tile.innerHtml = "" + coord.lat + " | " + coord.long
        return tile;
    }
});
var thatLayer = new WasmLayer({
    minZoom: -999,
    minNativeZoom: -2,
    maxNativeZoom: -2,
});
thatLayer.addTo(monoMap);

console.log(thatLayer);