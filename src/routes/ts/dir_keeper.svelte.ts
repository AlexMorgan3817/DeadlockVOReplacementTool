import type { DirectoryNode, Serializable } from "./types";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export async function get_dir(path:string):Promise<DirectoryNode>
{
	return await invoke<DirectoryNode>("read_file_tree", { "path":path });
}

export class DirKeeper implements Serializable{
	node: DirectoryNode | null;
	constructor(
		public title:string,
		public last_dir_loc?: string | null,
		public absolute_dir?: string | null
	) {
		this.node = $state<DirectoryNode | null>(null);
	}
	async open_dialog(): Promise<string | null> {
		let res = await open({
			title: this.title,
			defaultPath: this.last_dir_loc as string | undefined,
			directory: true,
			recursive: true
		});
		if (res) {
			this.last_dir_loc = res;
		}
		return res;
	}

	async load_dialog(): Promise<boolean> {
		let path:string|null = await this.open_dialog();
		if (path)
		{
			return await this.load(path);
		}
		return false;
	}
	async load(path: string): Promise<boolean> {
		let p = path.replaceAll("\\", "/");
		this.absolute_dir = p.split("/").slice(0, -1).join("/");
		console.log(this.absolute_dir);
		console.log(path);
		let node = await get_dir(p);
		if (node) {
			this.node = node;
			return true;
		}
		return false;
	}
	async playAudio(path: string) {
		console.log(this.absolute_dir);
		console.log(path);
	    try {
	        await invoke("play_audio", {path: this.absolute_dir + "/" + path });
	    } catch (error) {
	        console.error(error, path);
	    }
	}

	public serialize(): Object {
		return {
			"last_dir_loc": this.last_dir_loc,
			"folder_path": this.absolute_dir + "/" + this.node?.name
		}
	}
	public async deserialize(o: Object): Promise<void> {
		if ("last_dir_loc" in o)
			this.last_dir_loc = o["last_dir_loc"] as string;
		if ("folder_path" in o) {
			await this.load(o["folder_path"] as string);
		}
	}
}
