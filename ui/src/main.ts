import { invoke } from "@tauri-apps/api/core"

globalThis.test = async (nb) => {
	await invoke('test', { nb });
	console.log('test');
}