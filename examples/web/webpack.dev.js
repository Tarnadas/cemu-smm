/* eslint-disable */

const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');
const webpack = require('webpack');

const dist = path.resolve(__dirname, 'dist');

module.exports = {
  mode: 'development',
  entry: './src/index.tsx',
  output: {
    path: dist,
    filename: 'bundle.js'
  },
  devtool: 'inline-source-map',
  devServer: {
    contentBase: dist
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'src/index.html'
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'development'
    })
  ],
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.jsx', '.json', '.wasm']
  },
  experiments: {
    syncWebAssembly: true
  },
  watchOptions: {
    aggregateTimeout: 200,
    ignored: ['../../target/**']
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: ['babel-loader']
      },
      {
        test: /\.(jpe?g|png|gif|svg)$/i, 
        use: 'file-loader'
      }
    ]
  }
};
