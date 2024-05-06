<script>
	export let version = '';
	export let type = '';

	/** @type {Object.<String, String>} */
	let typeMappings = {
		release: 'block_grass.png',
		snapshot: 'block_workbench.png',
		old_beta: 'block_wood.png',
		old_alpha: 'block_stone.png'
	};
</script>

<div
	class="flex flex-row bg-base-100 hover:bg-base-200 p-2 items-center transition-all justify-between"
>
	<div class="start">
		<div class="flex flex-row items-center text-center">
			<img
				loading="lazy"
				src={`/blocks/${typeMappings[type]}`}
				alt="Minecraft Avatar"
				class="w-5 h-5 mr-1"
			/>
			<p class="text-xs">{type}</p>
		</div>

		<p class="text-md mt-1">{version}</p>
	</div>
	<div class="end">
		<button class="btn btn-active btn-primary justify-self-center">Select</button>
	</div>
</div>
