<script lang="ts">
	import TreeNode from './TreeNode.svelte';
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
		Links?:Map<string, string>;
		SelectReaction?:Function,
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
					/>
			{/each}
			</div>
			<div class="content">
			{#each node.files as file}
				{@const  p = (path ? path : node.name) + "/" + file.name}
				<div class ='row'>
					{#if PlayReaction}
						<button type="button" class="playbutton"
							onclick={(event)=>PlayReaction?.(p, event)}
						>▶</button>
					{/if}
					<button type="button"
						class="file original"
						class:selected={SelectedOriginals?.includes(p) || LinksValuesForRepl?.includes(p)}
						path={p}
						onmousedown={(event) => {
							console.log(SelectReaction);
							if(SelectReaction)
								return SelectReaction(event);
						}}
					>
						{file.name}
					</button>
					{#if Links}
						{#if Links.has(p)}
							{@const repl_key = Links.get(p)}
							<div class='replacement'>
								<button type="button" class="playbutton"
									onclick={(event)=>PlayReactionRepl?.(repl_key, event)}
								>▶</button>
								{ repl_key.split("/").at(-1) }
								<button type="button" class="removeLinkButton"
									onclick={(event)=>{DelReplReaction(p)}}
								>X</button>
							</div>
						{/if}
					{/if}
				</div>
			{/each}
			</div>
		{/if}
	{/if}
</div>
