use std::io::ErrorKind::{AlreadyExists};
use std::{io::Write, path::PathBuf};
use std::{eprintln, format, fs};
use std::fs::{File};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SerializedDirKeeper{
	last_dir_loc:String,
	folder_path:String
}
#[derive(Serialize, Deserialize)]
pub struct Project{
	links:HashMap<String, String>,
	sources:SerializedDirKeeper,
	replacements:SerializedDirKeeper,
	save_path:Option<String>
}

#[tauri::command]
pub fn save_as(path:String, project_data:Project) -> Result<(), String>{
	// fs::create_dir_all(path);
	let path = PathBuf::from(path);
	let backup:Option<PathBuf>;
	let mut file = match File::create_new(&path){
		Ok(f)=>{
			backup = None;
			f
		}
		Err(e) => {
			match e.kind(){
				AlreadyExists=>{
					let mut tmp = (&path).clone();
					if let Some(fname) = tmp.file_name(){
						tmp.set_file_name(fname.to_string_lossy().to_string() + ".back");
						if let Err(e) = fs::copy(&path, &tmp){
							return Err(format!("Can't backup: {}", e))
						}
						backup = Some(tmp);
						match File::create(path){
							Ok(f) => f,
							Err(e) => return Err(format!("Can't overwrite file: {}", e))
						}
					}
					else{return Err(format!("Can't backup your file, since it has no name."))}
				}
				_ => {return Err(format!("Can't create file: {}.", e))}
			}
		}
	};
	let s = serde_json::to_string(&project_data)
		.map_err(|e| format!("Unable to serialize project: {}", e))?;
	file.write_all(s.as_bytes()).map_err(|e| format!("Unable to write data: {}", e))?;
	if let Some(backup) = backup{
		if let Err(e) = fs::remove_file(backup){
			eprintln!("Unable to remove backup file: {}", e);
		}
	}
	Ok(())
}
#[tauri::command]
pub fn load(path:String) -> Result<Project, String>
{
	let data = fs::read_to_string(&path)
		.map_err(|e| format!("Unable to read file {}: {}", path, e))?;
	let obj = serde_json::from_str::<Project>(&data)
		.map_err(|e| format!("Unable to serialize {}: {}", path, e))?;
	Ok(obj)
}
