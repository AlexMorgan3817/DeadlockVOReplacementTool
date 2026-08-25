use rodio::{Decoder, MixerDeviceSink, Player};
use tauri::State;
use std::sync::Mutex;
use std::fs::File;

pub struct AudioState {
	pub stream: MixerDeviceSink,
	pub players: Mutex<Vec<Player>>,
	pub volume: Mutex<f32>
}

#[tauri::command]
pub fn set_volume(volume: f32, state: State<'_, AudioState>) -> Result<(), String> {
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
pub fn stop_sounds(state: State<'_, AudioState>) -> Result<(), String> {
	_stop_sounds(&state)
}
pub fn _stop_sounds(state: &State<'_, AudioState>) -> Result<(), String> {
	match state.players.lock(){
		Ok(mut p) => {
			for player in p.iter(){
				player.stop();
			}
			p.retain(|_player| true);
			Ok(())
		},
		Err(e) => Err(e.to_string())
	}
}

#[tauri::command]
pub fn play_audio(path: String, state: State<'_, AudioState>) -> Result<(), String> {
	let file = File::open(&path)
		.map_err(|e| e.to_string())?;

	let source = Decoder::try_from(file)
		.map_err(|e| e.to_string())?;
	let _ =_stop_sounds(&state);
	let player = Player::connect_new(state.stream.mixer());
	let vol = state.volume.lock()
		.map_err(|e| e.to_string())?;
	player.set_volume(*vol);

	player.append(source);
	state.players
		.lock()
		.map_err(|e| e.to_string())?
		.push(player);

	Ok(())
}
