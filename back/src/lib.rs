use tauri::command;

pub mod loader;
pub mod simulate;
pub mod utils;

#[command]
async fn test(nb: u64) {
	let mut a: u64 = 0;
	for i in 0..nb {
		a = a >> 3 & i | 0x345 * i * 456 - i >> 3;
	}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![test])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
