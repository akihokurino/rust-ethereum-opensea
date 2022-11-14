const nodeExternals = require("webpack-node-externals");
const path = require("path");

module.exports = {
  mode: "production",
  entry: "./src/index.ts",
  output: {
    filename: "dist/index.js",
    path: path.join(__dirname, ""),
    libraryTarget: "commonjs2",
  },
  target: "node",
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
    modules: [path.resolve("./src"), path.resolve("./node_modules")],
  },
  externals: [nodeExternals()],
};