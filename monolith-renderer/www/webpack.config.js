const CopyPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    // Webpack is infuriating to deal with, it seems to want thousands of
    // plugins for something as simple as "include leaflet.css on this page". I
    // guess just providing a list of files to copy is at least a good escape
    // hatch.
    new CopyPlugin({
      patterns: [
        'index.html',
        { from: 'node_modules/leaflet/LICENSE', to: 'LICENSE-Leaflet.txt' },
        { from: 'leaflet.latlng-graticule/LICENSE', to: 'LICENSE-leaflet.latlng-graticule.txt' },
        { from: 'node_modules/leaflet/dist/leaflet.css', to: 'leaflet.css'},
      ],
    }),
  ],
  experiments: {
    futureDefaults: true, // to get any sort of WASM support
  },
};
