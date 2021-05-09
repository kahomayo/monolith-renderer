const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  // module: {
  //   rules: [
  //     {
  //       test: /\.worker\.js$/,
  //       use: { loader: "worker-loader" }
  //     }
  //   ]
  // },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
