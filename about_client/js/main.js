import init, { run_app } from './pkg/about_client.js';
async function main() {
	await init('./pkg/about_client_bg.wasm');
	run();
}
main()
