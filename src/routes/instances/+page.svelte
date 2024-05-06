<script>
	import InstancesList from '$lib/subpages/instances/InstancesList.svelte';
</script>

<div class="flex flex-row justify-end px-4 pt-4">
	<div class="tooltip tooltip-left" data-tip="Add instance">
		<button class="btn btn-ghost px-3"><span class="material-symbols-rounded">add</span></button>
	</div>
</div>

<div role="tablist" class="tabs tabs-bordered px-4 pb-4 text-base-content">
	<input type="radio" name="my_tabs_1" role="tab" checked class="tab" aria-label="Instances" />
	<div role="tabpanel" class="tab-content border-base-content/10 border-t-2 border-x-0 border-y-0">
		<div>
			<InstancesList></InstancesList>
		</div>
	</div>

	<input type="radio" name="my_tabs_1" role="tab" class="tab" aria-label="Shared Instances" />
	<div role="tabpanel" class="tab-content border-base-content/10 border-t-2 border-x-0 border-y-0">
		<div>
			<p>Shared</p>
		</div>
	</div>
</div>
