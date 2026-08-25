<script lang="ts">
	import "./sass/common.sass";
	import { invoke } from "@tauri-apps/api/core";
	import { DirKeeper } from "./ts/dir_keeper.svelte";
	import TreeNode from "./components/TreeNode.svelte";

	let links:Map<string, string> = $state(new Map());
	let SelectedOriginals:string[] = $state<string[]>([]);
	let sources:DirKeeper = new DirKeeper("Select your original files directory");
	let replacements_library:DirKeeper = new DirKeeper("Select your replacements files directory");
	$effect(()=>{
		$inspect(sources.node);
		$inspect(SelectedOriginals);
	});
</script>
<div class='toolbar'>
	<!-- <span>{sources.node ? sources.node.name : ""}</span> -->
	<span class="left">
		<button type="button" class="stop" onclick={()=>invoke("stop_sounds")}>■</button>
		<input id="volume" type='range' min=0 max=1 step=0.001 defaultValue=0.5
			onchange={(event)=> {invoke("set_volume", {volume:Number(event.currentTarget.value)})}}/>
	</span>
	<span class="buttons">
		<button type="button" onclick={() => {
			sources.load();
			links = new Map();
		}}>sources</button>
		<button type="button" onclick={() => {
			replacements_library.load();
			links = new Map();
		}}>replacements</button>
	</span>
	<!-- <span>{replacements_library.node ? replacements_library.node.name : ""}</span> -->
</div>
<main class="container">
	<TreeNode node={sources.node} Id="originals"
		Links={links}
		SelectedOriginals={SelectedOriginals}
		SelectReaction={(event:MouseEvent)=>
		{
			console.log(event);
			if(!event.currentTarget)
				return;
			let button:HTMLButtonElement = event.currentTarget as HTMLButtonElement;
			let repl_path = button.getAttribute("path");
			if(repl_path)
			{
				if(SelectedOriginals.includes(repl_path))
				{
					let idx = SelectedOriginals.indexOf(repl_path);
					SelectedOriginals.splice(idx, 1);
				}
				else
					SelectedOriginals.push(repl_path);
			}
		}}
		PlayReaction={(path:string, event:MouseEvent)=>{sources.playAudio(path);}}
		PlayReactionRepl={(path:string, event:MouseEvent)=>{replacements_library.playAudio(path);}}
		DelReplReaction={(path:string) =>{
			links.delete(path);
			links = new Map(links);
		}}
	/>
	<TreeNode node={replacements_library.node} Id="repl"
		LinksValuesForRepl={Array.from(links.values())}
		SelectReaction={(event:MouseEvent)=>
		{
			if(!event.currentTarget)
			{
				return;
			}
			let button:HTMLButtonElement = event.currentTarget as HTMLButtonElement;
			let repl_path = button.getAttribute("path");
			if(!repl_path)
			{
				return;
			}
			for(let i of SelectedOriginals){
				let src_path = i;
				if(!src_path)
					continue;
				links.set(src_path, repl_path);
			}
			links = new Map(links);
			SelectedOriginals.length = 0;
		}}
		PlayReaction={(path:string, event:MouseEvent)=>{replacements_library.playAudio(path);}}
	/>
</main>
