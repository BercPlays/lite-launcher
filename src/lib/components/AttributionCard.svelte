<script>
	export let title = '';
	export let description = '';
</script>

<div class="z-[1] card card-compact w-64 p-2 shadow bg-primary text-primary-content">
	<div class="card-body justify-center">
		<div>
			<p class="text-lg"><strong>{title}</strong></p>
			<p class="text-xs">{description}</p>
		</div>

		<slot />
	</div>
</div>
