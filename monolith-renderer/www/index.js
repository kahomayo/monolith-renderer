import * as wasm from "monolith-renderer";
import {memory} from "monolith-renderer/monolith_renderer_bg";
import Worker from "worker-loader!./tile.worker.bootstrap.js"

wasm.use_seed("8676641231682978167");

var job_id = 0;
const jobs = []
const running_jobs = { }
const idle_workers = [Worker(), Worker(), Worker(), Worker(), Worker(), Worker()]

idle_workers.forEach(w => {
    w.onmessage = e => {
        console.log("Main got: ", e.data)
        idle_workers.push(w);
        const job = running_jobs[e.data.id];
        running_jobs[e.data.id] = undefined;
        job.tile.getContext("2d").putImageData(e.data.img, 0, 0);
        job.on_done();
        try_start_job();
    }
})

function try_start_job() {
    if (jobs.length > 0) {
        const worker = idle_workers.pop();
        if (worker) {
            const job = jobs.shift();
            console.log("Starting job ", job)
            running_jobs[job.id] = job;
            worker.postMessage({
                id: job.id,
                seed: job.seed,
                tile_x: job.coord.x,
                tile_y: job.coord.y,
                tile_z: job.coord.z,
            });
        }
    }
}

function add_job(desc) {
    jobs.push(desc);
    try_start_job();
}

var monoMap = L.map('leaflet-map', {
    maxZoom: 2,
    minZoom: -4,
    crs: L.CRS.Simple,
}).setView([-3743, -3051], 0);
var WasmLayer = L.GridLayer.extend({
    createTile: function(coord, done) {
        var error;
        console.log("Leaflet requested", coord);
        // let pos_x = coord.x * 1024;
        // let pos_z = coord.y * 1024;
        // console.log(pos_x, pos_z)
        var tile = L.DomUtil.create('canvas', 'leaflet-tile');
        var size = this.getTileSize();
        tile.width = size.x;
        tile.height = size.y;

        add_job({
            id: ++job_id,
            seed: 8676641231682978167n,
            coord: coord,
            tile: tile,
            on_done: () => done(error, tile)
        });

        // var context = tile.getContext("2d");
        // const result_buf = wasm.render_tile(pos_x, pos_z);
        // const wasmByteMemoryArray = new Uint8Array(memory.buffer);
        // const imageDataArray = wasmByteMemoryArray.slice(result_buf, result_buf + 256 * 256 * 4);
        // const canvasImageData = context.createImageData(256, 256);
        // canvasImageData.data.set(imageDataArray);
        // context.putImageData(canvasImageData, 0, 0);
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
