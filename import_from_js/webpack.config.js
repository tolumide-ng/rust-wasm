const path = require("path");
const htmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
    },
    plugins: [
        new HtmlWebpackPlugin({
            crateDirectory: path.resolve(__dirname, "."),
        }),
        new webpack.ProvidePlugin({
            TextDecoder: ["text-encoding", "TextDecoder"],
            TextEncoder: ["text-encoding", "TextEncoder"],
        }),
    ],
    mode: "development",
    experiments: {
        asyncWebAssembly: true
    }
}