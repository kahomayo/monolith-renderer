import Worker from "worker-loader!./tile.worker.bootstrap.js"
import L from "leaflet"
import 'leaflet/dist/leaflet.css';
import { LatLngGraticule } from "./leaflet.latlng-graticule.js"
import "leaflet.fullscreen"
import 'leaflet.fullscreen/Control.FullScreen.css'


var job_id = 0;
const jobs = []
const running_jobs = { }
const idle_workers = [Worker(), Worker(), Worker(), Worker(), Worker(), Worker()]
const coord_x_input = document.getElementById("coord-x")
const coord_z_input = document.getElementById("coord-z")

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
    fullscreenControl: true,
}).setView([0, 0], -6);
const WasmLayer = L.GridLayer.extend({
    createTile: function(coord, done) {
        var error;
        var tile = L.DomUtil.create('canvas', 'leaflet-tile');
        var size = this.getTileSize();
        tile.width = size.x;
        tile.height = size.y;

        add_job({
            id: ++job_id,
            seed: this.options.seed,
            coord: coord,
            tile: tile,
            on_done: () => done(error, tile)
        });
        return tile;
    },
    options: {
        seed: 1n
    }
});
var currentLayer = null;
new LatLngGraticule({
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
    ],
    color: "#fff",
    weight: 1,
}).addTo(monoMap);
function chooseRandomSeed() {
    return Math.floor(Math.random() * Math.pow(2, 48));
}

function showSeed(in_seed) {
    const seed = in_seed & ((1n << 48n) - 1n);
    if (currentLayer) {
        monoMap.removeLayer(currentLayer);
    }
    currentLayer =  new WasmLayer({
        minZoom: -999,
        minNativeZoom: -16,
        maxNativeZoom: -2,
        bounds: [[-33000000, -33000000], [33000000, 33000000]],
        seed: seed,
    });
    monoMap.addLayer(currentLayer);
}

monoMap.on("moveend", function () {
    const center = monoMap.getCenter()
    coord_x_input.value = Math.round(center.lng);
    coord_z_input.value = -Math.round(center.lat);
})

const seedBox = document.getElementById("seed-input");

const seed = chooseRandomSeed();
seedBox.value = seed;
showSeed(BigInt(seed));


document.getElementById("seed-random-button").onclick = function() {
    const seed = chooseRandomSeed();
    seedBox.value = seed;
    showSeed(BigInt(seed));
}


document.getElementById("show-seed-button").onclick = function () {
    if (seedBox.value === "") {
        seedBox.value = chooseRandomSeed();
    }
    var seed;
    try {
        seed = BigInt(seedBox.value);
    } catch (e) {
        alert("Seed must be a number!");
        return;
    }
    showSeed(seed);
}



document.getElementById("coord-go-to").onclick = function () {
    monoMap.setView([-coord_z_input.value, coord_x_input.value], -2)
};