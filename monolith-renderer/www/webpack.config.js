const CopyPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  module: {
    rules: [
      {
        test: /\.png$/,
        use: [
            'file-loader'
        ],
      },
    ]
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
        'favicon.png',
        { from: 'node_modules/leaflet/LICENSE', to: 'LICENSE-Leaflet.txt' },
        { from: 'leaflet.latlng-graticule/LICENSE', to: 'LICENSE-leaflet.latlng-graticule.txt' },
      ],
    }),
  ],
  experiments: {
    futureDefaults: true, // to get any sort of WASM support
  },
  stats: {
    errorDetails: true
  }
};
