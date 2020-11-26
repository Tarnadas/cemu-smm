const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');
const webpack = require('webpack');

const dist = path.resolve(__dirname, 'dist');

module.exports = {
  mode: 'development',
  entry: './src/index.ts',
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
        use: [
          {
            loader: 'babel-loader',
            options: {
              babelrc: false,
              presets: [
                [
                  '@babel/env',
                  {
                    targets: {
                      browsers: [
                        'edge >= 17',
                        'ff >= 61',
                        'chrome >= 63',
                        'safari >= 11.1'
                      ]
                    },
                    useBuiltIns: 'usage',
                    modules: false,
                    corejs: 3
                  }
                ]
              ],
              plugins: [
                '@babel/plugin-transform-typescript',
                '@babel/plugin-syntax-dynamic-import'
              ]
            }
          }
        ]
      }
    ]
  }
};
