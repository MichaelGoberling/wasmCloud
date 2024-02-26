const path = require('path');

const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin");

module.exports = {
  optimization: { 
    minimize: false
  },
  entry: {
    'http-hello-world': './http-hello-world.ts',
  },
  module: {
    rules: [
      {
        test: /\.ts?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    plugins: [
        new TsconfigPathsPlugin({ configFile: "./tsconfig.json" })
    ],
    extensions: ['.tsx', '.ts', '.js']
  },
  externalsType: 'module',
  externals: {
    "wasi:http/types@0.2.0": "wasi:http/types@0.2.0"
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, 'dist'),
    library: { 
        type: 'module'
    }
  },
  experiments: { 
    outputModule: true
  }
};