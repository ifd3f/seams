import commonjs from "@rollup/plugin-commonjs";
import nodeResolve from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";

export default {
  input: "js/index.ts",
  output: {
    file: "out-scripts/bundle.js",
    format: "iife",
    sourcemap: true,
  },
  plugins: [
    nodeResolve({
      browser: true,
    }),
    typescript(),
    commonjs({
      transformMixedEsModules: true,
    }),
  ],
};
