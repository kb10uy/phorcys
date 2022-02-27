const webpack = require("webpack");
const path = require("path");
// const AutoPrefixer = require("autoprefixer");
// const MiniCssExtractPlugin = require("mini-css-extract-plugin");
// const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: {
        app: [
            "./scripts/index.ts",
            // "./styles/index.scss",
        ],
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
                use: "ts-loader",
                exclude: /node_modules/,
            },
            /*
            {
                test: /\.scss$/,
                use: [
                    MiniCssExtractPlugin.loader,
                    {
                        loader: "css-loader",
                        options: {
                            sourceMap: false,
                            importLoaders: 2,
                            url: false,
                        },
                    },
                    {
                        loader: "postcss-loader",
                        options: {
                            sourceMap: false,
                            postcssOptions: {
                                plugins: [AutoPrefixer()],
                            },
                        },
                    },
                ],
                exclude: /node_modules|vendor/,
            },
            */
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
        /*
        new MiniCssExtractPlugin({
            filename: "[name].css",
        }),
        new CopyPlugin({
            patterns: [{ from: "./assets/images", to: "./images" }],
        }),
        */
    ],

    devtool: false,
};
