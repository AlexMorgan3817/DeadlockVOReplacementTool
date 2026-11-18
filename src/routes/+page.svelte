<script lang="ts">
	import "./sass/common.sass";
	import { invoke } from "@tauri-apps/api/core";
	import { DirKeeper } from "./ts/dir_keeper.svelte";
	import TreeNode from "./components/TreeNode.svelte";
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';
	import { save, open } from "@tauri-apps/plugin-dialog";
	import type { DirectoryNode } from "./ts/types"
	let project_filters = [
		{'name':"SFX Replacement JSON", 'extensions':["vo.json"]},
		{'name':"Other", 'extensions':["*"]}
	];

	let sources:DirKeeper = new DirKeeper("Select your original files directory");
	let replacements:DirKeeper = new DirKeeper("Select your replacements files directory");

	let Links:SvelteMap<string, string> = $state(new SvelteMap());
	let link_values = $derived(Array.from(Links.values()));
	let SelectedOriginals:SvelteSet<string> = $state(new SvelteSet<string>());
	let last_selected:string|undefined = $state(undefined);
	let save_path:string|null = null;
	let build_path:string|null = null;

	async function open_sources(){
		if(await sources.load_dialog())
			Links.clear()
	}
	async function open_repl(){
		if(await replacements.load_dialog())
			Links.clear()
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
			links       : new Map(Links),
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
			Links = new SvelteMap(Object.entries(dot.links));
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
					links: new Map(Links),
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

	let lmb_selection_queue:string[]|null = null;
	function toogle_links_path(target_path:string){
		if(lmb_selection_queue)
		{
			if(lmb_selection_queue.includes(target_path))
				return;
			lmb_selection_queue.push(target_path);
		}
		if(SelectedOriginals.has(target_path))
			SelectedOriginals.delete(target_path);
		else
		{
			SelectedOriginals.add(target_path);
			last_selected = target_path;
		}
	}
	let container: HTMLElement;
	let ratio = 0.6;
	let dragging = $state(false);
	let raf = 0;
	function apply() {
		raf = 0;
		// container.style.setProperty("--ratio", String(ratio));
		container.style.setProperty("grid-template-columns", `minmax(0, ${ratio}fr) 6px minmax(0, ${1 - ratio}fr)`)
	}
</script>
<svelte:window onkeydown={handleKeydown} />

<div class='toolbar'>
	<span class="left">
		<button type="button" class="stop" onclick={()=>invoke("stop_sounds")}>■</button>
		<input id="volume" type='range' min=0 max=1 step=0.001 defaultValue=0.5
			onchange={(event)=> {invoke("set_volume", {volume:Number(event.currentTarget.value)})}}/>
		{#if SelectedOriginals.size}
		<span>{ SelectedOriginals.size } selected</span>
		<button type="button" class="stop" onclick={()=>{
			SelectedOriginals.clear();
			lmb_selection_queue = null;
		}}>X</button>
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
			{#if Links.size > 0}
				<button type='button' onclick={build}>Build</button>
			{/if}
			<button type='button' onclick={()=>{export_project()}}>Save As</button>
		{/if}
		<button type='button' onclick={load_project}>Load</button>
	</span>
</div>
<main class="container" bind:this = {container}>
	<TreeNode node={sources.node} Id="originals"
		Links={Links}

		SelectedCheck={(path:string) => SelectedOriginals?.has(path)}
		NameClickBehaviour = {{
			down:(target_path:string, event:MouseEvent) => {
				event.preventDefault();
				switch (event.button){
					case 0:
						toogle_links_path(target_path);
						lmb_selection_queue = [target_path];
				}
			},
			contextmenu:(target_path:string, event:MouseEvent, node:DirectoryNode) => {
				event.preventDefault();
				if(!last_selected)
					return;
				let last_split = last_selected.split("/");
				let target_split = target_path.split("/");
				let dir_last = last_split.slice(0, -1).join("/");
				let last_name = last_split.at(-1);
				let dir_target = target_split.slice(0, -1).join("/");
				let target_name = target_split.at(-1);
				if(dir_last != dir_target)
					return;
				let files = [];
				for(let i of node.files)
					files.push(i.name);
				let a = files.indexOf(last_name!);
				let b = files.indexOf(target_name!);
				if (a > b)
				{
					let tmp = a;
					a = b;
					b = tmp;
				}
				let res = []
				for(let i = a; i <= b; i++)
				{	let r = dir_last + "/" + files[i]
					res.push(r);
					SelectedOriginals.add(r)
				}
				console.log(res);
				last_selected = res.at(-1);
			},
			up:(target_path:string, event:MouseEvent) => {
				lmb_selection_queue = null;
				event.preventDefault();
			},
			move:(target_path:string, event:MouseEvent) => {
				if(lmb_selection_queue && !(lmb_selection_queue.includes(target_path)))
					toogle_links_path(target_path);
			}
		}}
		PlayReaction={(path:string, event:MouseEvent)=>{sources.playAudio(path);}}
		PlayReactionRepl={(path:string, event:MouseEvent)=>{replacements.playAudio(path);}}
		DelReplReaction={(path:string) =>{
			Links.delete(path);
		}}
	/>
	<div class="splitter" role="separator" aria-orientation="vertical"
			onpointerdown={(e: PointerEvent) =>{
				e.preventDefault();
				(e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
				dragging = true;
			}}
			onpointermove={(e: PointerEvent) => {
				if (!dragging)
					return;
				const r = container.getBoundingClientRect();
				ratio = Math.min(0.85, Math.max(0.15, (e.clientX - r.left) / r.width));
				raf = requestAnimationFrame(apply);
			}}
			onpointerup={() => dragging = false}
			onpointercancel={() => dragging = false}
			ondblclick={() => ratio = 0.6}></div>
	<TreeNode node={replacements.node} Id="repl"
		SelectedCheck={(path:string)=>link_values?.includes(path)}
		NameClickBehaviour = {{
			down:(target_path:string, event:MouseEvent)=>
			{
				if(!event.currentTarget)
					return;
				for(let i of SelectedOriginals){
					let src_path = i;
					if(!src_path)
						continue;
					Links.set(src_path, target_path);
				}
				SelectedOriginals.clear();
				lmb_selection_queue = null;
			}
		}}
		PlayReaction={(path:string, event:MouseEvent)=>{replacements.playAudio(path);}}
	/>
</main>
