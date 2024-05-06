<script>
	import BaseListItem from '../BaseListItem.svelte';
</script>

<BaseListItem>
	<svelte:fragment slot="start">
		<div class="flex flex-row items-center text-center">
			<img loading="lazy" src={`/icons/logo_ms.png`} alt="Minecraft Avatar" class="w-5 h-5 mr-1" />
			<p class="text-xs">Microsoft Account</p>
		</div>
		<div class="flex flex-row items-center text-center">
			<img
				loading="lazy"
				src={`https://crafatar.com/avatars/383ce3f5589c4aa591b01a96cb5ec382?&default=MHF_Steve&overlay`}
				alt="Minecraft Avatar"
				class="w-5 h-5 mr-1"
			/>
			<p class="text-md mt-1">zsigsza</p>
		</div>
	</svelte:fragment>
	<svelte:fragment slot="end">
		<button class="btn btn-active btn-primary justify-self-center">Select</button>
	</svelte:fragment>
</BaseListItem>
