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
	    }, {
		test: /\.png$/,
		loader: 'url-loader?limit=100000'
	    }, {
		test: /\.woff(2)?(\?v=[0-9]\.[0-9]\.[0-9])?$/,
		loader: 'url-loader?limit=10000&mimetype=application/font-woff'
	    }, {
		test: /\.(ttf|otf|eot|svg)(\?v=[0-9]\.[0-9]\.[0-9])?|(jpg|gif)$/,
		loader: 'file-loader'
	    }
	]}
};

module.exports = config;
