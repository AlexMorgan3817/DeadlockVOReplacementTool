import type { DirectoryNode } from "./types";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export async function get_dir(path:string):Promise<DirectoryNode>
{
	return await invoke<DirectoryNode>("read_file_tree", { "path":path });
}

export class DirKeeper{
	node: DirectoryNode | null;
	constructor(
		public title:string,
		public last_dir_loc?: string | null,
		public absolute_dir?: string | null
	) {
		this.node = $state<DirectoryNode | null>(null);
	}
	async open_dialog():Promise<string|null>{
		let res = await open({
			title: this.title,
			defaultPath: this.last_dir_loc as string|undefined,
			directory: true,
			recursive: true
		});
		this.last_dir_loc = res;
		return res;
	}
	async load() {
		let path = await this.open_dialog();
		if (path)
		{
			this.absolute_dir = path.split("\\").slice(0, -1).join("/");
			this.node = await get_dir(path);
		}
	}
	async playAudio(path: string) {
	    try {
	        await invoke("play_audio", {path:this.absolute_dir + "/" + path });
	    } catch (error) {
	        console.error(error, path);
	    }
	}
}
