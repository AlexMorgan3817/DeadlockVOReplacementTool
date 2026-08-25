mod file_tree;
use file_tree::{DirectoryNode, build_tree};
use std::{path::Path};
use std::sync::Mutex;
use std::fs::File;
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};
use tauri::State;

struct AudioState {
	stream: MixerDeviceSink,
	players: Mutex<Vec<Player>>,
	volume: Mutex<f32>
}

#[tauri::command]
fn read_file_tree(path: String) -> Result<DirectoryNode, String> {
	build_tree(&Path::new(&path))
		.map_err(|e| e.to_string())
}

#[tauri::command]
fn set_volume(volume: f32, state: State<'_, AudioState>) -> Result<(), String> {
	*state.volume.lock().map_err(|e| e.to_string())? = volume;
	match state.players.lock(){
		Ok(p) => {
			for player in p.iter(){
				player.set_volume(volume);
			}
			Ok(())
		}
		Err(e) => Err(e.to_string())
	}
}

#[tauri::command]
fn stop_sounds(state: State<'_, AudioState>) -> Result<(), String> {
	match state.players.lock(){
		Ok(p) => {
			for player in p.iter(){
				player.stop();
			}
			Ok(())
		}
		Err(e) => Err(e.to_string())
	}
}


#[tauri::command]
fn play_audio(path: String, state: State<'_, AudioState>) -> Result<(), String> {
	let file = File::open(&path)
		.map_err(|e| e.to_string())?;

	let source = Decoder::try_from(file)
		.map_err(|e| e.to_string())?;

	let player = Player::connect_new(state.stream.mixer());
	let vol = state.volume.lock().map_err(|e| e.to_string()).unwrap();
	player.set_volume(*vol);

	player.append(source);

	state.players
		.lock()
		.map_err(|e| e.to_string())?
		.push(player);

	Ok(())
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
			stop_sounds
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
