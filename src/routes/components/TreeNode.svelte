<script lang="ts">
	import TreeNode from './TreeNode.svelte';
	import { SvelteMap } from 'svelte/reactivity';
	import type { DirectoryNode, AudioFile, ReplLink } from "../ts/types";
	import type { MouseButtonBehaviour } from "../ts/mouse_button_behaviour"
    import { event } from '@tauri-apps/api';
	let {
		node,
		Id,
		Links,
		NameClickBehaviour,
		SelectedCheck,
		PlayReaction,
		PlayReactionRepl,
		DelReplReaction,
		path,
		root = true
	}:{
		node: DirectoryNode|null;
		Id?:string;
		Links?:SvelteMap<string, string>;
		NameClickBehaviour?: MouseButtonBehaviour<[string, MouseEvent, DirectoryNode], void>;
		SelectedCheck?: (path:string) => boolean;
		PlayReaction?: (path: string, event:MouseEvent) => void;
		PlayReactionRepl?: (path: string, event:MouseEvent) => void;
		DelReplReaction?: (path: string) => void;
		path?:string;
		root?:boolean;
	} = $props();

	let expanded = $state(true);
</script>

<div class={"directory" + (expanded ? " expanded" : "")} id={Id ? Id : ""}>
	<!-- { `${Links}` } -->
	{#if node}
		<button type="button"
			class={
				"dir_title_button " +
				((node.children.length || node.files.length) ?
					(expanded ? "expanded" : "")
					: "empty")
			}
			onclick={() => {
				if(node.children.length || node.files.length)
					expanded = !expanded;
			}}>

			{node.name}
		</button>
		{#if expanded}
			{@const abs_path = (root ? "" : ((path ? path : node.name) + "/"))}
			<div class="children">
			{#each node.children as child}
				<!-- rootNode={rootNode ? rootNode : node} -->
				<TreeNode
					node={child}
					path={abs_path + child.name}
					Links={Links}
					SelectedCheck={SelectedCheck}
					NameClickBehaviour={NameClickBehaviour}
					PlayReaction={PlayReaction}
					PlayReactionRepl={PlayReactionRepl}
					DelReplReaction={DelReplReaction}
					root = {false}
					/>
			{/each}
			</div>
			<div class="content">
			{#each node.files as file}
				{@const p = abs_path + file.name}
				{@const repl_key = Links && Links.has(p) ? Links.get(p) : null}
				<div class = {'row' + (repl_key ? " haveLink" : "")}>
					{#if PlayReaction}
						<button type="button" class="playbutton"
							onclick={(event)=>PlayReaction?.(p, event)}
						>▶</button>
					{/if}
					<button type="button"
						class="file original"
						class:selected={SelectedCheck?.(p)}
						onmousedown={(event) => NameClickBehaviour?.down?.(p, event, node)}
						onmouseup={(event) => NameClickBehaviour?.up?.(p, event, node)}
						onmousemove={(event) => NameClickBehaviour?.move?.(p, event, node)}
						oncontextmenu={(event) => NameClickBehaviour?.contextmenu?.(p,event, node)}
					>
						{file.name}
					</button>
					<span class="file_size">{Math.ceil(file.size / 1024)}kb</span>
					{#if repl_key}
						<div class='replacement'>
							<button type="button" class="removeLinkButton"
								onclick={(event)=>{DelReplReaction?.(p)}}
							>X</button>
							<button type="button" class="playbutton"
								onclick={(event)=>PlayReactionRepl?.(repl_key, event)}
							>▶</button>
							{ repl_key.split("/").at(-1) }
						</div>
					{/if}
				</div>
			{/each}
			</div>
		{/if}
	{/if}
</div>
