mod file_tree;
use file_tree::{DirectoryNode, build_tree};

mod persistency;
use persistency::{save_as, load};

mod player;
use player::{AudioState, play_audio, set_volume, stop_sounds};

mod build;
use build::build;

use std::{path::Path};
use std::sync::Mutex;
use rodio::{DeviceSinkBuilder};


#[tauri::command]
fn read_file_tree(path: String) -> Result<DirectoryNode, String> {
	build_tree(&Path::new(&path))
		.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let stream = DeviceSinkBuilder::open_default_sink()
		.expect("Failed to open audio device");

	let audio_state = AudioState {
		stream,
		players: Mutex::new(Vec::new()),
		volume: Mutex::new(0.5)
	};
	tauri::Builder::default()
		.manage(audio_state)
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_dialog::init())
		.invoke_handler(tauri::generate_handler![
			read_file_tree,
			play_audio,
			set_volume,
			stop_sounds,
			save_as,
			load,
			build
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
