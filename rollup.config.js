import rust from "@wasm-tool/rollup-plugin-rust";
import livereload from "rollup-plugin-livereload";
import serve from "rollup-plugin-serve";
import copy from 'rollup-plugin-copy';
import resolve from '@rollup/plugin-node-resolve';
// import { wasm  } from '@rollup/plugin-wasm';

// TODO [no_mangle]
const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        app: "./Cargo.toml", // may not be necessary
        // index: "./src/main.js",
    },
    output: {
        dir: "dist/",
        format: "es",
        sourcemap: true,
        // inlineDynamicImports: true,
    },
    plugins: [
        resolve(),

        rust({
            serverPath: "/",
            // export: "instance", // DrSensor
            target: "wasm32-unknown-unknown",
            release: false, //TODO
            debug: true,
            verbose: true,
            // nodejs: true,
            // inlineWasm: false,
            // include: ["src/*", "static/*"],
            watchPatterns: ["src/**", "static/**", "**.js", "**.toml", "**.css"],
            inlineWasm: true,
            experimental: {
                    // directExports: true,
                    // synchronous: true,
                    // asyncWebAssembly: true,
            }
        }),
        copy({
            targets: [
                // { src: 'static/assets/', dest: 'dist/' },
                // { src: 'static/index.html', dest: 'dist/' },
                { src: 'static/**', dest: 'dist/'}
                // { src: 'static/main.js', dest: 'dist/static' },
            ]
        }),
        is_watch && serve({
            contentBase: "dist",
            historyApiFallback: true,
            open: true,
        }),
        is_watch && livereload({
            watch: "dist",
            verbose: false,
            // port: 8888,
            delay: 300,
        }),

    ],


};
