const webpack = require('webpack');
const path = require('path');

const config = {
  context: __dirname,
  entry: {
    main: [
      './js/app.js',
    ],
  },
  // Render source-map file for final build
  devtool: 'source-map',
  // output config
  output: {
    path: path.resolve(__dirname, './public/'), // Path of output file
    filename: 'app.js', // Name of output file
  },
  plugins: [],
  resolve: {
    modules: [path.join(__dirname, 'src', 'main', 'assets', 'js'), "node_modules"],
    extensions: ['.js'],
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        loader: 'babel-loader',
      },
    ],
  },
};

module.exports = config;
