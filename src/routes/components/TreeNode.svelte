<script lang="ts">
	import TreeNode from './TreeNode.svelte';
	import { SvelteMap } from 'svelte/reactivity';
	import type { DirectoryNode, AudioFile, ReplLink } from "../ts/types";
	let {
		node,
		Id,
		path,
		Links,
		SelectReaction,
		SelectedOriginals,
		LinksValuesForRepl,
		PlayReaction,
		PlayReactionRepl,
		DelReplReaction
	} = $props<{
		node: DirectoryNode|null;
		Id?:string;
		path?:string;
		Links?:SvelteMap<string, string>;
		SelectReaction?: (path: string, event:MouseEvent) => void;
		SelectedOriginals?:string[],
		LinksValuesForRepl?:string[];
		PlayReaction?: (path: string, event:MouseEvent) => void;
		PlayReactionRepl?: (path: string, event:MouseEvent) => void;
		DelReplReaction?: (path: string) => void;
	}>();

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
			<div class="children">
			{#each node.children as child}
				<!-- rootNode={rootNode ? rootNode : node} -->
				<TreeNode
					node={child}
					path={(path ? path : node.name) + "/" + child.name}
					Links={Links}
					SelectedOriginals={SelectedOriginals}
					SelectReaction={SelectReaction}
					PlayReaction={PlayReaction}
					PlayReactionRepl={PlayReactionRepl}
					DelReplReaction={DelReplReaction}
					/>
			{/each}
			</div>
			<div class="content">
			{#each node.files as file}
				{@const p = (path ? path : node.name) + "/" + file.name}
				{@const repl_key = Links && Links.has(p) ? Links.get(p) : null}
				<div class = {'row' + (repl_key ? " haveLink" : "")}>
					{#if PlayReaction}
						<button type="button" class="playbutton"
							onclick={(event)=>PlayReaction?.(p, event)}
						>▶</button>
					{/if}
					<button type="button"
						class="file original"
						class:selected={SelectedOriginals?.includes(p) || LinksValuesForRepl?.includes(p)}
						onmousedown={(event) => {
							console.log(SelectReaction);
							if(SelectReaction)
								return SelectReaction(p, event);
						}}
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
