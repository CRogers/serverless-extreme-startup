const path = require('path');
const fs = require('fs');
const process = require('process');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const HtmlWebpackInlineSourcePlugin = require('html-webpack-inline-source-plugin');
const webpack = require('webpack');
const history = require('connect-history-api-fallback');
const convert = require('koa-connect');

const siteDir = path.resolve(__dirname, 'site');

module.exports = (env, originalArgv) => {
    const argv = originalArgv || {};
    const mode = argv.mode || (process.env.WEBPACK_SERVE && 'development') || 'production';
    console.log('In mode', mode);

    return {
        mode,
        entry: {
            'app': './src/index.tsx',
            '404': './src/github-pages/gh-pages-spa-redirect.ts',
        },
        module: {
            rules: [
                {
                    test: /\.tsx?$/,
                    use: 'ts-loader',
                    exclude: /node_modules/
                }
            ]
        },
        resolve: {
            extensions: ['.tsx', '.ts', '.js']
        },
        output: {
            filename: '[name].js',
            path: siteDir,
        },
        plugins: [
            new HtmlWebpackPlugin({
                title: 'Extreme Startup',
                chunks: ['app'],
            }),
            new HtmlWebpackPlugin({
                title: 'Mealfu Github Pages SPA Redirect',
                chunks: ['404'],
                filename: '404.html',
                inlineSource: '.js$'
            }),
            new HtmlWebpackInlineSourcePlugin(),
            new webpack.DefinePlugin({
                WEBPACK_DEFINED_API_URL_BASE: JSON.stringify(mode === 'production'
                    ? 'https://ya9t1bqh71.execute-api.eu-west-2.amazonaws.com/prod/'
                    : 'http://localhost:9123/'),
                WEBPACK_DEFINED_BROWSER_URL_BASENAME: JSON.stringify(mode === 'production'
                    ? '/serverless-extreme-startup-frontend/'
                    : '/')
            })
        ],
        serve: {
            open: true,
            hotClient: {
                allEntries: true
            },
            add: (app, middleware, options) => {
                const historyOptions = {
                    index: '/',
                    rewrites: [
                        { from: /^\/[^.]+$/, to: '/404.html' }
                    ]
                };

                app.use(convert(history(historyOptions)));
            },
        }
    };
}