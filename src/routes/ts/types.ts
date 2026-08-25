export interface DirectoryNode {
    name: string;
    children: DirectoryNode[];
    files: AudioFile[];
}

export interface AudioFile {
    name: string;
    duration: number;
    size: number;
}

export interface ReplLink{
	src_path:string;
	target_path:string;
}
