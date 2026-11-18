use rodio::{Decoder, MixerDeviceSink, Player};
use tauri::State;
use std::sync::Mutex;
use std::fs::File;

pub struct AudioState {
	pub stream: MixerDeviceSink,
	pub player: Mutex<Player>,
}

#[tauri::command]
pub fn set_volume(volume: f32, state: State<'_, AudioState>) -> Result<(), String> {
	match state.player.lock(){
		Ok(p) => {
			p.set_volume(volume);
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
	match state.player.lock(){
		Ok(p) => {
			p.clear();
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
	let player = state.player.lock();
	match player{
		Ok(p) => {
			p.clear();
			p.append(source);
			p.play();
			Ok(())
		},
		Err(e) => Err(e.to_string())
	}
}
