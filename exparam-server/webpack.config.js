const webpack = require("webpack");
const path = require("path");

const PostCSSPresetEnv = require("postcss-preset-env");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
// const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: {
        app: ["./assets/scripts/index.tsx", "./assets/styles/index.pcss"],
    },

    output: {
        path: path.resolve(__dirname, "public"),
        publicPath: "/",
        filename: "[name].js",
    },

    resolve: {
        extensions: [".ts", ".tsx", ".js", ".scss"],
    },

    module: {
        rules: [
            {
                test: /\.tsx?$/,
                exclude: /node_modules/,
                use: "ts-loader",
            },
            {
                test: /\.p?css$/,
                exclude: /node_modules|vendor/,
                use: [
                    MiniCssExtractPlugin.loader,
                    {
                        loader: "css-loader",
                        options: {
                            importLoaders: 1,
                            url: false,
                        },
                    },
                    {
                        loader: "postcss-loader",
                        options: {
                            postcssOptions: {
                                plugins: [PostCSSPresetEnv({ stage: 1 })],
                            },
                        },
                    },
                ],
            },
        ],
    },

    optimization: {
        splitChunks: {
            cacheGroups: {
                vendor: {
                    test: /node_modules/,
                    name: "vendor",
                    chunks: "initial",
                    enforce: true,
                },
            },
        },
    },

    plugins: [
        new MiniCssExtractPlugin({
            filename: "[name].css",
        }),
        /*
        new CopyPlugin({
            patterns: [{ from: "./assets/images", to: "./images" }],
        }),
        */
    ],

    devtool: false,
};
