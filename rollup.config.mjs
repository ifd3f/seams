import typescript from "@rollup/plugin-typescript";

export default {
	input: 'js/index.ts',
	output: {
		file: 'out-scripts/bundle.js',
		format: 'cjs',
		sourcemap: true,
	},
	plugins: [
		typescript()
	]
};