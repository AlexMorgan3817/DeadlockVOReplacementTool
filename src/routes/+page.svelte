<script lang="ts">
	import "./sass/common.sass";
	import { invoke } from "@tauri-apps/api/core";
	import { DirKeeper } from "./ts/dir_keeper.svelte";
	import TreeNode from "./components/TreeNode.svelte";
	import { SvelteMap } from 'svelte/reactivity';
	import { save, open } from "@tauri-apps/plugin-dialog";
	let project_filters = [
		{'name':"SFX Replacement JSON", 'extensions':["vo.json"]},
		{'name':"Other", 'extensions':["*"]}
	];

	let links:SvelteMap<string, string> = $state(new SvelteMap());
	let SelectedOriginals:string[] = $state<string[]>([]);
	let sources:DirKeeper = new DirKeeper("Select your original files directory");
	let replacements:DirKeeper = new DirKeeper("Select your replacements files directory");
	let save_path:string|null = null;

	async function open_sources(){
		if(await sources.load_dialog())
			links.clear()
	}
	async function open_repl(){
		if(await replacements.load_dialog())
			links.clear()
	}

	function is_project_savable(){
		return sources.node && replacements.node
	}

	async function export_project(){
		if(!is_project_savable())
			return;
		console.log("Save paths: ", save_path, replacements.last_dir_loc);
		let path = await save({
			title: "Where you should save your replacements project (Replacements directory recommended)",
			defaultPath: save_path ? save_path : replacements.last_dir_loc as string | undefined,
			canCreateDirectories: true,
			filters: project_filters
		});
		if(path){
			let dot = {
				links       : new Map(links),
				sources     : sources.serialize(),
				replacements: replacements.serialize(),
				save_path   : path
			};
			save_path = path;
			console.log(dot);
			await invoke("save_as", {'path':path, 'projectData':dot});
		}
	}
	async function load_project(){
		let path = await open({
			title: "Where you should save your replacements project (Replacements directory recommended)",
			defaultPath: save_path ? save_path : replacements.last_dir_loc as string | undefined,
			canCreateDirectories: true,
			filters: project_filters
		});
		if(!path)
			return;
		// @ts-ignore
		let dot = await invoke("load", {'path':path});
		if(dot){
			// @ts-ignore
			await sources.deserialize(dot.sources);
			// @ts-ignore
			await replacements.deserialize(dot.replacements);
			// @ts-ignore
			if ("save_path" in dot)
				save_path = dot.save_path as string;
			// @ts-ignore
			links = new SvelteMap(Object.entries(dot.links));
		}

	}
	$effect(()=>{
		$inspect(sources.node);
		$inspect(SelectedOriginals);
	});
</script>
<div class='toolbar'>
	<span class="left">
		<button type="button" class="stop" onclick={()=>invoke("stop_sounds")}>■</button>
		<input id="volume" type='range' min=0 max=1 step=0.001 defaultValue=0.5
			onchange={(event)=> {invoke("set_volume", {volume:Number(event.currentTarget.value)})}}/>
	</span>
	<span class="folder_selectors">
		<button type="button" onclick={() => {
			open_sources();
		}}>sources</button>
		<button type="button" onclick={() => {
			open_repl();
		}}>replacements</button>
	</span>
	<span class='right'>
		{#if is_project_savable()}
			<button type='button' onclick={export_project}>Export</button>
		{/if}
		<button type='button' onclick={load_project}>Load</button>
	</span>
</div>
<main class="container">
	<TreeNode node={sources.node} Id="originals"
		Links={links}
		SelectedOriginals={SelectedOriginals}
		SelectReaction={(target_path:string, event:MouseEvent)=>
		{
			console.log("Select reaction", SelectedOriginals, links);
			if(!event.currentTarget)
			{
				console.log("No current target");
				return;
			}
			let button:HTMLButtonElement = event.currentTarget as HTMLButtonElement;
			if(SelectedOriginals.includes(target_path))
			{
				let idx = SelectedOriginals.indexOf(target_path);
				SelectedOriginals.splice(idx, 1);
			}
			else
				SelectedOriginals.push(target_path);
		}}
		PlayReaction={(path:string, event:MouseEvent)=>{sources.playAudio(path);}}
		PlayReactionRepl={(path:string, event:MouseEvent)=>{replacements.playAudio(path);}}
		DelReplReaction={(path:string) =>{
			links.delete(path);
		}}
	/>
	<TreeNode node={replacements.node} Id="repl"
		LinksValuesForRepl={Array.from(links.values())}
		SelectReaction={(target_path:string, event:MouseEvent)=>
		{
			if(!event.currentTarget)
			{
				return;
			}
			let button:HTMLButtonElement = event.currentTarget as HTMLButtonElement;
			for(let i of SelectedOriginals){
				let src_path = i;
				if(!src_path)
					continue;
				links.set(src_path, target_path);
			}
			SelectedOriginals.length = 0;
		}}
		PlayReaction={(path:string, event:MouseEvent)=>{replacements.playAudio(path);}}
	/>
</main>
