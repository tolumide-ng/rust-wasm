const path = require('path');
const htmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const wasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');


module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new htmlWebpackPlugin(),
        new wasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "../hello")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or `TextDecoder` at this time
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder'],
        })
    ],
    mode: 'development'
};
