import * as wasm from "monolith-renderer";
import {memory} from "monolith-renderer/monolith_renderer_bg";
import Worker from "worker-loader!./tile.worker.bootstrap.js"

var job_id = 0;
const jobs = []
const running_jobs = { }
const idle_workers = [Worker(), Worker(), Worker(), Worker(), Worker(), Worker()]

idle_workers.forEach(w => {
    w.onmessage = e => {
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
    minZoom: -16,
    crs: L.CRS.Simple,
}).setView([-3743, -3051] /*[0, 0]*/, 0);
var WasmLayer = L.GridLayer.extend({
    createTile: function(coord, done) {
        var error;
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
        return tile;
    }
});
var thatLayer = new WasmLayer({
    minZoom: -999,
    minNativeZoom: -16,
    maxNativeZoom: -2,
    bounds: [[-30000000, -30000000], [30000000, 30000000]]
});
thatLayer.addTo(monoMap);
L.latlngGraticule({
    showLabel: true,
    dashArray: [4, 4],
    zoomInterval: [
        {start: 1, end: 2, interval: 50},
        {start: 0, end: 1, interval: 100},
        {start: -1, end: 1, interval: 250},
        {start: -2, end: 1, interval: 500},
        {start: -3, end: 1, interval: 1000},
        {start: -4, end: 1, interval: 2500},
        {start: -7, end: 1, interval: 10000},
        {start: -10, end: 1, interval: 100000},
        {start: -13, end: 1, interval: 1000000},
        {start: -16, end: 1, interval: 10000000},
    ]
}).addTo(monoMap);