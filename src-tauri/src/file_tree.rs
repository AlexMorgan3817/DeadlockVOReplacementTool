use serde::{Serialize};
use std::{fs, path::Path};

#[derive(Default)]
#[derive(Serialize)]
pub struct AudioFile{
	name: String,
	duration: f32,
	size: u64,
	relative_path:String
}
#[derive(Default)]
#[derive(Serialize)]
pub struct DirectoryNode {
    name: String,
    children: Vec<DirectoryNode>,
    files: Vec<AudioFile>,
}

pub fn build_tree(path: &Path) -> std::io::Result<DirectoryNode> {
    let mut children = Vec::new();
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            children.push(build_tree(&path)?);
        } else if metadata.is_file() {
        	let ext = path.extension().and_then(|e| e.to_str());
            if ext == Some("opus")
            || ext == Some("ogg")
            || ext == Some("mp3")
            || ext == Some("wav")
            || ext == Some("flac") {
                files.push(AudioFile {
                    name: entry.file_name().to_string_lossy().into(),
                    duration: 0.0,
                    size: metadata.len(),
                    relative_path: path.to_string_lossy().into()
                });
            }
        }
    }

    children.sort_by(|a, b| a.name.cmp(&b.name));
    files.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(DirectoryNode {
        name: path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .into_owned(),
        children,
        files,
    })
}
