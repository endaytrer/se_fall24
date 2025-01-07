const path = require('path');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
    entry: './index.ts',
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            }
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new CopyPlugin({
            patterns: [
                "index.html"
            ]
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "..")
        }),
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
   }
};

