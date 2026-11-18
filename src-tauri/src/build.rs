use std::{collections::HashMap, format};
use std::path::{PathBuf, Path};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BuildData
{
	links: HashMap<String, String>,
	sources: String,
	replacements: String
}


#[tauri::command]
pub fn build(build_path:String, data:BuildData) -> Result<(), String>{
	// println!("{}, {}", data.sources, data.replacements);
	let patient_dir = PathBuf::from(&data.sources);
	let donor_dir = PathBuf::from(&data.replacements);
	let result_dir = PathBuf::from(&build_path);
	if !patient_dir.exists(){
		return Err(format!("Patient with path {} does not exist.", data.sources));
	}
	if !donor_dir.exists(){
		return Err(format!("Donor with path {} does not exist.", data.replacements));
	}
	if !result_dir.exists(){
		return Err(format!("Build dir with path {} does not exist.", build_path));
	}
	for (p, d) in data.links{
		let patient = Path::new(&p);
		let donor = Path::new(&d);

		let donor_file = donor_dir.join(donor);
		let result_file = result_dir.join(patient);

		if patient.extension() == donor.extension(){
			let parent_dir = &result_file.parent();
			if let Some(p) = parent_dir{
				fs::create_dir_all(&p)
					.map_err(|e| format!("Unable to create subdir {}: {}", &p.to_string_lossy(), e))?;
			}
			fs::copy(&donor_file, &result_file)
				.map_err(|e| format!(
					"Unable to copy file {} to {}: {}",
					&donor_file.to_string_lossy(),
					&result_file.to_string_lossy(),
					e)
				)?;
		}
		else{
			eprintln!("FFMPEG UNIMPLEMENTED.");
			continue;
		}
	}
	Ok(())
}
