<script>
	import InstanceGridItem from '$lib/components/DataComponents/InstanceGridItem.svelte';
</script>

<div class="fix-svh grid gap-3 overflow-y-auto p-4">
	{#each { length: 1 } as item}
		<InstanceGridItem></InstanceGridItem>
	{/each}
</div>

<style>
	.grid {
		grid-template-columns: repeat(auto-fit, minmax(12em, 0.25fr));
		grid-auto-rows: max-content;
	}
	/* same goes for here */
	.fix-svh {
		height: calc(100svh - 11.1em);
	}
</style>
