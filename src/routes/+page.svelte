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
	let link_values = $derived(Array.from(links.values()));
	let SelectedOriginals:string[] = $state<string[]>([]);
	let sources:DirKeeper = new DirKeeper("Select your original files directory");
	let replacements:DirKeeper = new DirKeeper("Select your replacements files directory");
	let save_path:string|null = null;
	let build_path:string|null = null;

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

	async function export_project(path?:string|null):Promise<boolean|string|undefined>{
		if(!is_project_savable())
		{
			alert("Nothing to save");
			return;
		}
		if(!path)
			path = await save({
				title: "Where you should save your replacements project (Replacements directory recommended)",
				defaultPath: save_path ? save_path : replacements.last_dir_loc as string | undefined,
				canCreateDirectories: true,
				filters: project_filters
			});
		if(!path)
			return;
		let dot = {
			links       : new Map(links),
			sources     : sources.serialize(),
			replacements: replacements.serialize(),
			save_path   : path,
			build_path  : build_path
		};
		save_path = path;
		if(!await invoke("save_as", {'path':path, 'projectData':dot}))
		{
			alert("Save successful.")
			return true;
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
			if ("save_path" in dot) save_path = dot.save_path as string;
			// @ts-ignore
			if("build_path" in dot) build_path = dot.build_path as string
			// @ts-ignore
			links = new SvelteMap(Object.entries(dot.links));
		}

	}
	async function build(){
		let path = await open({
			title: "Select where will your result files will be located.",
			defaultPath: build_path ? build_path : replacements.last_dir_loc as string | undefined,
			canCreateDirectories: true,
			directory: true,
			filters: project_filters
		});
		if(!path)
			return;
		build_path = path;
		try
		{
			await invoke("build", {
				buildPath: build_path,
				data:{
					links: new Map(links),
					sources: sources.folder_path(),
					replacements: replacements.folder_path()
				}
			});
		}
		catch(e){
			alert(`Error during building content: ${e}`);
			return;
		}
		alert("Builded succesful.");
	}
	async function handleKeydown(event:KeyboardEvent){
		if(event.ctrlKey && event.code == "KeyS"){
			event.preventDefault();
			try{
				await export_project(save_path)
			}
			catch(e){
				alert("UNABLE TO SAVE YOUR PROJECT: " + (e as Error).message);
			}
		}
	}

	let mouse_down:string[]|null = null;
	function toogle_links_path(target_path:string){
		if(mouse_down)
		{
			if(mouse_down.includes(target_path))
				return;
			mouse_down.push(target_path);
		}
		if(SelectedOriginals.includes(target_path))
			SelectedOriginals.splice(SelectedOriginals.indexOf(target_path), 1);
		else
			SelectedOriginals.push(target_path);
	}
</script>
<svelte:window onkeydown={handleKeydown} />

<div class='toolbar'>
	<span class="left">
		<button type="button" class="stop" onclick={()=>invoke("stop_sounds")}>■</button>
		<input id="volume" type='range' min=0 max=1 step=0.001 defaultValue=0.5
			onchange={(event)=> {invoke("set_volume", {volume:Number(event.currentTarget.value)})}}/>
		{#if SelectedOriginals.length}
		<span>{ SelectedOriginals.length } selected</span>
		<button type="button" class="stop" onclick={()=>{SelectedOriginals.splice(0, SelectedOriginals.length)}}>X</button>
		{/if}
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
			{#if links.size > 0}
				<button type='button' onclick={build}>Build</button>
			{/if}
			<button type='button' onclick={()=>{export_project()}}>Save As</button>
		{/if}
		<button type='button' onclick={load_project}>Load</button>
	</span>
</div>
<main class="container">
	<TreeNode node={sources.node} Id="originals"
		Links={links}
		SelectedCheck={(path:string)=> SelectedOriginals?.includes(path)}
		MouseDownReaction={(target_path:string, event:MouseEvent) => {
			if(event.button == 0)
			{
				toogle_links_path(target_path);
				mouse_down = [target_path];
			}
		}}
		MouseUpReaction = {(target_path:string, event:MouseEvent) => {
			mouse_down = null;
		}}
		MouseMoveReaction = {(target_path:string, event:MouseEvent) => {
			if(mouse_down && !(mouse_down.includes(target_path)))
				toogle_links_path(target_path);
		}}
		PlayReaction={(path:string, event:MouseEvent)=>{sources.playAudio(path);}}
		PlayReactionRepl={(path:string, event:MouseEvent)=>{replacements.playAudio(path);}}
		DelReplReaction={(path:string) =>{
			links.delete(path);
		}}
	/>
	<TreeNode node={replacements.node} Id="repl"
		SelectedCheck={(path:string)=>link_values?.includes(path)}
		MouseDownReaction={(target_path:string, event:MouseEvent)=>
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
