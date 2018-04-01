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
	modules: [path.join(__dirname, 'styles'), "node_modules"],
	extensions: ['.js'],
    },
    module: {
	rules: [
	    {
		test: /\.js$/,
		exclude: /node_modules/,
		loader: 'babel-loader',
	    }, {
		test: /\.scss$/,
		use: [{
		    loader: "style-loader" // creates style nodes from JS strings
		}, {
		    loader: "css-loader" // translates CSS into CommonJS
		}, {
		    loader: "sass-loader" // compiles Sass to CSS
		}],
	    },
	]}
};

module.exports = config;
