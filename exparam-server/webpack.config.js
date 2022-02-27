const webpack = require("webpack");
const path = require("path");

const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: {
        app: ["./assets/scripts/index.tsx"],
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
        new CopyPlugin({
            patterns: [{ from: "./assets/images", to: "./images" }],
        }),
        */
    ],

    devtool: false,
};
